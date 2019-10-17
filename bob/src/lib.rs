pub fn reply(message: &str) -> &str {
    if is_yell(message) && is_question(message) {
      "Calm down, I know what I'm doing!"
    } else if  message.trim().is_empty() {
      "Fine. Be that way!"
    } else if is_question(message) {
      "Sure."
    } else if is_yell(message) {
      "Whoa, chill out!"
    } else {
      "Whatever."
    }
}


fn is_yell(message: &str) -> bool {
  message == message.to_uppercase() && message != message.to_lowercase()
}

fn is_question(message: &str) -> bool {
  message.trim().ends_with("?")
}
