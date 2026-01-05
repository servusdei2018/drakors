// Drakors
// Copyright (C) 2025-present  Nathanael Bracy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use blake3::Hasher;
use rand::TryRngCore;
use rand::rngs::OsRng;
use subtle::ConstantTimeEq;

const SALT_LEN: usize = 16;
const OUTPUT_LEN: usize = 32;

pub fn hash_password(password: &str) -> String {
    let mut salt = [0u8; SALT_LEN];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut salt)
        .expect("Failed to generate random salt from OS RNG");

    let mut hasher = Hasher::new();
    hasher.update(&salt);
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();

    let salt_b64 = STANDARD.encode(&salt);
    let hash_b64 = STANDARD.encode(hash.as_bytes());

    format!("BLAKE3${}${}", salt_b64, hash_b64)
}

#[allow(dead_code)]
pub fn verify_password(password: &str, stored: &str) -> bool {
    let parts: Vec<&str> = stored.split('$').collect();
    if parts.len() != 3 || parts[0] != "BLAKE3" {
        return false;
    }

    let salt = match STANDARD.decode(parts[1]) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let expected = match STANDARD.decode(parts[2]) {
        Ok(h) => h,
        Err(_) => return false,
    };

    if expected.len() != OUTPUT_LEN {
        return false;
    }

    let mut hasher = Hasher::new();
    hasher.update(&salt);
    hasher.update(password.as_bytes());
    let computed = hasher.finalize();

    computed.as_bytes().ct_eq(&expected).into()
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long.");
    }

    let mut flags = 0u8;

    for c in password.chars() {
        if c.is_uppercase() {
            flags |= 1
        }
        if c.is_lowercase() {
            flags |= 2
        }
        if c.is_ascii_digit() {
            flags |= 4
        }
        if flags == 7 {
            return Ok(());
        }
    }

    match flags {
        f if f & 1 == 0 => Err("Password must contain at least one uppercase letter."),
        f if f & 2 == 0 => Err("Password must contain at least one lowercase letter."),
        _ => Err("Password must contain at least one digit."),
    }
}
