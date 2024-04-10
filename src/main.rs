use std::io::{self, Read};

use clap::{Args, Parser, Subcommand};

use clip::{version::version, ClipboardContext, ClipboardProvider};

// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "winyank")]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(short_flag = 'i')]
    Input(InputArgs),
    #[command(short_flag = 'o')]
    Output(OutputArgs),
    #[command(short_flag = 'v')]
    Version,
}

#[derive(Args, Debug)]
struct InputArgs {
    #[arg(long)]
    lf: Option<bool>,
}

#[derive(Args, Debug)]
struct OutputArgs {
    #[arg(long)]
    crlf: Option<bool>,
}

fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    match args.command {
        Commands::Input(input) => {
            let mut stdin = io::stdin();
            let mut content = String::new();
            stdin.read_to_string(&mut content).unwrap();
            let mut provider = ClipboardContext::new().unwrap();
            if input.lf.unwrap_or(false) {
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
            if output.crlf.unwrap_or(false) {
                content = content.replace("\r\n", "\n");
            }
            print!("{}", content);
        }
        Commands::Version => {
            println!("{}", version());
        }
    }
}
