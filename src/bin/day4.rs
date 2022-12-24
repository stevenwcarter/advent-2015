use std::io::Read;

use md5::{Digest, Md5};
use stringreader::StringReader;

fn main() {
    let input: &str = "bgvyzdsv";

    let answer_a = a(input, 5);
    println!("A Score: {}", answer_a);

    let b_result = a(input, 6);
    println!("B Score: {}", b_result);
}

fn md5_digest<R: Read>(mut reader: R) -> Result<String, Box<dyn std::error::Error>> {
    let mut context = Md5::new();
    let mut buffer = [0; 65536];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.input(&buffer[..count]);
    }

    let md5data = context.result();
    let md5 = format!("{:x}", md5data);
    Ok(md5)
}

fn a(input: &str, num_zeroes: usize) -> u32 {
    let mut nonce = 0;

    let mut nonce_found = false;
    while !nonce_found {
        let test_string = format!("{}{}", input, nonce);
        let to_hash = StringReader::new(test_string.as_str());
        let result = md5_digest(to_hash);
        if let Ok(result) = result {
            if result.starts_with(&"000000000"[0..num_zeroes]) {
                nonce_found = true;
            }
        }
        if !nonce_found {
            nonce += 1;
        }
    }

    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a("abcdef", 5), 609043);
        println!("Found 1");
        assert_eq!(a("pqrstuv", 5), 1048970);
    }

    #[test]
    fn test_b() {
        assert_eq!(a("abcdef", 6), 70);
    }
}
