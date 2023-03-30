use std::path::PathBuf;

use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
struct Cli {

    /// 可选输入，名称
    name: Option<String>,

}

fn main() {
    println!("hello clap");
}