use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "server")]
pub struct Opt {
    /// Where to write bind address
    #[structopt(short, long, default_value = "0.0.0.0:12351")]
    pub addr: String,
}
