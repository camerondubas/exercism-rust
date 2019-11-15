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
            output = build_output(current_count, current_letter, output);
            current_letter = c;
            current_count = 1;
        }

        match peekable_chars.peek() {
            None => output = build_output(current_count, current_letter, output),
            _ => {}
        }
    }
    output
}

fn build_output(count: u8, letter: char, output: String) -> String {
    let partial = match count {
        1 => format!("{}", letter),
        _ => format!("{}{}", count, letter),
    };

    return format!("{}{}", output, partial);
}

fb han

pub fn decode(source: &str) -> String {
    let mut output: String = String::new();

    if source.is_empty() {
        return output;
    }

    let mut chars_iter = source.chars();
    let mut num_str = String::new();

    while let Some(c) = chars_iter.next() {
        if c.is_ascii_digit() {
            num_str.push(c);
        } else {
            let p: u8 = num_str.parse().unwrap_or(1);
            let partial: String = (1..=p).map(|_| c).collect::<String>();
            output = format!("{}{}", output, partial);
            num_str = String::new();
        }
    }

    output
}
