use structopt::{clap::AppSettings, StructOpt};

use std::{net::SocketAddr, path::PathBuf};

/// Minimal mpd terminal client that aims to be simple yet highly configurable
///
/// Homepage: https://github.com/figsoda/mmtc
#[derive(StructOpt)]
#[structopt(
    name = "mmtc",
    rename_all = "kebab-case",
    global_setting = AppSettings::ColoredHelp,
)]
pub struct Opts {
    /// Clear query on play
    #[structopt(long)]
    pub clear_query_on_play: bool,

    /// Cycle through the queue
    #[structopt(long)]
    pub cycle: bool,

    /// Don't clear query on play
    #[structopt(long, overrides_with("clear_query_on_play"))]
    pub no_clear_query_on_play: bool,

    /// Don't cycle through the queue
    #[structopt(long, overrides_with("cycle"))]
    pub no_cycle: bool,

    /// Specify the address of the mpd server
    #[structopt(long, value_name = "address")]
    pub address: Option<SocketAddr>,

    /// Specify the config file
    #[structopt(short, long, value_name = "file")]
    pub config: Option<PathBuf>,

    /// The number of lines to jump
    #[structopt(long, value_name = "number")]
    pub jump_lines: Option<usize>,

    /// The time to seek in seconds
    #[structopt(long, value_name = "number")]
    pub seek_secs: Option<f32>,

    /// The amount of status updates per second
    #[structopt(long, value_name = "number")]
    pub ups: Option<f32>,
}

#[derive(Debug)]
pub enum Command {
    Quit,
    UpdateFrame,
    UpdateStatus,
    UpdateQueue,
    ToggleRepeat,
    ToggleRandom,
    ToggleSingle,
    ToggleConsume,
    ToggleOneshot,
    TogglePause,
    Stop,
    SeekBackwards,
    SeekForwards,
    Previous,
    Next,
    Play,
    Reselect,
    Down,
    Up,
    JumpDown,
    JumpUp,
    InputSearch(char),
    BackspaceSearch,
    QuitSearch,
    Searching(bool),
}