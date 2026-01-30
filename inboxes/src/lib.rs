use rand;
use std::env;

use fs_extra::dir::{copy, CopyOptions};
use std::path::Path;
use std::io::Result;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn inboxes() -> Result<()>
{
    let src = "source_dir";
    let dst = "destination_dir";

    let mut key = "HOME";

    if (env::consts::OS == "windows")
    {
        key = "USERPROFILE";
    }
    else
    {
        println!("This OS is not supported yet.");
    }

    let user_profile = match env::var(key)
    {
        Ok(val) => val,
        Err(e) => {
            println!("Couldn't read {key}: {e}");
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to get user profile"));
        }
    };

    // Default options: won't overwrite existing files
    let options = CopyOptions::new();

    copy(src, dst, &options);
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
