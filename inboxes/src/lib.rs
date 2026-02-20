use anyhow::Context;
use anyhow::Result;
use fs_extra::dir::CopyOptions;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn process_entry(entry: fs::DirEntry, destination: &Path) -> Result<()>
{
    let path = entry.path();
    println!("Checking: {}", path.display());

    // Skip junctions, symlinks, and anything that isn't a real file/dir
    let metadata = fs::symlink_metadata(&path)?;

    if metadata.file_type().is_symlink()
    {
        eprintln!("  Warning: skipping symlink/junction: {}", path.display());
        return Ok(());
    }

    // Default options: won't overwrite existing files
    let move_options = CopyOptions::new();

    if path.is_dir()
    {
        fs_extra::dir::move_dir(&path, &destination, &move_options)
            .context(format!("Failed to move dir {:?}", path))?;
    }
    else
    {
        fs_extra::file::move_file(
            &path, destination.join(path.file_name().unwrap()), 
            &fs_extra::file::CopyOptions::new())
            .map_err(
                |e| anyhow::anyhow!("Failed to move file {:?}: {}", path, e))?;
    }
    Ok(())
}

pub fn process() -> Result<()>
{
    println!("Processing inboxes...");

    let sources = vec!
    [
        "/Documents",
        "/Downloads",
        "/OneDrive/Documents",
        "/OneDrive/Pictures",
    ];

    let key = if cfg!(windows) {
        "USERPROFILE"
    } else if cfg!(unix) {
        "HOME"
    } else {
        anyhow::bail!("This OS is not supported yet");
    };

    let user_profile = PathBuf::from(env::var(key)?);

    let destination = user_profile.join("Data").join("Inbox");
    fs::create_dir_all(&destination)?;

    for base_source in &sources {
        let source = user_profile.join(base_source.trim_start_matches('/'));
        println!("Processing: {}", source.display());

        if !source.exists() {
            println!("  Skipping: not found");
            continue;
        }

        let paths = fs::read_dir(&source).unwrap();

        for entry in paths {
            let entry = entry?;
            process_entry(entry, &destination)?;
        }
    }

    Ok(())
}

pub fn inbox(directory: &str) -> bool
{
    println!("Processing inbox in directory: {}", directory);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
