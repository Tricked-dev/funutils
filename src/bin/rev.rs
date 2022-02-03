use std::io;

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        println!("{}", buffer.chars().rev().collect::<String>().trim())
    }
}
