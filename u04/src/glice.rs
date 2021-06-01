use std::fmt;

use crate::chessboard::ChessBoard;

pub fn does_vec_capacity_double_on_realloc() {
    let mut xs = Vec::new();
    let mut last_cap = xs.capacity();
    for n in 0..2048 {
        xs.push(n);
        if xs.capacity() != last_cap {
            println!("{}", xs.capacity());
            last_cap = xs.capacity();
        }
    }
}

pub fn explore_vectors() {
    // xs := {}
    let mut xs = vec![];
    dump_vec(&xs);
    // xs = append(xs, 42)
    xs.push(42);
    dump_vec(&xs);
    // xs = append(xs, 3, 7, 1)
    // xs = append(xs, arr...)
    xs.extend(&[3, 7, 1]);
    dump_vec(&xs);
    // xs = append(xs, 420)
    xs.push(420);
    dump_vec(&xs);

    // xs := make([]int32, 0, 4)
    let mut xs = Vec::with_capacity(4);
    dump_vec(&xs);
    // xs = append(xs, 71)
    xs.push(71);
    dump_vec(&xs);

    // xs := []int{1,2,3,4} // len=4, cap=4
    let mut xs = vec![1, 2, 3, 4];
    dump_vec(&xs);
    // ys := xs[1:3]   // len=2, cap=3
    // can write to xs beyond the range [1:3] through ys by append(ys, 7)

    // zs := xs[1:3:2]   // len=2, cap=2
    // cannot change xs outside of the range [1:3] through zs;
    // append(zs, 7) will allocate a new backing array
    println!("Before: xs is {:?}", xs);
    {
        let zs = &mut xs[1..3];
        zs.fill(96);
    }
    println!("After: xs is {:?}", xs);
}

pub fn dump_vec<T>(v: &Vec<T>)
where
    T: fmt::Debug,
{
    println!(
        "ptr={:016p} len={:3} cap={:3}",
        v.as_ptr(),
        v.len(),
        v.capacity()
    );
    println!(" {:?}", v);
}

pub fn terraforming() {
    let mut planets = Planets::new(vec![
        "Vulcan".into(),
        "Earth".into(),
        "Ferenginar".into(),
        "Bajor".into(),
    ]);
    println!("Before: {:?}", planets);
    planets.terraform();
    println!("After: {:?}", planets);
}

#[derive(Debug)]
pub struct Planets(Vec<String>);

impl Planets {
    pub fn new(planets: Vec<String>) -> Self {
        Self(planets)
    }

    pub fn terraform(&mut self) {
        for planet in &mut self.0 {
            planet.push_str(" Nova");
        }
    }
}

pub fn explore_slicing_mut() {
    let mut planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    println!("Before {:?}", planets);
    {
        // slices are views into the underlying array (slice)
        // and modifications are reflected in the backing  array/slice.
        let some = &mut planets[1..3];
        some[0] = "Venera";
    }
    println!("After {:?}", planets);

    let trimmed: Vec<_> = planets.iter().map(|p| p.trim()).collect();
    println!("Trimmed: {}", trimmed.concat());

    {
        let inners = &mut planets[..4];
        inners.sort();
    }

    println!("After sorting inners {:?}", planets);
}

pub fn explore_slicing() {
    let planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];

    println!("All: {:?}", planets);
    // slice arrays
    // missing begin defaults to 0
    let terrestrial = &planets[..4];
    println!("Rocky: {:?}", terrestrial);
    let gas = &planets[4..6];
    println!("Gassy: {:?}", gas);
    // missing end defaults to length of slice/array
    let ice = &planets[6..];
    println!("Icy: {:?}", ice);
    let giants = &planets[4..8];
    println!("Giants: {:?}", giants);
    // slice slices
    let ggas = &giants[0..2];
    println!("Gassy of Gs: {:?}", ggas);
    let gice = &giants[2..4];
    println!("Icy of Gs: {:?}", gice);

    // explicitly create a slice from all the elements of an array
    let all = &planets[..];
    println!("All your sliced {:?} are belong to us.", all);
    // implicitly create a slice of all the elements of an array
    // here `all` requires a slice (&[T]) and a reference to an array (&[T;N]) is
    // converted to a slice
    let all: &[&str] = &planets;
    println!("All your implicitly as_ref'd {:?} are belong to us.", all);
    let all: &[&str] = planets.as_ref();
    println!("All your explicitly as_ref'd {:?} are belong to us.", all);
    let all: &[&str] = AsRef::as_ref(&planets);
    println!(
        "All your desugared explicitly as_ref'd {:?} are belong to us.",
        all
    );

    let zz = &[1, 2, 3];
    println!("Ref to array literal {:?}", zz);
    let zz: &[i32] = &[1, 2, 3];
    println!("Ref to slice literal; this works because the `&[T;N]` is implicitly converted into an `&[T]` {:?}", zz);
}

pub fn explore_chessboard() {
    let board = ChessBoard::at_start();
    println!("{}", board);
}

pub fn explore_multidimensional_arrays() {
    let mut ttt = [[' '; 3]; 3];
    ttt[0][1] = 'X';
    ttt[2][2] = 'O';
    for row in &ttt {
        for cell in row {
            print!("[{}]", cell);
        }
        println!();
    }
}

pub fn explore_array() {
    let mut planets = [""; 8];
    println!("{:?}", planets);
    planets[0] = "Mercury";
    planets[1] = "Venus";
    planets[2] = "Earth";
    println!("{:?}", planets);
    println!("{:?}", planets[0]);
    // println!("{:?}", planets[8]); // compile error
    // println!("{:?}", planets[which()]); // runtime panic

    let dwarves = ["Ceres", "Eris", "Makemake"];
    println!("{:?}", dwarves);

    let explicit_dwarfs: [&str; 3] = ["Ceres", "Eris", "Makemake"];
    println!("{:?}", explicit_dwarfs);

    for (i, d) in dwarves.iter().enumerate() {
        println!("{} @ {}", d, i);
    }

    for (n, d) in (1..).zip(dwarves.iter()) {
        println!("No. {:2}: {}", n, d);
    }
}

pub fn explore_arrays_of_copy_are_copy() {
    let mut mits = [1, 2, 3];
    for mit in mits.iter() {
        println!("By ref! It's {}", mit);
    }
    for mit in mits.iter_mut() {
        *mit += 2;
        println!("By mut ref! It's {}", mit);
    }
    for mit in std::array::IntoIter::new(mits) {
        println!("Me take {}", mit);
    }
    println!("mits is {:?}", mits);
    let mut bits = mits;

    println!("mits is {:?}", mits);
    println!("bits is {:?}, a copy of mits!", bits);

    bits[1] = 77;

    println!("mits is now {:?}", mits);
    println!(
        "bits is now {:?}; changes to bits do not affect mits!",
        bits
    );
}

pub fn explore_tuples_of_copy_are_copy() {
    let sig = (1, 'c');
    let mut sug = sig;
    println!("sig is {:?}", sig);
    println!("sug is {:?}", sug);
    sug.1 = 'x';

    println!("sig is now {:?}", sig);
    println!("sug is now {:?}; updates to sug does not affect sig!", sug);
}

// to simulate a runtime value
pub fn which() -> usize {
    8
}
