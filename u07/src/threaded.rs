use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn waiting_on_workers_with_channels() {
    let completed = {
        let (tx, rx) = mpsc::channel();
        for i in 0..5 {
            let tx = tx.clone();
            thread::spawn(move || super_genius(i, tx));
        }
        rx
    };

    // These two versions are equivalent; both loop will run until the
    // channel is closed.
    if false {
        while let Ok((id, took)) = completed.recv() {
            println!(
                "Super Genius No. {} took {} s to do great things.",
                id, took
            );
        }
    } else {
        for (id, took) in completed {
            println!(
                "Super Genius No. {} took {} s to do great things.",
                id, took
            );
        }
    }
}

fn super_genius(id: i32, completed: mpsc::Sender<(i32, u64)>) {
    println!(
        "Super Genius No. {} making perpetual motion machines and unobtainum alloys in secret waiting for Useless Eaters to die off...",
        id
    );
    let took = busy_for_upto_n_secs(3);
    completed.send((id, took)).unwrap();
}

pub fn waiting_on_workers_with_join_handles() {
    let eaters: Vec<_> = (0..5)
        .map(|i| thread::spawn(move || useless_eater(i)))
        .collect();

    for eater in eaters {
        println!(
            "Useless Eater died after {} s of starvation",
            eater.join().unwrap()
        );
    }
}

fn useless_eater(id: i32) -> u64 {
    println!("Useless Eater No. {} is starving because Super Geniuses are hiding away in Fountainhead and witholding their genius inventions...", id);
    thread::sleep(Duration::from_secs(1));
    busy_for_upto_n_secs(3)
}

fn busy_for_upto_n_secs(n: u64) -> u64 {
    let wait = rand::thread_rng().gen_range(1..=n);
    thread::sleep(Duration::from_secs(wait));
    wait
}
