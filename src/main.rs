use clap::{Parser, Subcommand};
use nexus::NexusEndpoints;
use provider::ModProvider;
use std::path::{Path, PathBuf};
use std::string::String;

use crate::lockfile::{LockProvider, ModLock};

mod lockfile;
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
    // TODO: instead of using lock per match arm
    // use the match inside of the lock map lol
    // (please...)

    let args = Cli::parse();
    let mut lock = lockfile::Lockfile::from(None);

    match args.command {
        // make lockfile in current directory
        Commands::Init { r#where } => {
            // FIXME: look for better handling (because redundancy...)

            let mut cwd = PathBuf::from(r#where.map_or_else(
                || {
                    std::env::current_dir()
                        .expect("could not read cwd")
                        .to_string_lossy()
                        .to_string()
                },
                |w| w,
            ));

            if cwd.is_file() {
                cwd = cwd
                    .join("/..")
                    .canonicalize()
                    .expect("unable to canonicalize path");
            }

            lockfile::Lockfile::new()
                .write(Some(&cwd))
                .map(|_| {
                    println!("wrote lockfile to path {:?}/nmm.lock", &cwd);
                })
                .unwrap()
        }

        Commands::Fetch { provider } => match provider {
            Provider::Nexus {
                domain,
                mod_id,
                file_id,
            } => lock
                .map(|mut lockfile| {
                    if lockfile
                        .get_mod_id("nexus", &mod_id.to_string(), &file_id.to_string())
                        .is_some()
                    {
                        println!("Provider 'nexus' already has mod {mod_id:?}:{file_id:?}");
                        return Ok(());
                    }

                    let nexus_provider = nexus::NexusProvider::new(None);
                    nexus_provider.download(domain, mod_id, file_id, &mut lockfile)
                })
                .expect("Test failed while looking for lockfile..."),
        }
        .expect("An unknown error occurred."),

        // allows the user to check their api power
        // in case they're on the verge of ratelimiting
        Commands::Limits { provider } => match provider {
            Some(provider) => lock.map_or((), |lockfile| {
                println!(
                    "{:?}",
                    lockfile
                        .get_provider(&provider)
                        .expect(&format!(
                            "could not find limits for provider {}",
                            provider.clone()
                        ))
                        .limits
                )
            }),

            None => lock.map_or((), |lockfile| {
                for (_, LockProvider { name, limits, .. }) in lockfile.providers {
                    println!("Limits for {name:?}:\n${limits:#?}")
                }
            }),
        },

        // iterates through the lockfile and
        // generates download links (if necessary),
        // lockfile members should probably have `isApi`
        // to indicate whether nix-mod-manager should use
        // nmm-cli or just run straight-up fetchurl. :thinking: /shrug
        Commands::Checkout { provider } => {
            if let Some(mut lockfile) = lock {
                match provider {
                    Some(provider) => match provider.as_str() {
                        "nexus" => {
                            // i believe this amount of shadowing
                            // is a terrible idea. Too bad!

                            if let Some(provider) = lockfile.get_provider(provider) {
                                for (mod_id, file_ids) in &provider.mods.clone() {
                                    let nexus_provider = nexus::NexusProvider::new(None);
                                    for (file_id, ModLock { game_id, .. }) in file_ids {
                                        println!("Downloading {game_id:?}/{mod_id:?}:{file_id:?}");
                                        let parse = str::parse::<i32>;
                                        let download_result = nexus_provider.download(
                                            game_id.clone(),
                                            parse(&mod_id).unwrap(),
                                            parse(&file_id).unwrap(),
                                            &mut lockfile,
                                        );

                                        if let Ok(..) = download_result {
                                            println!("Download completed.");
                                        } else {
                                            panic!("Download.. failed?")
                                        }
                                    }
                                }
                            }
                        }
                        _ => todo!(),
                    },
                    None => todo!(),
                };
            }
        }
    };
}
