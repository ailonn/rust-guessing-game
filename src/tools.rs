use rand::Rng;
use std::io;

pub fn read_line(mut line: &mut String) {
    io::stdin()
        .read_line(&mut line)
        .expect("Échoue à lire la ligne.");
}

pub fn random(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min, max)
}
