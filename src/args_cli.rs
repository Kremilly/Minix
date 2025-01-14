use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Flags {
    #[arg(short, long)]
    /// Set the file to minify
    pub input: String,

    #[arg(short, long)]
    /// Set the file output after minified
    pub output: Option<String>,

    #[arg(short, long)]
    /// Watch mode for minify
    pub watch: bool,
}
