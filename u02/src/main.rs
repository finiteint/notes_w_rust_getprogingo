fn main() {
    // ascii_rot_13();
    let decoded = ascii_vig_decode("CSOITEUIWUIZNSROCNKFD", "GOLANG");
    println!("{}", decoded);
    let msg = "Ve dig you. Luv, Ze Gophers";
    let key = "endoplasmic bombing";
    println!("Cleartext : {}", msg);
    let coded = ascii_vig_encode(msg, key);
    println!("Ciphertext: {}", coded);
    println!("Cleartext : {}", ascii_vig_decode(&coded, key));
}

fn ascii_vig_decode(ciphertext: &str, key: &str) -> String {
    ciphertext
        .chars()
        .zip(key_to_shifts(key).into_iter().cycle())
        .map(|(c, shift)| ascii_rot_shift(c, 26 - shift))
        .collect::<String>()
}

fn ascii_vig_encode(cleartext: &str, key: &str) -> String {
    cleartext
        .chars()
        .zip(key_to_shifts(key).into_iter().cycle())
        .map(|(c, shift)| ascii_rot_shift(c, shift))
        .collect::<String>()
}

fn key_to_shifts(key: &str) -> Vec<u8> {
    const START: u8 = 'A' as u8;
    key.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                c.to_ascii_uppercase() as u8 - START
            } else {
                0
            }
        })
        .collect::<Vec<_>>()
}

pub fn ascii_rot_13() {
    let message = "uv vagreangvbany fcnpr fgngvba";
    let decoded = rot13_decode(message);
    println!("Ciphertext: {}", message);
    println!("Cleartext : {}", decoded);
    println!("Ciphertext: {}", rot13_encode(&decoded));

    let message = "Hola Estación Espacial Internacional";
    let coded = rot13_encode(message);
    println!("Ciphertext: {}", coded);
    println!("Cleartext: {}", rot13_decode(&coded));

    let julius = "L fdph, L vdz, L frqtxhuhg.";
    let clear_julius = ascii_rot_shift_decode(3, julius);
    println!("Julius said `{}`", clear_julius);
}

fn rot13_decode(ciphertext: &str) -> String {
    ascii_rot_shift_decode(13, ciphertext)
}

fn rot13_encode(cleartext: &str) -> String {
    ascii_rot_shift_encode(13, cleartext)
}

fn ascii_rot_shift_encode(shift: u8, cleartext: &str) -> String {
    cleartext
        .chars()
        .map(|c| ascii_rot_shift(c, shift))
        .collect::<String>()
}

fn ascii_rot_shift_decode(shift: u8, ciphertext: &str) -> String {
    ciphertext
        .chars()
        .map(|c| ascii_rot_shift(c, 26 - shift))
        .collect::<String>()
}

#[inline]
fn ascii_rot_shift(c: char, shift: u8) -> char {
    const WIDTH: u8 = 26;
    const START: u8 = 'a' as u32 as u8;
    const USTART: u8 = 'A' as u32 as u8;
    if c.is_ascii_lowercase() {
        let shifted = ((c as u8 - START) + shift) % WIDTH;
        (START + shifted) as char
    } else if c.is_ascii_uppercase() {
        let shifted = ((c as u8 - USTART) + shift) % WIDTH;
        (USTART + shifted) as char
    } else {
        c
    }
}
