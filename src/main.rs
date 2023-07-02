use std::io;

fn main() {
    let arg_type = std::env::args().nth(1).expect("no type was specified");

    match arg_type.as_str() {
        "-l" => count_lines(),
        "--lines" => count_lines(),
        "-c" => count_characters(),
        "--characters" => count_characters(),
        "-w" => count_words(),
        "--words" => count_words(),
        _ => {}
    }
}

fn count_lines(){
    let mut num_of_lines = 0;
    let lines = io::stdin().lines();

    for _line in lines {
        num_of_lines += 1;
    }

    println!("{}", num_of_lines);
}

fn count_characters(){
    let mut num_of_chars = 0;
    let lines = io::stdin().lines();

    for line in lines {
        num_of_chars += line.unwrap().as_str().chars().count() + 1;
    }

    println!("{}", num_of_chars);
}

fn count_words(){
    let mut num_of_words = 0;
    let lines = io::stdin().lines();

    for line in lines {
        num_of_words += line.
            unwrap().
            as_str().
            split(" ").
            filter(|w| (!w.eq_ignore_ascii_case(""))).
            count()
    }

    println!("{}", num_of_words);
}
