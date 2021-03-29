use std::env;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    if args.len() > 1 {
        filename = String::from(&args[1]);
    } else {
        filename = String::from("input/_test.txt");
        println!("No filename given, defaulting to {}", filename);
    }
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    first(&mut reader);
    f.seek(SeekFrom::Start(0))?;

    Ok(())
}

fn first(reader: &mut BufReader<File>) -> i32 {

    1
}

fn second(reader: &mut BufReader<File>) -> i32 {
    1
}
