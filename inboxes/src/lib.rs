use anyhow::Result;
use fs_extra::dir::{move_dir, CopyOptions};
use std::env;
use std::path::Path;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn inboxes() -> Result<()>
{
    let src = "source_dir";
    let dst = "destination_dir";

    let key = if cfg!(windows) {
        "USERPROFILE"
    } else if cfg!(unix) {
        "HOME"
    } else {
        anyhow::bail!("This OS is not supported yet");
    };

    let user_profile = env::var(key)?;

    // Default options: won't overwrite existing files
    let options = CopyOptions::new();
    let options = options.content_only(true); 

    move_dir(src, dst, &options)?;
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
