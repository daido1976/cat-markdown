use cat_markdown::{cat, clipboard};
use clap::Parser;
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input files
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Copy to clipboard
    #[arg(long, short)]
    clipboard: bool,
    // 開発終わったら stdout をオプションにして clipboard の方をデフォルトにしてもいいかも
    ///// Output to stdout
    // #[arg(long)]
    // stdout: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = cat::cat_files(cli.files).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    if cli.clipboard {
        clipboard::copy(&text).unwrap();
    } else {
        println!("{}", text);
    }
}
