use clap::Parser;

use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::pkey::PKey;
use openssl::sign::Signer;

mod args;
#[cfg(test)]
mod test;

use args::Args;

fn sign_with_key(
    key: &[u8],
    input: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let pkey = PKey::hmac(key)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
    signer.update(input)?;
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
