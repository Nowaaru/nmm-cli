use clap::{Parser, Subcommand};
use nexus::NexusEndpoints;
use provider::ModProvider;
use std::{
    env,
    path::{Path, PathBuf},
};

use crate::lockfile::{LockProvider, ModLock};

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

fn main() {
    let args = Cli::parse();
    let lock = lockfile::Lockfile::from_cwd();

    match args.command {
        // make lockfile in current directory
        Commands::Init { r#where } => match r#where {
            // FIXME: look for better handling (because redundancy...)
            Some(here) => {
                println!("bruh: {:#?}", std::env::current_dir().unwrap());
                if let None = lock {
                    lockfile::Lockfile::new()
                        .write(Some(Path::new(&here)))
                        .map(|_| {
                            println!("wrote lockfile to path {}", &here);
                        })
                        .unwrap()
                } else {
                    println!("lockfile already exists. exiting...");
                }
            }

            None => {
                if let None = lock {
                    lockfile::Lockfile::new()
                        .write(std::env::current_dir().ok().as_deref())
                        .map(|_| ())
                        .unwrap();
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
            } => lock
                .map(|mut lockfile| {
                    let nexus_provider = nexus::NexusProvider::new(None);
                    nexus_provider
                        .make_download_links(domain, mod_id, file_id)
                        .map(|links| {
                            let uri = links.get_server_or("Los Angeles", |links| {
                                links.first().unwrap().URI.to_owned()
                            });

                            nix::prefetch_url(uri.clone(), None)
                                .map(|prefetch| {
                                    let nix::Prefetched { store_path, sha } = prefetch;
                                    let added_mod = lockfile.add_mod(
                                        "nexus",
                                        ModLock::new(mod_id, file_id, sha.clone(), store_path.clone()),
                                    );

                                    if let Ok(()) = added_mod {
                                        println!(
                                            "Test complete! Output link: {:#?}",
                                            nix::Prefetched {
                                                sha: sha.clone(),
                                                store_path: store_path.clone(),
                                            }
                                        );

                                        lockfile.write(None).map_err(|err| {
                                            println!("wtf: {}", err);
                                            ()
                                        })
                                    } else {
                                        panic!("Test failed while adding to lockfile...");
                                    }
                                })
                                .expect("Test failed while prefetching...")
                        })
                        .expect("Failed to fetch download links.")
                })
                .expect("Test failed while looking for lockfile..."),
        }
        .expect("An unknown error occurred."),

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
