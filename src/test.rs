use std::io::Write;

use tempfile::NamedTempFile;

use crate::args::{Args, Command};
use crate::{sign, sign_with_key, verify};

const TEXT1: &str = "⬛🟨⬛🟩⬛\n⬛🟨⬛🟩🟨\n🟩🟩🟩🟩🟩";
const TEXT2: &str = "⬛🟨⬛🟩⬛\n⬛🟨🟩🟩🟨\n🟩🟩🟩🟩🟩";
const KEY1: &str = "fiver";
const KEY2: &str = "sixer";

#[test]
fn sign_with_key_makes_new_sign_with_different_data() {
    let key = KEY1.as_bytes();
    let sign1 = sign_with_key(key, TEXT1.as_bytes()).unwrap();
    let sign2 = sign_with_key(key, TEXT2.as_bytes()).unwrap();
    assert_ne!(sign1, sign2);
}

#[test]
fn sign_with_key_makes_new_sign_with_different_key() {
    let text = TEXT2.as_bytes();
    let sign1 = sign_with_key(KEY1.as_bytes(), text).unwrap();
    let sign2 = sign_with_key(KEY2.as_bytes(), text).unwrap();
    assert_ne!(sign1, sign2);
}

#[test]
fn sign_invalid_file_returns_err() {
    // a somewhat roundabout way to create a random temptfile name
    let file = NamedTempFile::new().unwrap();
    let path = file.into_temp_path();
    let path_name = path.to_string_lossy().into_owned();
    path.close().unwrap();

    let args = Args {
        command: Command::File { file: path_name },
        key: KEY1.to_string(),
    };

    let result = sign(args);
    assert!(result.is_err());
}

#[test]
fn sign_with_file_matches_sign_with_text() {
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile.as_file_mut(), "{}", TEXT1).unwrap();
    let args1 = Args {
        command: Command::File {
            file: tmpfile.path().to_string_lossy().into_owned(),
        },
        key: KEY1.to_string(),
    };
    let args2 = Args {
        command: Command::Text {
            text: TEXT1.to_string(),
        },
        key: KEY1.to_string(),
    };
    let result1 = sign(args1).unwrap();
    let result2 = sign(args2).unwrap();
    assert_eq!(result1, result2);
}

#[test]
fn we_can_verify_a_file() {
    let mut tmpfile = NamedTempFile::new().unwrap();

    let sign_args = Args {
        command: Command::Text {
            text: TEXT1.to_string(),
        },
        key: KEY1.to_string(),
    };
    let signed_text = sign(sign_args).unwrap();
    write!(tmpfile.as_file_mut(), "{}", signed_text).unwrap();

    let verify_args = Args {
        command: Command::VerifyFile {
            file: tmpfile.path().to_string_lossy().into_owned(),
        },
        key: KEY1.to_string(),
    };

    let verify_result = verify(verify_args).unwrap();
    assert!(verify_result);
}

#[test]
fn we_can_verify_what_we_sign() {
    let sign_args = Args {
        command: Command::Text {
            text: TEXT1.to_string(),
        },
        key: KEY1.to_string(),
    };
    let sign_result = sign(sign_args).unwrap();
    let verify_args = Args {
        command: Command::Verify { text: sign_result },
        key: KEY1.to_string(),
    };

    assert!(verify(verify_args).unwrap());
}

#[test]
fn verify_with_different_key_and_text_is_err() {
    let sign_args = Args {
        command: Command::Text {
            text: TEXT1.to_string(),
        },
        key: KEY1.to_string(),
    };
    let sign_result = sign(sign_args).unwrap();
    let replaced_text = sign_result.replace(TEXT1, TEXT2);
    let verify_args1 = Args {
        command: Command::Verify { text: sign_result },
        key: KEY2.to_string(),
    };
    let verify_args2 = Args {
        command: Command::Verify {
            text: replaced_text,
        },
        key: KEY1.to_string(),
    };
    assert!(!verify(verify_args1).unwrap());
    assert!(!verify(verify_args2).unwrap());
}

#[test]
fn verify_with_wrong_command_is_err() {
    let sign_args = Args {
        command: Command::Text {
            text: TEXT1.to_string(),
        },
        key: KEY1.to_string(),
    };
    assert!(verify(sign_args).is_err());
}
