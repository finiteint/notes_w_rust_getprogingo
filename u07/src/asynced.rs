use async_std::io::prelude::WriteExt;
use rand::Rng;
use std::time::Duration;

use async_std::channel::{self, Receiver, Sender};
use async_std::io;
use async_std::task;

#[derive(Debug)]
pub enum Message {
    Fact(String),
    Stop,
}

pub async fn facts_pipeline() {
    // Note how the sending and receiving parts of  channel has to be
    // split (out of necessity) in the Rust version.
    //
    // In Go the same channel instance can be shared by all parties involved
    // using either end of the channel; some parties may only see (choose to see)
    // a send (`out chan<- T`) or receive (`in <-chan T`).
    //
    // In Rust the channel parts are transferred or cloned (if it needs to be shared).
    let (facts_tx, facts_rx) = channel::bounded(1);
    let (true_facts_tx, true_facts_rx) = channel::bounded(1);
    task::spawn(fact_digger(facts_tx));
    task::spawn(minitrue(facts_rx, true_facts_tx));
    task::block_on(fact_poster(true_facts_rx));
}

async fn fact_digger(output: Sender<Message>) {
    for &fact in &[
        "we probably, maybe, should take climate change a bit more seriously.",
        "rainbows and unicorns everwhere. consume! consume!",
        "study finds tobacco can cause lung cancer",
        "being selfish is the greatest form of charity",
        "study conducted by AZR Oil Co. finds that climate inaction is the best action",
    ] {
        busy_for_upto_n_secs(3).await;
        output.send(Message::Fact(fact.into())).await.unwrap();
    }
    output.send(Message::Stop).await.unwrap();
}

async fn minitrue(input: Receiver<Message>, output: Sender<Message>) {
    fn to_alt_fact(mut fact: String) -> String {
        if fact.contains("climate change") {
            fact.push_str(" Climate change is not real; if it is real it is not man made; the best action is inaction anyway.");
        }
        fact
    }

    fn affects_profit(fact: &str) -> bool {
        fact.contains("tobacco can cause")
    }

    while let Ok(msg) = input.recv().await {
        match msg {
            Message::Fact(fact) => {
                if !affects_profit(&fact) {
                    output.send(Message::Fact(to_alt_fact(fact))).await.unwrap();
                }
            }
            Message::Stop => {
                output.send(Message::Stop).await.unwrap();
                break;
            }
        }
    }
}

async fn fact_poster(facts: Receiver<Message>) {
    while let Ok(msg) = facts.recv().await {
        match msg {
            Message::Fact(fact) => io::stdout()
                .write_fmt(format_args!("FACT!: {}\n\n", fact))
                .await
                .unwrap(),
            Message::Stop => break,
        }
    }
}

pub async fn waiting_on_workers_with_channels() {
    let completed = {
        let (tx, rx) = channel::bounded(5);
        for i in 0..5 {
            let tx = tx.clone();
            // `task::spawn` is roughly equivalent to Go's `go` statement.
            task::spawn(async move { super_genius(i, tx).await });
        }
        rx
    };
    // In Go receiving on closed channels will always returnt the zero value
    // and the closed state can be checked with the `value, ok` idiom:
    // ```
    //   v, ok := <- ch
    //   if ok { ... }
    // ```
    // In Rust, `recv()` returns a `Result` that will be an `Err` if
    // the channel is closed and there are no messages to return.
    // Rust channels are automatically closed when all the senders
    // have gone out of scoped (dropped).
    //
    // Go's `for` `range` loop provides a way to keep receving from a channel
    // until it's closed.
    // ```
    // for v := range ch { ... }
    // ```
    // In Rust a `while let` is required for the async version.
    // For the non-async version a for loop can be used instead.
    // It may be possible to do this if `async for` loops feature
    // is added.
    while let Ok((id, took)) = completed.recv().await {
        println!(
            "Super Genius No. {} took {} s to do great things.",
            id, took
        );
    }
}

async fn super_genius(id: i32, completed: Sender<(i32, u64)>) {
    println!(
        "Super Genius No. {} making perpetual motion machines and unobtainum alloys in secret waiting for Useless Eaters to die off...",
        id
    );
    task::sleep(Duration::from_secs(1)).await;
    let took = busy_for_upto_n_secs(3).await;
    completed.send((id, took)).await.unwrap();
}

pub async fn waiting_on_workers_with_join_handles() {
    let eaters: Vec<_> = (0..5)
        .map(|i| task::spawn(async move { useless_eater(i).await }))
        .collect();

    for eater in eaters {
        println!("Useless Eater died after {} s of starvation", eater.await);
    }
}

async fn useless_eater(id: i32) -> u64 {
    println!(
        "Useless Eater No. {} is starving because Super Geniuses are hiding away in Fountainhead and witholding their genius inventions...",
        id
    );
    busy_for_upto_n_secs(3).await
}

async fn busy_for_upto_n_secs(n: u64) -> u64 {
    let wait = rand::thread_rng().gen_range(1..=n);
    task::sleep(Duration::from_secs(wait)).await;
    wait
}
