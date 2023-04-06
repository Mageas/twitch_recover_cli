use anyhow::{Context, Result};
use chrono::naive::NaiveDateTime;

use clap::Parser;

use twitch_recover::*;

mod vod_downloader;
use vod_downloader::*;

mod args;
use args::*;

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
