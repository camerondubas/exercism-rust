pub fn encode(source: &str) -> String {
    let mut output: String = String::from("");

    if source.is_empty() {
        return output;
    }

    let mut peekable_chars = source.chars().peekable();
    let mut current_letter: char = source.chars().next().unwrap();
    let mut current_count: u8 = 0;

    while let Some(c) = peekable_chars.next() {
        if c == current_letter {
            current_count = current_count + 1;
        } else {
            output = build_encode_output(current_count, current_letter, output);
            current_letter = c;
            current_count = 1;
        }

        match peekable_chars.peek() {
            None => output = build_encode_output(current_count, current_letter, output),
            _ => {}
        }
    }
    output
}

pub fn decode(source: &str) -> String {
    let mut output: String = String::new();
    let mut chars_iter = source.chars();
    let mut num_str = String::new();

    while let Some(c) = chars_iter.next() {
        if c.is_ascii_digit() {
            num_str.push(c);
        } else {
            let count = num_str.parse().unwrap_or(1);
            output = output + c.to_string().repeat(count).as_str();
            num_str = String::new();
        }
    }

    output
}


fn build_encode_output(count: u8, letter: char, output: String) -> String {
    let partial = match count {
        1 => format!("{}", letter),
        _ => format!("{}{}", count, letter),
    };

    return format!("{}{}", output, partial);
}
