// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

/// Normalize phone number, remove prefix and non digit characters.
pub(crate) fn normalize_phone_number<T: AsRef<str> + std::fmt::Display>(phone_number: T) -> String {
    let mut phone_number = phone_number.to_string();
    phone_number.retain(|c| !(b"- ()".contains(&(c as u8))) );
    if phone_number.starts_with("62") {
        phone_number = phone_number[2..].to_string();
    }else if phone_number.starts_with("0") {
        phone_number = phone_number[1..].to_string();
    }
    phone_number
}

