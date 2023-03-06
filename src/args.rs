use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="OlshaMB, Cat", version, about, long_about = None)]
pub struct Args {
    /// Url to open
    pub url: String,
}