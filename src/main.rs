use clap::Parser;

use ring::hmac;

mod args;
#[cfg(test)]
mod test;

use args::Args;

fn sign_with_key(
    key: &[u8],
    input: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let pkey = hmac::Key::new(hmac::HMAC_SHA256, key);
    Ok(hmac::sign(&pkey, input).as_ref().to_owned())
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
    let pkey = hmac::Key::new(hmac::HMAC_SHA256, key);
    match hmac::verify(&pkey, input_text.as_bytes(), &hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
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
