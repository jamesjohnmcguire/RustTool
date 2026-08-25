fn main()
{
    println!("Hello, world!");
    if let Err(e) = inboxes::process() {
        eprintln!("Processing failed: {}", e);
    }
}
