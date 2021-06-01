use std::collections::{HashMap, HashSet};

use crate::floaten::Hf64;

pub fn word_counts() {
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    for word in load_words() {
        *(word_counts.entry(word).or_default()) += 1;
    }
    let word_counts = {
        let mut word_counts = word_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .collect::<Vec<_>>();
        word_counts.sort_unstable_by_key(|(_, count)| *count);
        word_counts
    };
    for (word, count) in word_counts {
        println!("{:15}: {:2}", word, count);
    }
}

fn load_words() -> impl Iterator<Item = String> {
    const TEXT: &str = r#"
    My reconcilement to the yahoo kind in general might not be so difficult, 
    if they would be content with those vices and follies only, which nature 
    has entitled them to. I am not in the least provoked at the sight of a lawyer,
    a pickpocket, a colonel, a fool, a lord, a gamester, a politician, a
    physician, an evidence, a suborner, an attorney, a traitor, or the like;
    this is all according to the due course of things: but when I behold a lump
    of deformity and diseases, both in body and mind, smitten with pride, it
    immediately breaks all the measures of my patience; neither shall I be ever
    able to comprehend how such an animal, and such a vice, could tally together.
    The wise and virtuous Houyhnhnms, who abound in all the excellencies that
    can adorn a rational creature, have no name for this vice in their language;
    which has no terms to express anything that is evil, except those whereby
    they describe the detestable qualities of their yahoos; among which they
    were not able to distinguish this of pride for want of thoroughly understanding
    human nature, as it shows itself in other countries where that animal presides.
    But I, who had more experience, could plainly observe some rudiments of it
    among the wild yahoos.
    "#;
    // Gulliver's Travels, by Jonathan Swift
    TEXT.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|c| c.to_lowercase())
}

pub fn exploring_sets() {
    let temps: Vec<Hf64> = [-28.0f64, 32.0, -31.0, -29.0, -23.0, -29.0, -28.0, -33.0]
        .iter()
        .map(|f| Hf64::new(*f))
        .collect();
    let mut uniq = HashSet::new();
    for temp in temps {
        uniq.insert(temp);
    }

    println!("{:?}", uniq);
}

pub fn exploring_frequencies() {
    let mut freqs: HashMap<Hf64, i32> = HashMap::new();
    let temps: Vec<Hf64> = [-28.0f64, 32.0, -31.0, -29.0, -23.0, -29.0, -28.0, -33.0]
        .iter()
        .map(|f| Hf64::new(*f))
        .collect();
    for temp in temps {
        // freqs[temp] += 1
        *(freqs.entry(temp).or_default()) += 1;
    }
    for (k, v) in freqs {
        println!("I saw {:+5.2} {} times", k.as_f64(), v);
    }

    let mut groups: HashMap<i64, Vec<f64>> = HashMap::new();

    let temps: &[f64] = &[-28.0f64, 32.0, -31.0, -29.0, -23.0, -29.0, -28.0, -33.0];
    for temp in temps.iter().copied() {
        let group = (temp / 10.0).trunc() as i64 * 10;
        groups.entry(group).or_default().push(temp);
    }
    for (k, v) in groups {
        println!("{}: {:?}", k, v);
    }
}

pub fn exploring_maps() {
    let mut moo: HashMap<&str, i32> = HashMap::new();
    moo.insert("moon", 42);
    moo.insert("noon", 371);

    println!("{:?}", moo.get("moon"));
    println!("{:?}", moo.get("goon"));
    println!("{}", *moo.get("goon").unwrap_or(&Default::default()));

    if let Some(points) = moo.get("moon") {
        println!("Has {} points.", points);
    } else {
        println!("Skipped the game.");
    }

    println!("Before delete: {:?}", moo);
    // delete(moo, "noon")
    moo.remove("noon");
    println!("After delete: {:?}", moo);

    // oons := map[string]int{"moon": 96, "soon": 81, "loon": 62}
    let oons = maplit::hashmap! {
        "moon" => 96,
        "soon" => 81,
        "loon" => 62,
    };
    println!("From literal: {:?}", oons);
}
