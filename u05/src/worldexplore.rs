use serde::Serialize;
use std::io::{self, Write};
use u05::world::{self, Coordinate, Gps, Hemisphere, Location, Rover};

pub fn explore_customization_interfaces() {
    let bradbury = Location::new(
        Coordinate::new(4, 35, 22.2, Hemisphere::SOUTH),
        Coordinate::new(137, 126, 30.1, Hemisphere::EAST),
        "Bradbury Landing".to_owned(),
    );
    serde_json::to_writer(std::io::stdout(), &bradbury).unwrap();
    println!();
    println!("{}", serde_json::to_string_pretty(&bradbury).unwrap());
    println!();
}

pub fn explore_struct_composition() {
    let bradbury = Location::new(
        Coordinate::new(4, 35, 22.2, Hemisphere::SOUTH),
        Coordinate::new(137, 126, 30.1, Hemisphere::EAST),
        "Bradbury Landing".to_owned(),
    );
    let elysium = Location::new(
        Coordinate::new(4, 30, 0.0, Hemisphere::NORTH),
        Coordinate::new(135, 54, 0.0, Hemisphere::EAST),
        "Elysium Planitia".to_owned(),
    );
    let curiosity = Rover::new(
        "Curiosity".to_owned(),
        Gps::new(world::mars(), bradbury, elysium),
    );
    println!("{}", curiosity.message());
}

pub fn explore_struct_methods() {
    let lat = Coordinate::new(4, 35, 22.2, Hemisphere::SOUTH);
    let long = Coordinate::new(137, 26, 30.12, Hemisphere::EAST);
    println!(
        "({}, {}) aka ({:.6}, {:.6})",
        lat,
        long,
        lat.to_decimal_degrees(),
        long.to_decimal_degrees()
    );
    let nlat = Coordinate::from_decimal_degrees(
        lat.to_decimal_degrees(),
        Hemisphere::for_latitude(lat.to_decimal_degrees()),
    );
    println!("Now: {}", nlat);

    let mars = world::mars();
    let spirit = Location::from_decimal_degrees(
        -14.5684,
        175.472636,
        "Columbia Memorial Station".to_owned(),
    );
    let opportunity =
        Location::from_decimal_degrees(-1.9462, 354.4734, "Challenger Memorial Station".to_owned());

    let dist = mars.distance(&spirit, &opportunity);
    println!("Spirit and Opportunity are {:.2} km apart.", dist);

    let curiosity = Location::new(
        Coordinate::new(4, 35, 22.2, Hemisphere::SOUTH),
        Coordinate::new(137, 126, 30.1, Hemisphere::EAST),
        "Bradbury Landing".to_owned(),
    );
    let insight = Location::new(
        Coordinate::new(4, 30, 0.0, Hemisphere::NORTH),
        Coordinate::new(135, 54, 0.0, Hemisphere::EAST),
        "Elysium Planitia".to_owned(),
    );
    let bots = [
        ("spirit", spirit),
        ("opportunity", opportunity),
        ("curiosity", curiosity),
        ("insight", insight),
    ];
    let mut distances = Vec::new();
    for (i, (bxn, bxl)) in bots.iter().enumerate() {
        for (byn, byl) in bots.iter().skip(i + 1) {
            if bxn != byn {
                distances.push((bxn, byn, mars.distance(bxl, byl)))
            }
        }
    }
    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());
    for (x, y, dist) in distances {
        println!("{:15} {:15} {:6.1} km ", x, y, dist);
    }

    let earth = world::earth();
    let london = Location::new(
        Coordinate::new(51, 30, 0.0, Hemisphere::NORTH),
        Coordinate::new(0, 8, 0.0, Hemisphere::WEST),
        "London".to_owned(),
    );
    let paris = Location::new(
        Coordinate::new(48, 51, 0.0, Hemisphere::NORTH),
        Coordinate::new(2, 21, 0.0, Hemisphere::EAST),
        "Paris".to_owned(),
    );
    println!(
        "{} is {:.1} km from {}.",
        london.name(),
        earth.distance(&london, &paris),
        paris.name()
    );
}

pub fn explore_struct_json_serialization() {
    #[derive(Debug, Serialize)]
    struct Location {
        lat: f64,
        // rename is like struct tags `json:"bingo"`
        #[serde(rename = "bingo")]
        long: f64,
    }

    let loc = Location {
        lat: 72.5,
        long: 99.3,
    };
    // string(json.MarshalIndent())
    println!(
        "To pretty printed string: {}",
        serde_json::to_string_pretty(&loc).unwrap()
    );

    // json.Marshall()
    println!("To bytes:");
    match serde_json::to_vec(&loc) {
        Ok(data) => io::stdout().write_all(&data).unwrap(),
        Err(err) => eprintln!("{}", err),
    }
    println!()
}
