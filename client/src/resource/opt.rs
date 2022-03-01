use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "server")]
pub struct Opt {
    /// Whether server is running on same machine or not
    #[structopt(short, long)]
    pub is_remote: bool,

    /// Where to write bind address
    #[structopt(short, long, default_value = "127.0.0.1:12351")]
    pub server_addr: String,

    /// How many frames of delay input has
    /// Lower value = less input lag / more other player lag
    /// Higher value = more input / less other player lag
    #[structopt(long, default_value = "0")]
    pub input_lag: u32,

    /// How many frames of delay for remote players
    /// This will reduce warping of remote players
    #[structopt(long, default_value = "5")]
    pub remote_player_delay: usize,
}
