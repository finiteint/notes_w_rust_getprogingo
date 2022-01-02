use chrono::{TimeZone, Utc};
use num::BigInt;
use rand::prelude::SliceRandom;

pub fn run() {
    let when = Utc.timestamp_opt(12622780800, 0).unwrap();
    println!("{}", when);
    floating_piggy_bank();
    integral_piggy_bank();
    canis_distance();
    chars_exp();
    types_circus();
}

pub trait AsCBool {
    fn cbool(self) -> bool;
}

macro_rules! impl_as_c_bool_for_ints {
    ($($typ:ty),*) => {
        $(impl AsCBool for $typ {
            fn cbool(self) -> bool {
                self != 0
            }
        })*
    };
}

impl_as_c_bool_for_ints!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_as_c_bool_for_ptrs {
    (<$typ_var:ident> => $($typ:ty),*) => {
        $(impl<$typ_var> AsCBool for $typ  {
            fn cbool(self) -> bool {
                !self.is_null()
            }
        })*
    };
}

impl_as_c_bool_for_ptrs!(<T> => *const T, *mut T);

fn types_circus() {
    let won: bool = false;
    let wonint = won as usize;
    println!("{} is {}", won, wonint);
    let wonint = 1;
    let won = wonint != 0;
    println!("{} is {}", wonint, won);
    println!("{} is {}", 75u32, 75u32.cbool());
    let i = 42;
    let ip = &i as *const i32;
    let ip2 = ip;
    let mip = ip as *mut i32;
    println!("Are {:?} and {:?} ze same? Vat aboot {:?}?", ip, ip2, mip);
    println!("{} {}", ip.cbool(), mip.cbool());
    println!("Are {:?} and {:?} ze same? Vat aboot {:?}?", ip, ip2, mip);
}

fn chars_exp() {
    let question = "¿Cómo estás?";
    println!("bytes: {}", question.len());
    println!("chars: {}", question.chars().count());
    if let Some(c) = question.chars().next() {
        println!("`{}` of {} bytes", c, c.len_utf8());
    }
    for (i, c) in question.char_indices() {
        println!("{:2} `{}`", i, c);
    }

    fn sho(s: &str) {
        println!("{}", s);
    }

    let pint: u32 = 960;
    let pi: char = char::from_u32(pint).unwrap();
    sho(&pi.to_string());
    let pi: char = '\u{03c0}';
    sho(&pi.to_string());
    let bang: u8 = 33;
    let bang: char = bang as char;
    sho(&bang.to_string());
}

fn canis_distance() {
    let canis_distance = BigInt::parse_bytes(b"236000000000000000", 10).unwrap();
    let light_year = BigInt::from(365) * 24 * 60 * 60 * 300_000; // km
    let canis_ly = &canis_distance / light_year;
    println!(
        "Canis Major is {} km ({} light years) away from us.",
        canis_distance, canis_ly
    );
}

fn integral_piggy_bank() {
    let mut rng = rand::thread_rng();

    const DENOMS: [i32; 3] = [5, 10, 25];
    const TARGET: i32 = 2000;
    let mut piggy = 0;
    while piggy < TARGET {
        piggy += DENOMS.choose(&mut rng).unwrap();
        println!("iPiggy at ${:2}.{:02}.", piggy / 100, piggy % 100);
    }
    println!(
        "Finally! iPiggy is at ${:2}.{:02}.",
        piggy / 100,
        piggy % 100
    );
}

fn floating_piggy_bank() {
    let mut rng = rand::thread_rng();

    const DENOMS: [f64; 3] = [0.05, 0.10, 0.25];
    const TARGET: f64 = 20.0;
    let mut piggy = 0.0;
    while piggy < TARGET {
        piggy += DENOMS.choose(&mut rng).unwrap();
        println!("Piggy at ${:5.2}.", piggy);
    }
    println!("Finally! Piggy is at ${:5.2}.", piggy);
}
