use std::env;
use std::path::Path;
use tracky_core::scan_media;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: tracky-cli <media-folder>");
        std::process::exit(1);
    }

    let root = Path::new(&args[1]);
    let result = scan_media(root);

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
