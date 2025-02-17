use soroban_sdk::{Env, String};

/// Trait providing additional string manipulation utilities.
pub trait StringExtensions {
    /// Concatenates two `String` values while ensuring a max length of 35 characters.
    fn concat(&self, e: &Env, other: String) -> String;
}

impl StringExtensions for String {
    fn concat(&self, e: &Env, other: String) -> String {
        let len_0 = self.len() as usize;
        let len_1 = other.len() as usize;
        let combined_len = (len_0 + len_1).min(35); // Enforce max length

        let mut slice: [u8; 35] = [0; 35];
        self.copy_into_slice(&mut slice[..len_0]);
        other.copy_into_slice(&mut slice[len_0..combined_len]);

        String::from_str(&e, core::str::from_utf8(&slice[..combined_len]).unwrap())
    }
}