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
    pub chunck: Option<usize>,
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
