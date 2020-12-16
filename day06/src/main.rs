use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    //let i = first(&mut reader);
    let i = testSec();
    let j = second(&mut reader);
    println!("Part 1: {}", i);
    println!("Part 2: {}", j);

    Ok(())
}

fn first(input: &mut BufReader<File>) -> i32 {
    let mut count = 0;
    let mut questions: [bool;  26] = [false; 26];
    for line in input.lines() {
        let l = line.unwrap();
        if !l.is_empty() {
            for c in l.chars() {
                questions[(c as u32 - 97) as usize] = true;
            }
        } else {
            //count of true in question map
            for i in 0..questions.len() {
                if questions[i] {
                    count += 1;
                }
                questions[i] = false;
            }
        }
    }

    count
}

//TODO
fn second(input: &mut BufReader<File>) -> i32 {
    let mut count = 0;//total to return
    let mut groupCount = 0;//people in the group
    let mut questions = Vec::new();
    for line in input.lines() {
        let l = line.unwrap();
        if !l.is_empty() {
            groupCount += 1;
            for c in l.chars() {
                questions.push(c);
            }
        } else {
            //when blank row hit, count unique chars. For each charChount == groupCount, increment
            //total count.
            for c in (b'a'..=b'z').map(|c| c as char) {
                if questions.iter().filter(|&ch| *ch == c).count() == groupCount {
                    count += 1;
                }
            }
            //reset for new group
            groupCount = 0;
            questions.truncate(0);
        }
    }
    count as i32
}

fn testSec() -> i32 {
    let mut count = 0;
    let groupCount = 3;
    let questions = vec!['a', 'a', 'b', 'a', 'b', 'c', 'j', 'j', 'j', 'z', 'z', 'z'];
    for c in (b'a'..=b'z').map(|c| c as char) {
        if questions.iter().filter(|&ch| *ch == c).count() == groupCount {
            count += 1;
        }
    }
    count as i32
}
