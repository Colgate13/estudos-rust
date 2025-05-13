use std::env;
use std::fs::File;
use std::io::Read;

struct FileAttach {
    file: File,
    file_contents: String,
}

fn main() {
    // [1] => Search Term, [2] => File
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Mini grep need two arguments");
    }

    let query_term = &args[1];
    let file_name = &args[2];

    let mut file = File::open(file_name)
        .expect("File not found");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Something went wrong in reading the file");

    let file_attach: FileAttach = FileAttach { 
        file,
        file_contents
    };

    let mut lines_match: Vec<String> = vec![];
    for line in file_attach.file_contents.lines() {
        match line.find(query_term) {
            Some(_) => lines_match.push(String::from(line)),
            _ => {continue;}
        }
    }

    for matches in lines_match {
        println!("{matches}");
    }
}
