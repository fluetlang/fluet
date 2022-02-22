/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub fn is_digit(c: &str) -> bool {
    c.chars().nth(0).unwrap_or_default().is_ascii_digit()
}

pub fn is_alpha(c: &str) -> bool {
    c.chars().nth(0).unwrap_or_default().is_ascii_alphabetic() || c == "_"
}

pub fn is_alphanumeric(c: &str) -> bool {
    is_alpha(c) || is_digit(c)
}

// TODO: figure out more valid chars for identifiers
//       or even blacklist chars and allow all other chars
pub fn is_valid_identifier(c: &str) -> bool {
    is_alphanumeric(c)
}
