use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::{count_characters, count_lines, count_words};

    #[test]
    fn count_lines_works_as_expected() {
        let mut lines = Vec::new();
        lines.push(String::from("first line"));
        lines.push(String::from("second line"));
        lines.push(String::from("third line"));

        assert_eq!(3, count_lines(lines));
    }

    #[test]
    fn count_words_works_as_expected() {
        let mut lines = Vec::new();
        lines.push(String::from("one word"));
        lines.push(String::from("another word"));
        lines.push(String::from("yet another word"));

        assert_eq!(7, count_words(lines));
    }

    #[test]
    fn count_characters_works_as_expected() {
        let mut lines = Vec::new();
        lines.push(String::from("first set of characters"));
        lines.push(String::from("another group of characters"));
        lines.push(String::from("final characters"));

        assert_eq!(69, count_characters(lines));
    }
}

fn main() {
    let arg_type = std::env::args().nth(1);
    let file_name = std::env::args().nth(2);
    let input = file_name.unwrap_or_else(|| String::from(""));

    let mut all_lines = Vec::new();

    if input != *""{
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

    let result:u32;

    match arg_type.unwrap_or("".parse().unwrap()).as_str() {
        "-l" => result = count_lines(all_lines),
        "--lines" => result = count_lines(all_lines),
        "-c" => result = count_characters(all_lines),
        "--characters" => result = count_characters(all_lines),
        "-w" => result = count_words(all_lines),
        "--words" => result = count_words(all_lines),
        _ => {
            result = count_lines(all_lines.clone());
            let result2 = count_words(all_lines.clone());
            let result3 = count_characters(all_lines);

            println!("\t{} \t{} \t{}", result, result2, result3);
            return
        }
    }

    println!("{}", result);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
    where T: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_lines(all_lines: Vec<String>) -> u32 {
    let mut num_of_lines:u32 = 0;

    for _line in all_lines {
        num_of_lines += 1;
    }

    num_of_lines
}

fn count_characters(all_lines: Vec<String>) -> u32 {
    let mut num_of_chars:u32 = 0;

    for line in all_lines {
        num_of_chars += line.chars().count() as u32 + 1;
    }

    num_of_chars
}

fn count_words(all_lines: Vec<String>) -> u32{
    let mut num_of_words:u32 = 0;

    for line in all_lines {
        num_of_words += line.
            split(' ').
            filter(|w| !w.eq_ignore_ascii_case("")).
            count() as u32
    }

    num_of_words
}