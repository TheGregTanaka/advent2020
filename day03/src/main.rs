use std::env;
use std::io::{self, Read, BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    first(&mut reader);
    reader.seek(io::SeekFrom::Start(0));
    second(&mut reader, 1, false);
    reader.seek(io::SeekFrom::Start(0));
    second(&mut reader, 3, false);
    reader.seek(io::SeekFrom::Start(0));
    second(&mut reader, 5, false);
    reader.seek(io::SeekFrom::Start(0));
    second(&mut reader, 7, false);
    reader.seek(io::SeekFrom::Start(0));
    second(&mut reader, 1, true);
    Ok(())
}

fn first(input: &mut BufReader<File>) -> Result<()> {
    let mut pos = 0;
    let mut sled;
    let mut trees = 0;
    for line in input.lines() {
        let l = line.unwrap();
        let wrap = l.len();
        pos = pos%wrap;
        sled = l.chars().nth(pos).unwrap();
        if sled == '#' {
            //println!("crash");
            trees += 1;
        } else {
            //println!("wee");
        }

        pos += 3;
    }
    println!("hit {} trees", trees);

    Ok(())
}

fn second(input: &mut BufReader<File>, r: usize, skip: bool) -> Result<()> {
    let mut pos = 0;
    let mut sled;
    let mut trees = 0;
    let mut lineCount = 0;
    for line in input.lines() {
        if !skip || lineCount%2 == 0 {
            let l = line.unwrap();
            let wrap = l.len();
            pos = pos%wrap;
            sled = l.chars().nth(pos).unwrap();
            if sled == '#' {
                trees += 1;
            }

            pos += r;
        }
        lineCount += 1;
    }
    println!("hit {} trees", trees);

    Ok(())
}
