use rust_embed::RustEmbed;
use std::io;
use std::io::{stdout, Write};
use std::{thread, time};

#[derive(RustEmbed)]
#[folder = "src/parrots/"]
#[exclude = "*.txt"]
struct Assets;

fn list_cows() -> Vec<String> {
    Assets::iter()
        .map(|x| x.split("/").last().unwrap().replace(".png.txt", ""))
        .collect::<Vec<String>>()
}

fn main() {
    let parrots = Assets::iter()
        .map(|x| {
            std::str::from_utf8(&Assets::get(&x).unwrap().data)
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<String>>();
    loop {
        for parrot in &parrots {
            print!("{}", parrot);
            thread::sleep(time::Duration::from_millis(60));
        }
    }
}
