pub fn binary(input: &str) -> [String; 3] {
    let value = u64::from_str_radix(input, 2).unwrap_or(0);

    let octal = format!("{:o}", value);
    let decimal = value.to_string();
    let hexadecimal = format!("{:x}", value);

    [octal, decimal, hexadecimal]
}

pub fn octal(input: &str) -> [String; 3] {
    let value = u64::from_str_radix(input, 8).unwrap_or(0);

    let binary = format!("{:b}", value);
    let decimal = value.to_string();
    let hexadecimal = format!("{:x}", value).to_lowercase();

    [binary, decimal, hexadecimal]
}

pub fn decimal(input: &str) -> [String; 3] {
    let value = input.parse::<u64>().unwrap_or(0);

    let binary = format!("{:b}", value);
    let octal = format!("{:o}", value);
    let hexadecimal = format!("{:x}", value).to_lowercase();

    [binary, octal, hexadecimal]
}

pub fn hexadecimal(input: &str) -> [String; 3] {
    let value = u64::from_str_radix(input, 16).unwrap_or(0);

    let binary = format!("{:b}", value);
    let octal = format!("{:o}", value);
    let decimal = value.to_string();

    [binary, octal, decimal]
}
