use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct FileModel {
    name: String,
    data: Vec<u8>,
}

fn main() {
    file_struct();
    print_divider();

    file_read();
}

fn file_read() {
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

// a function never returns
// fn dead_end() -> ! {
//     panic!("dead end!");
// }

fn file_struct() {
    let f1 = FileModel {
        name: String::from("file1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_len = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_len);

    print_divider();

    let mut f2 = FileModel {
        name: String::from("file2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };
    let mut buffer: Vec<u8> = vec![];
    open(&mut f2);
    let f2_len = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f2);
    println!("{} is {} bytes long", f2.name, f2_len);
    println!("{} text is: {}", f2.name, text);
}

fn open(f: &mut FileModel) -> bool {
    true
}

fn close(f: &mut FileModel) -> bool {
    true
}

fn read(f: &FileModel, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let len = tmp.len();

    save_to.reserve(len);
    save_to.append(&mut tmp);

    len
}

fn print_divider() {
    println!("============================================");
}
