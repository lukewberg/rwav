use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Wav file to operate on
    #[arg(long, short)]
    pub input: String,

    /// Output header information
    #[arg(long, default_value="false")]
    pub header: bool,
}
