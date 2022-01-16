use clap::Parser;

use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::pkey::PKey;
use openssl::sign::Signer;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,

    /// signature
    #[clap(short, long)]
    key: String,
}

/// Subcommand: sign with command line text, sign the contents of the file,
/// or verify text on the command line
#[derive(clap::Subcommand, Debug)]
#[clap(author, version, about, long_about = None)]
enum Command {
    /// Sign the command line text
    Text { text: String },
    /// Sign the contents of a file
    File { file: String },
    /// Verify the output of the signing commands.
    Verify { text: String },
}

impl Command {
    fn is_sign(&self) -> bool {
        match self {
            Command::Text { .. } | Command::File { .. } => true,
            Command::Verify { .. } => false,
        }
    }
    fn is_verify(&self) -> bool {
        !self.is_sign()
    }

    fn get_string(&self) -> Result<String, Box<dyn std::error::Error + 'static>> {
        match self {
            Command::Text { text } | Command::Verify { text } => Ok(text.trim_end().to_string()),
            Command::File { file } => std::fs::read_to_string(file).map_err(Box::from),
        }
    }

    fn get_data(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        match self {
            Command::Text { text } | Command::Verify { text } => {
                Ok(text.trim_end().to_string().into_bytes())
            }
            Command::File { file } => std::fs::read(file).map_err(Box::from),
        }
    }
}

fn sign_with_key(
    key: &[u8],
    input: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let pkey = PKey::hmac(key)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
    signer.update(&input)?;
    signer.sign_to_vec().map_err(Box::from)
}

fn sign(args: Args) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let key = args.key.as_bytes();
    let input = args.command.get_data()?;
    let output = args.command.get_string()?;
    let hmac = base64::encode(sign_with_key(key, &input)?);
    Ok(format!("{}\nsha256:{}", output, hmac))
}

fn verify(args: Args) -> Result<bool, Box<dyn std::error::Error + 'static>> {
    let err_msg = "Expected format text\\nsha256:hash";
    let key = args.key.as_bytes();
    let input = args.command.get_string()?;
    let mut parts = input.split("sha256:");
    let input_text = parts
        .next()
        .ok_or_else(|| String::from(err_msg))?
        .trim_end();
    let hash = base64::decode(
        parts
            .next()
            .ok_or_else(|| String::from(err_msg))?
            .trim_end(),
    )?;
    let test = sign_with_key(key, input_text.as_bytes())?;
    Ok(memcmp::eq(&test, &hash))
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();

    if args.command.is_verify() {
        match verify(args)? {
            true => println!("Verified!"),
            false => return Err("NOT Verified!".into()),
        }
    } else {
        let output = sign(args)?;
        println!("{}", output);
    }
    Ok(())
}
