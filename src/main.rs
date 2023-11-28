use std::{fs, path::Path};

use dir_handling::handle_dir;

mod dir_handling;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 || args.len() > 3 {
        panic!("ERROR: Usage: del <file_name>")
    }

    let target = &args[1];

    match Path::new(target).is_dir() {
        true => {
            handle_dir(target);
        }

        false => {
            let file = fs::read(target).unwrap();

            let mut replaced = vec![];

            for _ in 0..file.len() {
                replaced.push(0);
            }

            let e = fs::write(target, replaced);

            if e.is_err() {
                panic!("ERROR: Couldn't replace file!")
            }
        }
    }


}
