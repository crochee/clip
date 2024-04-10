use std::io::{self, Read};

use clap::*;

use clipx::{
    clipboard::{self, ClipboardProvider},
    version::version,
};

#[cfg(windows)]
mod cli {
    use super::*;

    // A fictional versioning CLI
    #[derive(Debug, Parser)]
    #[command(name = "winyank")]
    #[command(author, about, long_about = None)]
    pub struct Cli {
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
        crlf: Option<bool>,
    }
    #[derive(Args, Debug)]
    struct OutputArgs {
        #[arg(long)]
        lf: Option<bool>,
    }
    pub fn handle_clipboard(args: Cli) {
        match args.command {
            Commands::Input(input) => {
                let mut stdin = io::stdin();
                let mut content = String::new();
                stdin.read_to_string(&mut content).unwrap();
                let mut provider =
                    clipboard::windows_clipborad::WindowsClipboardContext::new().unwrap();

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
                let mut provider =
                    clipboard::windows_clipborad::WindowsClipboardContext::new().unwrap();
                let mut content = provider.get_contents().unwrap();
                if output.lf.unwrap_or(false) {
                    content = content.replace("\r\n", "\n");
                }
                print!("{}", content);
            }
            Commands::Version => println!("{}", version()),
        }
    }
}

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
mod cli {
    use super::*;

    // A fictional versioning CLI
    #[derive(Debug, Parser)]
    #[command(name = "winyank")]
    #[command(author, about, long_about = None)]
    pub struct Cli {
        #[arg(short, value_enum, default_missing_value = "clipboard")]
        selection: Option<Selection>,
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Debug, ValueEnum, Clone)]
    enum Selection {
        Primary,
        Clipboard,
    }
    #[derive(Debug, Subcommand)]
    enum Commands {
        #[command(short_flag = 'i')]
        Input,
        #[command(short_flag = 'o')]
        Output,
        #[command(short_flag = 'v')]
        Version,
    }

    pub fn handle_clipboard(args: Cli) {
        match args.command {
            Commands::Input => {
                let mut stdin = io::stdin();
                let mut content = String::new();
                stdin.read_to_string(&mut content).unwrap();
                match args.selection.unwrap_or(Selection::Clipboard) {
                    Selection::Primary => {
                        let mut provider = clipboard::x11_clipboard::X11ClipboardContext::<
                            clipboard::x11_clipboard::Primary,
                        >::new()
                        .unwrap();
                        provider.set_contents(content).unwrap();
                    }
                    Selection::Clipboard => {
                        let mut provider = clipboard::x11_clipboard::X11ClipboardContext::<
                            clipboard::x11_clipboard::Clipboard,
                        >::new()
                        .unwrap();
                        provider.set_contents(content).unwrap();
                    }
                }
            }
            Commands::Output => {
                let content = match args.selection.unwrap_or(Selection::Clipboard) {
                    Selection::Primary => {
                        let mut provider = clipboard::x11_clipboard::X11ClipboardContext::<
                            clipboard::x11_clipboard::Primary,
                        >::new()
                        .unwrap();
                        provider.get_contents().unwrap()
                    }
                    Selection::Clipboard => {
                        let mut provider = clipboard::x11_clipboard::X11ClipboardContext::<
                            clipboard::x11_clipboard::Clipboard,
                        >::new()
                        .unwrap();
                        provider.get_contents().unwrap()
                    }
                };
                print!("{}", content);
            }
            Commands::Version => println!("{}", version()),
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    let args = cli::Cli::parse();
    cli::handle_clipboard(args)
}
