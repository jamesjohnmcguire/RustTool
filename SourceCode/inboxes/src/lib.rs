use anyhow::Context;
use anyhow::Result;
use fs_extra::dir::CopyOptions;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

fn process_entry(entry: fs::DirEntry, destination: &Path, ignore_errors: bool) -> Result<()>
{
    let path = entry.path();
    println!("Checking: {}", path.display());

    // Skip junctions, symlinks, and anything that isn't a real file/dir
    let metadata = match fs::symlink_metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                eprintln!("  Warning: permission denied accessing {}: {}", path.display(), e);
                if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Failed to read metadata {:?}: {}", path, e)); }
            } else {
                if ignore_errors { eprintln!("  Warning: failed to read metadata {:?}: {}", path, e); return Ok(()); }
                return Err(anyhow::anyhow!("Failed to read metadata {:?}: {}", path, e));
            }
        }
    };

    if metadata.file_type().is_symlink()
    {
        eprintln!("  Warning: skipping symlink/junction: {}", path.display());
        return Ok(());
    }

    // Default options: won't overwrite existing files
    let move_options = CopyOptions::new();

    if path.is_dir()
    {
        // Prefer atomic rename when possible so we can inspect std::io::ErrorKind.
        let file_name = match path.file_name() {
            Some(n) => n,
            None => {
                eprintln!("  Warning: skipping path without file name: {}", path.display());
                return Ok(());
            }
        };
        let target = destination.join(file_name);

        if target.exists() {
            eprintln!("  Warning: destination exists for {:?}, skipping", path);
            if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Destination exists: {:?}", path)); }
        }

        match fs::rename(&path, &target) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!("  Warning: destination exists for {:?}, skipping", path);
                    if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Destination exists: {:?}", path)); }
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("  Warning: permission denied moving {:?}: {}", path, e);
                    if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Permission denied moving: {:?}", path)); }
                }
                _ => {
                    // Fall back to fs_extra which can handle cross-volume moves.
                    match fs_extra::dir::move_dir(&path, &destination, &move_options) {
                        Ok(_) => {}
                        Err(e2) => {
                            eprintln!("  Warning: failed to move dir {:?}: {}", path, e2);
                            if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Failed to move dir {:?}: {}", path, e2)); }
                        }
                    }
                }
            },
        }
    }
    else
    {
        let file_name = match path.file_name() {
            Some(n) => n,
            None => {
                eprintln!("  Warning: skipping file without name: {}", path.display());
                return Ok(());
            }
        };
        let target = destination.join(file_name);
        if target.exists() {
            eprintln!("  Warning: destination exists for {:?}, skipping", path);
            if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Destination exists: {:?}", path)); }
        }

        match fs::rename(&path, &target) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!("  Warning: destination exists for {:?}, skipping", path);
                    if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Destination exists: {:?}", path)); }
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("  Warning: permission denied moving {:?}: {}", path, e);
                    if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Permission denied moving: {:?}", path)); }
                }
                _ => {
                    match fs_extra::file::move_file(&path, &target, &fs_extra::file::CopyOptions::new()) {
                        Ok(_) => {}
                        Err(e2) => {
                            eprintln!("  Warning: failed to move file {:?}: {}", path, e2);
                            if ignore_errors { return Ok(()); } else { return Err(anyhow::anyhow!("Failed to move file {:?}: {}", path, e2)); }
                        }
                    }
                }
            },
        }
    }
    Ok(())
}

pub fn process_with_profile(ignore_errors: bool, user_profile: &Path) -> Result<()> {
    println!("Processing inboxes...");

    let sources = vec!["/Documents", "/Downloads", "/OneDrive/Documents", "/OneDrive/Pictures"];

    let destination = user_profile.join("Data").join("Inbox");
    fs::create_dir_all(&destination)?;

    for base_source in &sources {
        let source = user_profile.join(base_source.trim_start_matches('/'));
        println!("Processing: {}", source.display());

        if !source.exists() {
            println!("  Skipping: not found");
            continue;
        }

        let paths = match fs::read_dir(&source) {
            Ok(iter) => iter,
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    eprintln!("  Warning: permission denied reading {}: {}", source.display(), e);
                    continue;
                } else {
                    return Err(e).context(format!("Failed to read dir {:?}", source));
                }
            }
        };

        for entry in paths {
            let entry = match entry {
                Ok(ent) => ent,
                Err(e) => {
                    eprintln!("  Warning: failed to read entry in {}: {}", source.display(), e);
                    continue;
                }
            };

            let entry_path = entry.path();

            match process_entry(entry, &destination, ignore_errors) {
                Ok(_) => {}
                Err(e) => {
                    if ignore_errors {
                        eprintln!("  Warning: failed to process {}: {}", entry_path.display(), e);
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn process(ignore_errors: bool) -> Result<()> {
    let key = if cfg!(windows) { "USERPROFILE" } else if cfg!(unix) { "HOME" } else { anyhow::bail!("This OS is not supported yet"); };
    let user_profile = PathBuf::from(env::var(key)?);
    process_with_profile(ignore_errors, &user_profile)
}

pub fn inbox(directory: &str) -> bool
{
    println!("Processing inbox in directory: {}", directory);

    true
}

fn is_skip_error(e: &std::io::Error) -> bool {
    matches!(e.kind(), ErrorKind::PermissionDenied | ErrorKind::AlreadyExists)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_again()
    {
        let check = inbox("test_inbox");
        assert_eq!(check, true);
    }

    #[test]
    fn skip_errors_recognized() {
        let perm = io::Error::from(io::ErrorKind::PermissionDenied);
        let exists = io::Error::from(io::ErrorKind::AlreadyExists);
        let other = io::Error::from(io::ErrorKind::NotFound);

        assert!(is_skip_error(&perm));
        assert!(is_skip_error(&exists));
        assert!(!is_skip_error(&other));
    }

    #[test]
    fn integration_skip_on_destination_exists() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let base = std::env::temp_dir();
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let profile = base.join(format!("rusttool_integration_{}", t));

        // Call the processing function with the explicit test profile instead of setting env vars.

        let source_docs = profile.join("Documents");
        fs::create_dir_all(&source_docs).unwrap();
        let src_file = source_docs.join("hello.txt");
        fs::write(&src_file, "hello").unwrap();

        let dest = profile.join("Data").join("Inbox");
        fs::create_dir_all(&dest).unwrap();
        let dest_file = dest.join("hello.txt");
        fs::write(&dest_file, "existing").unwrap();

        assert!(process_with_profile(true, &profile).is_ok());
        assert!(src_file.exists());
        let dest_contents = fs::read_to_string(&dest_file).unwrap();
        assert_eq!(dest_contents, "existing");
    }

    #[cfg(unix)]
    #[test]
    fn integration_permission_denied_unix() {
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::os::unix::fs::PermissionsExt;

        let base = std::env::temp_dir();
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let profile = base.join(format!("rusttool_integration_{}", t));
        // Call processing against the explicit test profile rather than setting HOME.

        let source_docs = profile.join("Documents");
        fs::create_dir_all(&source_docs).unwrap();
        let protected = source_docs.join("protected_dir");
        fs::create_dir_all(&protected).unwrap();

        fs::set_permissions(&protected, fs::Permissions::from_mode(0)).unwrap();

        assert!(process_with_profile(true, &profile).is_ok());

        fs::set_permissions(&protected, fs::Permissions::from_mode(0o755)).unwrap();
    }
}
