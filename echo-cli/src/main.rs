use clap::{App, Arg};

fn main() {
    // println!("{:?}", std::env::args());

    let arg_text = "text";
    let arg_omit_newline = "omit_newline";

    let matches = App::new("echo-cli")
        .version("0.1.0")
        .author("@realtanx")
        .about("Rust version of CLI echo.")
        .arg(
            Arg::with_name(arg_text)
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name(arg_omit_newline)
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", matches);

    let text = matches.values_of_lossy(arg_text).unwrap();
    let omit_newline = matches.is_present(arg_omit_newline);

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);

    // cargo run hello world
    // cargo run hello world -n
}
