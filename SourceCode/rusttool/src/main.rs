fn main()
{
    println!("Rust Tool");
    let args: Vec<String> = std::env::args().collect();

    let strict = args.iter().any(|a| a == "--strict" || a == "-s");

    let ignore_errors = !strict;

    match inboxes::process(ignore_errors)
    {
        Ok(_) => {}
        Err(e) =>
        {
            eprintln!("Processing failed: {}", e);
            if strict
            {
                std::process::exit(1);
            }
        }
    }
}
