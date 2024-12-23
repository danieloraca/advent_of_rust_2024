use std::collections::HashMap;

pub struct SantaList {
    records: HashMap<String, bool>,
}

impl SantaList {
    pub fn new() -> Self {
        Self {
            records: Default::default(),
        }
    }

    pub fn add(&mut self, name: &str, behavior: bool) {
        self.records.insert(name.into(), behavior);
    }

    pub fn remove(&mut self, name: &str) {
        self.records.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<bool> {
        self.records.get(name).and_then(|v| Some(v.clone()))
    }

    pub fn count(&self) -> (u16, u16) {
        self.records
            .iter()
            .fold((0, 0), |(nice, naughty), (_, behavior)| {
                if *behavior {
                    (nice + 1, naughty)
                } else {
                    (nice, naughty + 1)
                }
            })
    }

    pub fn list_by_behavior(&self, behavior: bool) -> Vec<String> {
        self.records
            .iter()
            .filter_map(|(k, v)| (*v == behavior).then_some(k.clone()))
            .collect()
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();
    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);
    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }
    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);
    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);
    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
