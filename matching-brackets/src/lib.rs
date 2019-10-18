pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = vec![];
    let mut is_valid = true;

    for character in string.chars() {
        match character {
            '{' | '[' | '(' => vec.push(character),
            '}' | ']' | ')' => is_valid = check_previous(&mut vec, character, is_valid),
            _ => {},
        }
    }

    is_valid && vec.is_empty()
}

fn check_previous(vec: &mut Vec<char>, character: char, is_valid: bool ) -> bool {
    let flip = match character {
        ']' => '[',
        '}' => '{',
        ')' => '(',
        _ => panic!("Invalid Character!"),
    };

    if !vec.is_empty() && vec.last().expect("ERROR1") == &flip {
        vec.pop();
        is_valid
    } else {
        false
    }
}
