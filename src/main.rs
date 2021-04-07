fn parse_markdown_file(filename: &str) {
    print_banner();
    println!("[ INFO ] Trying to parse {}...", filename)
}

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn usage() -> String {
    String::from("Usage: tinymd <somefile>.md")
}

fn get_title() -> String {
    format!(
        "{} ({}), {}",
        env!("CARGO_PKG_NAME"),
        get_version(),
        env!("CARGO_PKG_DESCRIPTION")
    )
}

fn print_banner() {
    println!("{}", get_title());
    println!("{}", usage());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            println!("{}", usage());
        }
    }
}
