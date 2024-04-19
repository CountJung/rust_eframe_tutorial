use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Tutorial1 {
    running: bool,
}

impl Tutorial1 {
    //initializer
    pub fn new() -> Self {
        Self { running: true }
    }
    pub fn string_inout_test(&self) {
        let running = self.running;
        println!("is running? = {running}");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();

        loop {
            println!("Input Number 1 ~ 100");
            guess.clear();
            io::stdin()
                .read_line(&mut guess)
                .expect("Fail to read line");
            let guess_num: u32 = match guess.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Not Num");
                    continue;
                }
            };
            println!("Input Text = {guess} ");

            match guess_num.cmp(&secret_number) {
                Ordering::Less => {
                    println!("small");
                }
                Ordering::Greater => {
                    println!("greater");
                }
                Ordering::Equal => {
                    println!("correct, RandomNum = {secret_number}");
                    break;
                }
            }
        }
    }
}
