use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..2 {
        let n: u32 = rng.gen_range(1..=10);
        println!("Hello biological unit No. {}", n);
    }

    for _ in 0..3 {
        println!("On {}, something happened.", random_date(&mut rng));
    }

    let chris = "Christopher Walken";
    if chris.contains("Walk") {
        println!("Ze Walken Dead.");
    }

    mars_tickets(&mut rng);
}

fn mars_tickets(rng: &mut impl Rng) {
    const LINES: [&str; 3] = ["SpaceX", "Virgin Galactic", "Space Adventures"];
    const ONE_WAY: &str = "One-way";
    const ROUND_TRIP: &str = "Round-trip";
    println!("Spaceline         Days Trip type   Price");
    println!("========================================");
    const DIST: f64 = 62_100_000.0; // km
    const SECS_PER_DAY: f64 = 24.0 * 60.0 * 60.0; // s
    for _ in 0..10 {
        let spaceline = LINES[rng.gen_range(0..LINES.len())];
        let trip_type = if rng.gen() { ROUND_TRIP } else { ONE_WAY };
        let speed = rng.gen_range(16..=30) as f64; // km/s
        let price = 36.0 + (speed - 16.0); // USD
        let one_way_days = (DIST / speed / SECS_PER_DAY).ceil() as i32;
        let days = match trip_type {
            ONE_WAY => one_way_days,
            ROUND_TRIP => 2 * one_way_days,
            _ => unreachable!(),
        };
        println!(
            "{:17} {:4} {:11} ${:4.0}",
            spaceline, days, trip_type, price
        );
    }
}

fn random_date(rng: &mut impl Rng) -> String {
    let year = rng.gen_range(1970..=2070);
    let month = rng.gen_range(1..=12);
    let day = rng.gen_range(1..=days_in_month(year, month));
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}
