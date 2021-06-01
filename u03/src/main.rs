use std::iter;
use u03::tables;
use u03::tempr::{Celsius, Fahrenheit};

pub mod fnexplore;

fn main() {
    let mut temps =
        stepped_range_inclusive(Celsius::new(-40.0), Celsius::new(100.0), Celsius::new(5.0));
    tables::print_table(7, &["C", "K", "°F"], || {
        temps.next().map(|t| {
            vec![
                format!("{:5.1}", t.as_f64()),
                format!("{:5.1}", t.to_kelvin().as_f64()),
                format!("{:5.1}", t.to_fahrenheit().as_f64()),
            ]
        })
    });

    let mut temps = stepped_range_inclusive(
        Fahrenheit::new(-40.0),
        Fahrenheit::new(212.0),
        Fahrenheit::new(12.0),
    );
    tables::print_table(7, &["°F", "K", "C"], || {
        temps.next().map(|t| {
            vec![
                format!("{:5.1}", t.as_f64()),
                format!("{:5.1}", t.to_kelvin().as_f64()),
                format!("{:5.1}", t.to_celsius().as_f64()),
            ]
        })
    });

    // fnexplore::run();
}

fn stepped_range_inclusive<T>(begin: T, end: T, step: T) -> impl Iterator<Item = T>
where
    T: std::ops::Add<Output = T> + PartialOrd<T> + Copy,
{
    iter::successors(
        Some(begin),
        move |&t| {
            if t < end {
                Some(t + step)
            } else {
                None
            }
        },
    )
}
