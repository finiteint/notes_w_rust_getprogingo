use std::fmt;

use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug)]
pub struct Rover {
    name: String,
    gps: Gps,
}

impl Rover {
    pub fn new(name: String, gps: Gps) -> Self {
        Self { name, gps }
    }

    pub fn message(&self) -> String {
        format!("{}: {}", self.name, self.gps.message())
    }
}

#[derive(Debug, Clone)]
pub struct Gps {
    world: World,
    current: Location,
    destination: Location,
}

impl Gps {
    pub fn new(world: World, current: Location, destination: Location) -> Self {
        Self {
            world,
            current,
            destination,
        }
    }

    pub fn distance(&self) -> f64 {
        self.world.distance(&self.current, &self.destination)
    }

    pub fn message(&self) -> String {
        format!(
            "{:.1} km to {}",
            self.distance(),
            self.destination.description()
        )
    }
}

pub fn earth() -> World {
    World::new("Earth".to_string(), 6_371.0)
}

pub fn mars() -> World {
    World::new("Mars".to_string(), 3_389.5)
}

#[derive(Debug, Clone)]
pub struct World {
    name: String,
    radius: f64,
}

impl World {
    pub fn new(name: String, radius: f64) -> Self {
        Self { name, radius }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn distance(&self, a: &Location, b: &Location) -> f64 {
        // println!(
        //     "a: {} ({}, {})",
        //     a,
        //     a.latitude().to_decimal_degrees(),
        //     a.longitude().to_decimal_degrees()
        // );
        // println!(
        //     "b: {} ({}, {})",
        //     b,
        //     b.latitude().to_decimal_degrees(),
        //     b.longitude().to_decimal_degrees()
        // );

        let (sa, ca) = a.latitude().to_decimal_radians().sin_cos();
        let (sb, cb) = b.latitude().to_decimal_radians().sin_cos();
        let clong = (a.longitude().to_decimal_degrees() - b.longitude().to_decimal_degrees())
            .to_radians()
            .cos();
        self.radius * (sa * sb + ca * cb * clong).acos()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Location {
    lat: Coordinate,
    long: Coordinate,
    name: String,
}

impl Location {
    pub fn new(lat: Coordinate, long: Coordinate, name: String) -> Self {
        Self { lat, long, name }
    }

    pub fn from_decimal_degrees(lat: f64, long: f64, name: String) -> Self {
        Self {
            lat: Coordinate::from_decimal_degrees(lat, Hemisphere::for_latitude(lat)),
            long: Coordinate::from_decimal_degrees(long, Hemisphere::for_longitude(long)),
            name,
        }
    }

    pub fn latitude(&self) -> Coordinate {
        self.lat
    }

    pub fn longitude(&self) -> Coordinate {
        self.long
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> String {
        format!("{} {}", self.name, self)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.lat, self.long)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    degrees: u16,
    minutes: u8,
    seconds: f64,
    hemisphere: Hemisphere,
}

impl Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Coordinate", 6)?;
        state.serialize_field("decimal", &self.to_decimal_degrees())?;
        state.serialize_field("dms", &self.to_string())?;
        state.serialize_field("degrees", &self.degrees)?;
        state.serialize_field("minutes", &self.minutes)?;
        state.serialize_field("seconds", &self.seconds)?;
        state.serialize_field("hemisphere", &self.hemisphere.abbreviation())?;
        state.end()
    }
}

impl Coordinate {
    pub fn new(degrees: u16, minutes: u8, seconds: f64, hemisphere: Hemisphere) -> Self {
        Self {
            degrees,
            minutes,
            seconds,
            hemisphere,
        }
    }

    pub fn from_decimal_degrees(degrees: f64, hemisphere: Hemisphere) -> Self {
        // this is kind of broken: degrees field should be 0-180 and hemisphere will depend on this
        let degrees = degrees.abs();
        let (degrees, fraction) = (degrees.trunc() as u16, degrees.fract());
        let minutes = 60.0 * fraction;
        let (minutes, fraction) = (minutes.trunc() as u8, minutes.fract());
        let seconds = 60.0 * fraction;
        Self {
            degrees,
            minutes,
            seconds,
            hemisphere,
        }
    }

    pub fn to_decimal_degrees(self) -> f64 {
        let sign = self.hemisphere.sign();
        sign * (self.degrees as f64 + self.minutes as f64 / 60.0 + self.seconds / 3_600.0)
    }

    pub fn to_decimal_radians(self) -> f64 {
        self.to_decimal_degrees().to_radians()
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{}°{}'{:.4}"{}"#,
            self.degrees, self.minutes, self.seconds, self.hemisphere
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Hemisphere {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Hemisphere {
    pub fn abbreviation(self) -> char {
        match self {
            Self::NORTH => 'N',
            Self::EAST => 'E',
            Self::SOUTH => 'S',
            Self::WEST => 'W',
        }
    }

    pub fn sign(self) -> f64 {
        match self {
            Self::SOUTH | Self::WEST => -1.0,
            _ => 1.0,
        }
    }

    pub fn for_latitude(lat: f64) -> Self {
        if lat < 0.0 {
            Self::SOUTH
        } else {
            Self::NORTH
        }
    }

    pub fn for_longitude(lat: f64) -> Self {
        if lat < 0.0 {
            Self::WEST
        } else {
            Self::EAST
        }
    }
}

impl fmt::Display for Hemisphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}
