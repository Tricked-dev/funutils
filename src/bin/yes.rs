use clap::{App, Arg};
use std::io;

fn main() {
    let matches = App::new("yes")
        .version("v0.1.0")
        .author("Tricked-dev <tricked@tricked.pro>")
        .arg(Arg::new("MESSAGE").help("yes").multiple_values(true))
        .get_matches();
    let val = matches.values_of("MESSAGE");
    let text = if let Some(val) = val {
        val.collect::<Vec<&str>>().join(" ")
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        buffer.trim().to_owned()
    };

    loop {
        println!("{text}")
    }
}
