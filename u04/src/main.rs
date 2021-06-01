use std::io;
use u04::conway;

fn main() -> io::Result<()> {
    let mut sim = conway::random(80, 20);

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(300));
        sim.next();
        println!("{}", sim.universe());
    }
    Ok(())
}
