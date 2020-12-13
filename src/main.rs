mod tools;
use std::cmp::Ordering;
//use tools::{random, read_line};

fn guessing_game(target: u32, read_guessing: &dyn Fn(&mut String)) {
    println!("Devinez le nombre ;-)");

    let mut found = false;

    while !found {
        let mut guess = String::new();
        println!("Saisissez votre idée :");
        read_guessing(&mut guess);
        println!("Votre choix : {}", guess);
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Il faut saisir un nombre entre 1 et 101 !");
        match guess.cmp(&target) {
            Ordering::Less => println!("Trop petit"),
            Ordering::Greater => println!("Trop grand"),
            Ordering::Equal => {
                println!("Victoire !");
                found = true;
            }
        }
    }
}

fn main() {
    let mut stop = false;
    let mut target: u32 = tools::random(1, 101);

    while !stop {
        guessing_game(target, &tools::read_line);
        
        let mut want_to_stop = String::new();
        println!("Voulez vous rejouer ? (y/N)");
        tools::read_line(&mut want_to_stop);
        match want_to_stop.trim() {
            "y" => {
                stop = false;
                println!();
                println!("------------");
                println!();
                target = tools::random(1, 101);
            }
            _ => stop = true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn assert_that_random_function_work() {
        let max = 1000000;
        for iteration_count in 0..max {
            println!("iteration : {}/{}", iteration_count, max);
            let generated = tools::random(1, 101);
            assert!(generated > 0, "generated number is negative");
            assert!(generated < 101, "generated number is to high");
        }
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        panic!("Make this test fail");
    }
}
