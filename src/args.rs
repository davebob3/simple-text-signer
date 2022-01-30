#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// signature
    #[clap(short, long)]
    pub key: String,
}

/// Subcommand: sign with command line text, sign the contents of the file,
/// or verify text on the command line
#[derive(clap::Subcommand, Debug)]
#[clap(author, version, about, long_about = None)]
pub enum Command {
    /// Sign the command line text
    Text { text: String },
    /// Sign the contents of a file
    File { file: String },
    /// Verify the output of the signing commands.
    Verify { text: String },
    /// Verify the contents of a file.
    VerifyFile { file: String },
}

impl Command {
    pub fn is_sign(&self) -> bool {
        match self {
            Command::Text { .. } | Command::File { .. } => true,
            Command::Verify { .. } | Command::VerifyFile { .. } => false,
        }
    }
    pub fn is_verify(&self) -> bool {
        !self.is_sign()
    }

    pub fn get_string(&self) -> Result<String, Box<dyn std::error::Error + 'static>> {
        match self {
            Command::Text { text } | Command::Verify { text } => Ok(text.trim_end().to_string()),
            Command::File { file } | Command::VerifyFile { file } => {
                std::fs::read_to_string(file).map_err(Box::from)
            }
        }
    }

    pub fn get_data(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        match self {
            Command::Text { text } | Command::Verify { text } => {
                Ok(text.trim_end().to_string().into_bytes())
            }
            Command::File { file } | Command::VerifyFile { file } => {
                std::fs::read(file).map_err(Box::from)
            }
        }
    }
}
