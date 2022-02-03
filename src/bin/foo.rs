const QUESTIONS: [&str; 10] = [
    "Do you want to do this",
    "Do you really want to do this",
    "Are you sure you want to do this",
    "Are you really sure you want to do this",
    "Do you still want to do this",
    "You dont have to do this want to do this anyway",
    "Are you positive you want to do this",
    "Are you really positive you want to do this",
    "Are you very sure you want to do this",
    "You are 100% sure you want to do this",
];
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::process;
fn main() {
    let mut last = String::new();
    fn get_random_question() -> String {
        let mut rng = thread_rng();
        QUESTIONS
            .choose(&mut rng)
            .expect("Failed to pick a question")
            .to_string()
    }
    loop {
        //Randomize the answer so the last one cant appear twice
        loop {
            let res = get_random_question();
            if res == last {
                continue;
            } else {
                last = res;
                break;
            }
        }

        let reply = rprompt::prompt_reply_stdout(&format!("{}? (y/n): ", last))
            .expect("Failed to read prompt");
        if reply.as_str() == "n" {
            process::exit(0x0100);
        }
    }
}
