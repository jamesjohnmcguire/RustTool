use rand;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
