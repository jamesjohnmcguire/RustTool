use anyhow::Result;
use fs_extra::dir::{move_dir, CopyOptions};
use std::env;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn process() -> Result<()>
{
    println!("Processing inboxes...");

    let sources = vec!
    [
        "/Documents",
        "/Downloads",
        "/OneDrive/Desktop",
        "/OneDrive/Documents",
        "/OneDrive/Downloads",
        "/OneDrive/Pictures",
    ];

    let key = if cfg!(windows) {
        "USERPROFILE"
    } else if cfg!(unix) {
        "HOME"
    } else {
        anyhow::bail!("This OS is not supported yet");
    };

    let user_profile = env::var(key)?;

    let destination = format!("{}/Data/Inbox", user_profile);

    // Default options: won't overwrite existing files
    let options = CopyOptions::new();
    let options = options.content_only(true); 

    for base_source in &sources {
        let source = format!("{}{}", user_profile, base_source);

        move_dir(source, &destination, &options)?;
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
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_again()
    {
        let check = inbox("test_inbox");
        assert_eq!(check, true);
    }
}
