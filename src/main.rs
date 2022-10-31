use anyhow::{Context, Result};
use chrono::naive::NaiveDateTime;

use twitch_recover::*;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Recover the vod from twitchtracker (alias: tt)
    #[command(aliases = ["tt"])]
    _TwitchTracker {
        #[clap(flatten)]
        twitchtracker: TwitchTracker,

        #[command(flatten)]
        options: Opt,
    },
    /// Manually recover the vod
    _Manual {
        #[clap(flatten)]
        manual: Manual,

        #[command(flatten)]
        options: Opt,
    },
}

#[derive(Debug, Args)]
pub struct Opt {
    /// How many concurrent requests to recover the vod
    #[arg(short, long)]
    chunck: Option<usize>,
}

#[derive(Debug, Args)]
#[clap(arg_required_else_help(true))]
pub struct TwitchTracker {
    /// Url of twitchtracker
    pub url: String,
}

#[derive(Debug, Args)]
#[clap(arg_required_else_help(true))]
pub struct Manual {
    /// Streamer name
    #[arg(short, long)]
    pub streamer: String,
    /// Id from twitchtracker url
    #[arg(short, long)]
    pub vod_id: String,
    /// Start of the stream (Y-m-d H:M)
    #[arg(short, long)]
    pub date: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.commands {
        Commands::_TwitchTracker {
            twitchtracker,
            options,
        } => {
            let options = match options.chunck {
                Some(chunck) => VodRecoverOptions::new(chunck),
                None => VodRecoverOptions {
                    ..Default::default()
                },
            };

            let vod = VodRecover::from_twitchtracker(&twitchtracker.url)
                .await
                .context("Unable to parse twitchtracker")?;

            let url = vod
                .get_url(&options)
                .await
                .context("Unable to find a valid url")?;

            println!("\n{}\n", url);
        }
        Commands::_Manual { manual, options } => {
            let options = match options.chunck {
                Some(chunck) => VodRecoverOptions::new(chunck),
                None => VodRecoverOptions {
                    ..Default::default()
                },
            };

            let timestamp = NaiveDateTime::parse_from_str(&manual.date, "%Y-%m-%d %H:%M")
                .context(format!(
                    "Unable to parse the date for '{}' (format: Y-m-d H:M)",
                    &manual.date
                ))?
                .timestamp();

            let vod = VodRecover::from_manual(&manual.streamer, &manual.vod_id, timestamp);

            let url = vod
                .get_url(&options)
                .await
                .context("Unable to find a valid url")?;

            println!("\n{}\n", url);
        }
    }

    Ok(())
}
