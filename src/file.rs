use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
pub fn search_file() -> bool {
    let b = Path::new("file.txt").is_file();
    return b;
}
pub fn create_file() {
    File::create("file.txt").expect("creation failed");
}
pub fn write_file(data: &Vec<String>, append: bool) {
    if !search_file() {
        create_file();
    }
    if append {
        let file_toread = File::open("file.txt").unwrap();
        let reader = BufReader::new(file_toread);
        let no = reader
            .lines()
            .last()
            .map(|last_line| {
                last_line
                    .expect("IO error")
                    .split_once(' ')
                    .expect("Last line has no space characters")
                    .0
                    .trim_start()
                    .parse::<u64>()
                    .expect("Last line does not start with a number")
            })
            .unwrap_or(0);
        for i in 0..data.len() {
            let x = i as u64;
            let mut file_towrite = OpenOptions::new().append(true).open("file.txt");
            writeln!(
                file_towrite.expect("error writing"),
                "{} {}",
                x + no + 1,
                data[i]
            )
            .expect("File is corrupted");
        }
    } else {
        let mut file = File::create("file.txt");
        for i in 0..data.len() {
            let x = i as u64;
            let mut file_towrite = OpenOptions::new().append(true).open("file.txt");
            writeln!(file_towrite.expect("error writing"), "{} {}", x, data[i])
                .expect("File is corrupted");
        }
    }
}

pub fn show_data() {
    let file_toread = File::open("file.txt").unwrap();
    let reader = BufReader::new(file_toread);
    for lines in reader.lines() {
        println!("{:?}", lines.unwrap());
    }
}

pub fn fetch_data(data: &mut Vec<String>) {
    if (!search_file()) {
        println!("Local file doesn't exists");
    } else {
        let file_toread = File::open("file.txt").expect("No file created");
        let reader = BufReader::new(file_toread);
        data.clear();
        data.extend(reader.lines().map(|line| {
            line.expect("IO error")
                .split_once(' ')
                .expect("Last line has no space characters")
                .1
                .trim_end()
                .to_owned()
        }));
    }
}
