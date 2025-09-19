use clap::{Arg, Command};
use std::collections::HashMap;

const CODEBOOK: &[char] = &['齁', '哦', '噢', '喔', '咕', '咿', '嗯', '啊', '～', '哈', '！', '唔', '哼', '❤', '呃', '呼'];

fn main() {
    let matches = Command::new("Encoder/Decoder")
        .version("1.0")
        .author("Nostalgia")
        .about("网站加解密的rust实现")
        .arg(
            Arg::new("命令")
                .help("包括 en 和 de ,分别用于加密和解密")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("输入")
                .help("输入需要加密或解密的文本")
                .required(true)
                .index(2),
        )
        .get_matches();

    let command = matches.get_one::<String>("命令").unwrap();
    let input = matches.get_one::<String>("输入").unwrap();

    match command.as_str() {
        "en" => {
            let encoded = encode(input);
            println!("已加密: {}", encoded);
        }
        "de" => {
            match decode(input) {
                Ok(decoded) => println!("已解密: {}", decoded),
                Err(e) => eprintln!("错误: {}", e),
            }
        }
        _ => eprintln!("无效参数，使用 --help 获取帮助"),
    }
}

fn encode(input: &str) -> String {
    let encoder = input.as_bytes();
    let mut encoded = String::new();
    for &byte in encoder {
        let high = (byte >> 4) & 0x0F;
        let low = byte & 0x0F;
        encoded.push(CODEBOOK[high as usize]);
        encoded.push(CODEBOOK[low as usize]);
    }
    encoded
}

fn decode(input: &str) -> Result<String, String> {
    if input.len() % 2 != 0 {
        return Err("输入长度必须为偶数".to_string());
    }

    let codebook_map: HashMap<char, usize> = CODEBOOK.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let mut bytes = Vec::new();
    let mut chars = input.chars().peekable();

    while let (Some(high_char), Some(low_char)) = (chars.next(), chars.next()) {
        if high_char.is_whitespace() || low_char.is_whitespace() {
            continue;
        }

        let high = codebook_map.get(&high_char).ok_or(format!("非法字符: {}", high_char))?;
        let low = codebook_map.get(&low_char).ok_or(format!("非法字符: {}", low_char))?;

        let byte = (high << 4) | low;
        bytes.push(byte as u8);
    }

    match String::from_utf8(bytes) {
        Ok(decoded) => Ok(decoded),
        Err(_) => Err("无法正确解码为UTF-8文本".to_string()),
    }
}
