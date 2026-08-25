use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn make_temp_profile() -> PathBuf {
    let base = env::temp_dir();
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let dir = base.join(format!("rusttool_test_{}", t));
    let _ = fs::create_dir_all(&dir);
    dir
}

#[test]
fn skip_on_destination_exists() {
    let profile = make_temp_profile();
    // We'll call the library function with an explicit `profile` instead of setting env vars.

    // Create source Documents with a file
    let source_docs = profile.join("Documents");
    fs::create_dir_all(&source_docs).unwrap();
    let src_file = source_docs.join("hello.txt");
    fs::write(&src_file, "hello").unwrap();

    // Create destination with an existing file of the same name to trigger AlreadyExists
    let dest = profile.join("Data").join("Inbox");
    fs::create_dir_all(&dest).unwrap();
    let dest_file = dest.join("hello.txt");
    fs::write(&dest_file, "existing").unwrap();

    // Run processing; should not panic and should skip the existing destination
    assert!(inboxes::process_with_profile(true, &profile).is_ok());

    // Source should still exist (skipped)
    assert!(src_file.exists());

    // Destination should be unchanged
    let dest_contents = fs::read_to_string(&dest_file).unwrap();
    assert_eq!(dest_contents, "existing");
}

#[cfg(unix)]
#[test]
fn permission_denied_skipped_unix() {
    use std::os::unix::fs::PermissionsExt;

    let profile = make_temp_profile();

    let source_docs = profile.join("Documents");
    fs::create_dir_all(&source_docs).unwrap();
    let protected = source_docs.join("protected_dir");
    fs::create_dir_all(&protected).unwrap();

    // Remove all permissions to cause PermissionDenied on Unix
    fs::set_permissions(&protected, fs::Permissions::from_mode(0)).unwrap();

    // Should not panic and should return Ok (skipping the protected dir)
    assert!(inboxes::process_with_profile(true, &profile).is_ok());

    // Restore perms for cleanup
    fs::set_permissions(&protected, fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(windows)]
#[test]
fn permission_denied_skipped_windows() {
    use std::process::Command;

    let profile = make_temp_profile();

    let source_docs = profile.join("Documents");
    fs::create_dir_all(&source_docs).unwrap();
    let protected = source_docs.join("protected_dir");
    fs::create_dir_all(&protected).unwrap();

    let username = std::env::var("USERNAME").unwrap_or_else(|_| String::from(""));

    // Try to deny full control for the current user on the protected dir. This may require elevated rights.
    let deny_status = Command::new("icacls")
        .arg(&protected)
        .arg("/deny")
        .arg(format!("{}:F", username))
        .status();

    // If we couldn't run icacls or it failed (likely due to lack of privileges), skip this test.
    let deny_ok = match deny_status {
        Ok(s) if s.success() => true,
        Ok(s) => {
            eprintln!("icacls returned non-success: {} - skipping Windows permission test", s);
            false
        }
        Err(e) => {
            eprintln!("failed to run icacls ({}), skipping Windows permission test", e);
            false
        }
    };

    if !deny_ok {
        return;
    }

    // Running process() should not panic and should skip the protected dir
    let res = inboxes::process_with_profile(true, &profile);

    // Try to remove the deny ACE we added. Use /remove:d to remove deny entries; if that fails, try /grant.
    let _ = Command::new("icacls")
        .arg(&protected)
        .arg("/remove:d")
        .arg(&username)
        .status();

    let _ = Command::new("icacls")
        .arg(&protected)
        .arg("/grant")
        .arg(format!("{}:F", username))
        .status();

    assert!(res.is_ok());
}
