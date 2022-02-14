pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

// TODO: figure out more valid chars for identifiers
//       or even blacklist chars and allow all other chars
pub fn is_valid_identifier(c:char) -> bool {
    is_alphanumeric(c)
}
