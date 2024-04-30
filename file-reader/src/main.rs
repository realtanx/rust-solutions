use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("../README.md");
    match f {
        Ok(file) => {
            let reader = BufReader::new(file);

            // 1: read by line
            // let mut line = String::new();
            // loop {
            //     let len = reader.read_line(&mut line);
            //     match len {
            //         Ok(size) => {
            //             if size == 0 {
            //                 println!("\n EOF.");
            //                 break;
            //             }

            //             println!("{} ({} bytes long)", line, size);

            //             // Shrinks the String back to length 0
            //             // preventing lines from persisting into the following ones.
            //             line.truncate(0);
            //         }
            //         Err(err) => {
            //             println!("read line error: {}", err);
            //             break;
            //         }
            //     }
            // }

            // 2: read by lines
            for line in reader.lines() {
                match line {
                    Ok(s) => {
                        println!("{} ({} bytes long)", s, s.len());
                    }
                    Err(e) => {
                        println!("read line error: {}", e);
                    }
                }
            }
            println!("\n EOF.");
        }
        Err(err) => {
            println!("open file error: {}", err);
            return;
        }
    }
}
