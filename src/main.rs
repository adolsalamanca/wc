use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let arg_type = std::env::args().nth(1).expect("no type was specified");
    let file_name = std::env::args().nth(2);
    let input = file_name.unwrap_or_else(|| String::from(""));

    let mut all_lines = Vec::new();

    if input != String::from(""){
        if let Ok(lines) = read_lines(input) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ref _content) = line {
                    all_lines.push(line.unwrap_or_else(|_| String::from("")));
                }
            }
        }
    } else {
        let lines = io::stdin().lines();
        for line in lines {
            all_lines.push(line.unwrap_or_else(|_| String::from("")));
        }
    }

    match arg_type.as_str() {
        "-l" => count_lines(all_lines),
        "--lines" => count_lines(all_lines),
        "-c" => count_characters(all_lines),
        "--characters" => count_characters(all_lines),
        "-w" => count_words(all_lines),
        "--words" => count_words(all_lines),
        _ => {}
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_lines(all_lines: Vec<String>){
    let mut num_of_lines = 0;

    for _line in all_lines {
        num_of_lines += 1;
    }

    println!("{}", num_of_lines);
}

fn count_characters(all_lines: Vec<String>){
    let mut num_of_chars = 0;

    for line in all_lines {
        num_of_chars += line.chars().count() + 1;
    }

    println!("{}", num_of_chars);
}

fn count_words(all_lines: Vec<String>){
    let mut num_of_words = 0;

    for line in all_lines {
        num_of_words += line.
            split(" ").
            filter(|w| (!w.eq_ignore_ascii_case(""))).
            count()
    }

    println!("{}", num_of_words);
}