use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("../README.md");
    match f {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line = String::new();

            // 1: read by line
            loop {
                let len = reader.read_line(&mut line);
                match len {
                    Ok(size) => {
                        if size == 0 {
                            println!("\n EOF.");
                            break;
                        }

                        println!("{} ({} bytes long)", line, size);

                        // Shrinks the String back to length 0
                        // preventing lines from persisting into the following ones.
                        line.truncate(0);
                    }
                    Err(err) => {
                        println!("read line error: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            println!("open file error: {}", err);
            return;
        }
    }
}
