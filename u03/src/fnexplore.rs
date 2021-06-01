use rand::Rng;
use std::thread;
use std::time::Duration;

use u03::tempr::Kelvin;

pub fn run() {
    let k = Kelvin::new(255.0);
    println!("{} is {} and also {}", k, k.to_celsius(), k.to_fahrenheit());
    hofn_exploration();
}

pub fn hofn_exploration() {
    let always_42_sensor = || Kelvin::new(42.0);

    let mut sensor: fn() -> Kelvin = random_sensor;
    println!("-> {}", sensor());
    sensor = always_0_sensor;
    println!("-> {}", sensor());
    sensor = always_42_sensor;
    println!("-> {}", sensor());
    sensor = || Kelvin::new(371.0);
    println!("-> {}", sensor());

    measure_n_fptr(2, random_sensor);
    measure_n(2, random_sensor);
    measure_n_fptr(2, always_42_sensor);
    measure_n(2, always_42_sensor);
    measure_n_fptr(2, || Kelvin::new(371.0));
    measure_n(2, || Kelvin::new(371.0));
    measure_n(2, calibrate(always_42_sensor, Kelvin::new(3.0)));

    let mut counting_sensor = mk_counting_sensor(Kelvin::new(3.0));
    measure_n(2, calibrate(&mut counting_sensor, Kelvin::new(3.0)));
    measure_n(2, &mut counting_sensor);
}

fn mk_counting_sensor(mut start: Kelvin) -> impl FnMut() -> Kelvin {
    move || {
        dbg!(start);
        start += 1.0;
        start
    }
}

fn calibrate(mut sensor: impl FnMut() -> Kelvin, offset: Kelvin) -> impl FnMut() -> Kelvin {
    move || sensor() + offset
}

fn random_sensor() -> Kelvin {
    Kelvin::new(rand::thread_rng().gen_range(150.0..300.0))
}

fn always_0_sensor() -> Kelvin {
    Kelvin::new(0.0)
}

fn measure_n(n: usize, mut sensor: impl FnMut() -> Kelvin) {
    for i in 0..n {
        println!("{:02}: {}", i, sensor());
        thread::sleep(Duration::from_millis(125));
    }
}

fn measure_n_fptr(n: usize, sensor: fn() -> Kelvin) {
    for i in 0..n {
        println!("{:02}: {}", i, sensor());
        thread::sleep(Duration::from_millis(125));
    }
}
