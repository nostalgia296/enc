use clap::{Arg, Command};
use std::collections::HashMap;

const CODEBOOK: &[char] = &['齁', '哦', '噢', '喔', '咕', '咿', '嗯', '啊', '～', '哈', '！', '唔', '哼', '❤', '呃', '呼'];

fn main() {
    let matches = Command::new("Encoder/Decoder")
        .version("1.0")
        .author("Nostalgia")
        .about("https://msbt.seku.su/网站加解密的rust实现")
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
                Ok(decoded) => println!("以解密: {}", decoded),
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
    for i in (0..input.len()).step_by(2) {
        let high = codebook_map.get(&input.chars().nth(i).unwrap());
        let low = codebook_map.get(&input.chars().nth(i + 1).unwrap());
        if high.is_none() || low.is_none() {
            return Err("输入包含非法字符".to_string());
        }
        let byte = (high.unwrap() << 4) | low.unwrap();
        bytes.push(byte as u8);
    }

    match String::from_utf8(bytes) {
        Ok(decoded) => Ok(decoded),
        Err(_) => Err("无法正确解码为UTF-8文本".to_string()),
    }
}
