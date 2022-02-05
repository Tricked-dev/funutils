use rand::{prelude::IteratorRandom, thread_rng};
use rust_embed::RustEmbed;
use std::{
    collections::HashSet,
    io::{stdout, Write},
    thread, time,
};

#[derive(RustEmbed)]
#[folder = "src/parrots/"]
#[exclude = "*.par"]
struct Assets;

fn get_parrots() -> HashSet<String> {
    Assets::iter()
        .map(|x| x.to_owned().split("/").next().unwrap().to_owned())
        .collect::<_>()
}

fn main() {
    // let parrots = ;
    let mut rng = thread_rng();
    let parrot = get_parrots()
        .into_iter()
        .choose(&mut rng)
        .expect("Failed to pick a parrot")
        .to_string();
    let parrots = Assets::iter()
        .filter(|parr| parr.contains(&format!("{}/", parrot)))
        .map(|x| {
            std::str::from_utf8(&Assets::get(&x).unwrap().data)
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<String>>();
    loop {
        for parrot in &parrots {
            stdout().write_all(parrot.as_bytes()).unwrap();
            thread::sleep(time::Duration::from_millis(90));
            stdout().flush().unwrap();
            #[cfg(not(target_os = "windows"))]
            print!("\x1B[2J\x1B[1;1H");
        }
    }
}
