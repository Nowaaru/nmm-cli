use clap::{Parser, Subcommand};
use std::{
    env,
    path::{Path, PathBuf},
};

mod lockfile;
mod r#mod;
mod nexus;
mod nix;
mod provider;
mod query;
mod tests;

// https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes
// https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Provider {
    Nexus {
        #[arg(value_name = "GAME-DOMAIN-NAME")]
        domain: String,
        #[arg(value_name = "MOD-ID")]
        mod_id: i32,
        #[arg(value_name = "FILE-ID")]
        file_id: i32,

        #[arg(short, long)]
        expire: Option<usize>,
        #[arg(short, long)]
        key: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Fetch {
        #[command(subcommand)]
        provider: Provider,
    },

    Limits {
        provider: Option<String>,
    },

    Checkout {
        provider: Option<String>,
    },

    #[command(arg_required_else_help = true)]
    Init {
        r#where: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut lock = lockfile::Lockfile::from_pwd();

    match args.command {
        // make lockfile in current directory
        Commands::Init { r#where } => match r#where {
            Some(here) => {
                println!("bruh: {:#?}", std::env::current_dir().unwrap());
                lockfile::Lockfile::new()
                    .write(Path::new(&here))
                    .map(|_| {
                        println!("wrote lockfile to path {}", &here);
                    })
                    .unwrap()
            }

            None => {
                println!("huh?: {:#?}", std::env::current_dir().unwrap());
                if let None = lock {
                    match std::env::current_dir().ok() {
                        Some(dir) => lockfile::Lockfile::new().write(&dir).map(|_| ()).unwrap(),
                        None => (),
                    }
                } else {
                    println!("lockfile already exists. exiting...");
                }
            }
        },

        Commands::Fetch { provider } => match provider {
            Provider::Nexus {
                domain,
                mod_id,
                file_id,
                expire,
                key,
            } => {
                todo!();
                if let Some(lockfile) = lock {
                }
            }
        },

        // allows the user to check their api power
        // in case they're on the verge of ratelimiting
        Commands::Limits { provider } => match provider {
            Some(provider) => (),
            None => (),
        },

        // iterates through the lockfile and
        // generates download links (if necessary),
        // lockfile members should probably have `isApi`
        // to indicate whether nix-mod-manager should use
        // nmm-cli or just run straight-up fetchurl. :thinking: /shrug
        Commands::Checkout { provider } => match provider {
            Some(string) => (),
            None => (),
        },
    };
}
