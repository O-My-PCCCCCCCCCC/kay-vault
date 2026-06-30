use sha2::{Digest, Sha256};

fn extract_digits(hash_str: &str) -> Vec<char> {
    hash_str.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn generate_password(digits: &[char], start_pos: usize, password_len: usize) -> String {
    if digits.is_empty() || start_pos >= digits.len() { return String::new(); }
    let n = digits.len();
    let mut password = String::with_capacity(password_len);
    let mut pos = start_pos % n;
    for _ in 0..password_len {
        password.push(digits[pos]);
        let step = digits[pos].to_digit(10).unwrap_or(0) as usize + 1;
        pos = (pos + step) % n;
    }
    password
}

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn run_chain(input: &str, len: usize) -> Option<(String, String, String)> {
    let hash1 = sha256_hex(input);
    let digits1 = extract_digits(&hash1);
    if digits1.len() < 3 { return None; }
    let a = generate_password(&digits1, 2, len);

    let hash2 = sha256_hex(&a);
    let digits2 = extract_digits(&hash2);
    if digits2.len() < 6 { return None; }
    let b = generate_password(&digits2, 5, len);

    let combined = format!("{}{}", a, b);
    let hash3 = sha256_hex(&combined);
    let digits3 = extract_digits(&hash3);
    if digits3.len() < 4 { return None; }
    let result = generate_password(&digits3, 3, len);

    Some((a, b, result))
}

pub fn compute(input1: &str, input2: &str) -> Result<(String, String, String), String> {
    compute_with_len(input1, input2, 6)
}

pub fn compute_with_len(input1: &str, input2: &str, len: usize) -> Result<(String, String, String), String> {
    if input1.trim().is_empty() { return Err("标识不能为空".into()); }
    if input2.trim().is_empty() { return Err("主密码不能为空".into()); }

    let max_len = input1.len().max(input2.len());
    let mut mixed = String::new();
    for i in 0..max_len {
        if i < input1.len() { mixed.push(input1.as_bytes()[i] as char); }
        if i < input2.len() { mixed.push(input2.as_bytes()[i] as char); }
    }
    let seed = sha256_hex(&mixed);

    let forward = run_chain(&seed, len).ok_or("正向链生成失败")?;
    let rev_input: String = seed.chars().rev().collect();
    let reverse = run_chain(&rev_input, len).ok_or("反向链生成失败")?;

    let combined = format!("{}{}", forward.2, reverse.2);
    let hash_final = sha256_hex(&combined);
    let digits_final = extract_digits(&hash_final);
    if digits_final.len() < 3 { return Err("最终数字太少".into()); }
    let final_password = generate_password(&digits_final, 2, len);

    Ok((forward.2, reverse.2, final_password))
}
