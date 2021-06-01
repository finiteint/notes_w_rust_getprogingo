use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Kelvin(f64);

impl Kelvin {
    pub fn new(t: f64) -> Self {
        Self(t)
    }

    pub fn to_celsius(self) -> Celsius {
        Celsius(self.0 - 273.15)
    }

    pub fn to_fahrenheit(self) -> Fahrenheit {
        self.to_celsius().to_fahrenheit()
    }
}

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} K", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Fahrenheit(f64);

impl Fahrenheit {
    pub fn new(t: f64) -> Self {
        Self(t)
    }

    pub fn to_celsius(self) -> Celsius {
        Celsius((5.0 * (self.0 - 32.0)) / 9.0)
    }

    pub fn to_kelvin(self) -> Kelvin {
        self.to_celsius().to_kelvin()
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °F", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Celsius(f64);

impl Celsius {
    pub const fn new(t: f64) -> Self {
        Self(t)
    }

    pub fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit((self.0 * 9.0) / 5.0 + 32.0)
    }

    pub fn to_kelvin(self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} C", self.0)
    }
}

macro_rules! impl_common_temp_features {
    ($($typ:ty),*) => {
        $(

        impl std::cmp::PartialEq for $typ {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl std::cmp::PartialOrd for $typ {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl std::ops::Add for $typ {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign for $typ {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl std::ops::AddAssign<f64> for $typ {
            fn add_assign(&mut self, rhs: f64) {
                self.0 += rhs;
            }
        }

        impl $typ {
            pub fn as_f64(self) -> f64 {
                self.0
            }
        }
        )*
    };
}

impl_common_temp_features!(Kelvin, Celsius, Fahrenheit);
