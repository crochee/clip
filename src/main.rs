use std::io::{self, Read};

use clap::{Parser, Subcommand};

#[cfg(windows)]
use clap::Args;

use clip::{version::version, ClipboardContext, ClipboardProvider};

// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "winyank")]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(windows)]
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(short_flag = 'i')]
    Input(InputArgs),
    #[command(short_flag = 'o')]
    Output(OutputArgs),
    #[command(short_flag = 'v')]
    Version,
}

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(short_flag = 'i')]
    Input,
    #[command(short_flag = 'o')]
    Output,
    #[command(short_flag = 'v')]
    Version,
}

#[cfg(windows)]
#[derive(Args, Debug)]
struct InputArgs {
    #[arg(long)]
    crlf: Option<bool>,
}

#[cfg(windows)]
#[derive(Args, Debug)]
struct OutputArgs {
    #[arg(long)]
    lf: Option<bool>,
}
fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    #[cfg(windows)]
    match args.command {
        Commands::Input(input) => {
            let mut stdin = io::stdin();
            let mut content = String::new();
            stdin.read_to_string(&mut content).unwrap();
            let mut provider = ClipboardContext::new().unwrap();

            if input.crlf.unwrap_or(false) {
                let chunks = content.split("\r\n").map(|item| item.replace('\n', "\r\n"));
                let mut first = true;
                let mut out = String::with_capacity(content.len());
                for chunk in chunks {
                    if first {
                        first = false;
                    } else {
                        out.push_str("\r\n");
                    }
                    out.push_str(&chunk);
                }
                provider.set_contents(out).unwrap();
            } else {
                provider.set_contents(content).unwrap();
            }
        }
        Commands::Output(output) => {
            let mut provider = clip::ClipboardContext::new().unwrap();
            let mut content = provider.get_contents().unwrap();
            if output.lf.unwrap_or(false) {
                content = content.replace("\r\n", "\n");
            }
            print!("{}", content);
        }
        Commands::Version => {
            println!("{}", version());
        }
    }
    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
    ))]
    match args.command {
        Commands::Input => {
            let mut stdin = io::stdin();
            let mut content = String::new();
            stdin.read_to_string(&mut content).unwrap();
            let mut provider = ClipboardContext::new().unwrap();
            provider.set_contents(content).unwrap();
        }
        Commands::Output => {
            let mut provider = clip::ClipboardContext::new().unwrap();
            let content = provider.get_contents().unwrap();
            print!("{}", content);
        }
        Commands::Version => {
            println!("{}", version());
        }
    }
}
