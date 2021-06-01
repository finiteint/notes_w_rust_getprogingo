pub mod worldexplore;

use rand::{prelude::SliceRandom, Rng};
use std::fmt;

fn main() {
    explore_interfaces_animal_sim()
}

pub fn explore_interfaces_animal_sim() {
    let fish = Fish {};
    let bat = Bat {};
    let cow = Cow {};
    let animals: &[&dyn Animal] = &[&fish, &bat, &cow];
    let mut rng = rand::thread_rng();
    for hour in 0..72 {
        let hour_of_day = hour % 24;
        match hour_of_day {
            6..=18 => {
                let animal = animals.choose(&mut rng).unwrap();
                let action = if rng.gen_bool(0.5) {
                    format!("says {}", animal.say())
                } else {
                    format!("eats {}", animal.eat())
                };
                println!("Hour {:02}: {} {}", hour, animal, action)
            }
            _ => println!("Hour {:02}: everyone's asleep", hour),
        }
    }
}

pub trait Animal: fmt::Display {
    fn say(&self) -> String;
    fn eat(&self) -> String;
}

pub struct Cow {}

impl fmt::Display for Cow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cow")
    }
}

impl Animal for Cow {
    fn say(&self) -> String {
        "moo".into()
    }

    fn eat(&self) -> String {
        "grass".into()
    }
}
pub struct Fish {}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fish")
    }
}

impl Animal for Fish {
    fn say(&self) -> String {
        "blub".into()
    }

    fn eat(&self) -> String {
        "krill".into()
    }
}
pub struct Bat {}

impl fmt::Display for Bat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bat")
    }
}

impl Animal for Bat {
    fn say(&self) -> String {
        "click".into()
    }

    fn eat(&self) -> String {
        "moth".into()
    }
}

pub fn explore_interface_and_composition() {
    let f = Foo {};
    println!("f is {}", f.foo());
    let z = Zoo { inner: Foo {} };
    println!("z is {}", z.foo());
    let n = Noo(Zoo { inner: Foo {} });
    println!("n is {}", n.foo());
}

pub trait Z {
    fn foo(&self) -> bool;
}

macro_rules! derive_Z_delegating {
    ($type:ty) => {
        derive_Z_delegating!($type, 0);
    };

    ($type: ty, $field:tt) => {
        impl Z for $type {
            fn foo(&self) -> bool {
                self.$field.foo()
            }
        }
    };
}

struct Foo {}

impl Z for Foo {
    fn foo(&self) -> bool {
        false
    }
}

struct Zoo {
    inner: Foo,
}

derive_Z_delegating!(Zoo, inner);

struct Noo(Zoo);

derive_Z_delegating!(Noo);
