extern crate regex;
use std::env;
use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;
use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    

    //first(reader);
    second(reader);

    Ok(())
}

fn first(input: BufReader<File>) -> Result<()> {
    let mut validCount = 0;
    for line in input.lines() {
        let l = line.unwrap();
        //break each line into 3 parts, policy, required char, password
        //this is hacky, but it was the first way I thought to do this
        let mut i = 0usize;
        let mut policy: Vec<String> = Vec::new();
        let mut reqchr: Vec<String> = Vec::new();
        let mut passwd: Vec<String> = Vec::new();
        for word in l.split_whitespace() {
            //since we know each line has the same format, we can just match on field position
            match i % 3 {
                0 => {
                    policy.push(word.to_owned());
                },
                1 => {
                    reqchr.push(word.to_owned());
                },
                2 => {
                    passwd.push(word.to_owned());
                },
                _ => {
                    println!("FUCK!\n");
                    //ERROR
                }
            }
            i += 1;
        }
        let mut count = 0;
        for j in 0..policy.len() {
            //another hella hacky solution; I highly doubt this is idomatic, but
            // I'm mostly just experimenting with what works. Chaining like this
            // should be illegal. Only works for single digets anyways
            //let lower = &policy[j].chars().nth(0).unwrap().to_digit(10).unwrap(); 

            let p = &policy[j];
            let reg = Regex::new(r"(\d+)-(\d+)").unwrap();
            let cap = reg.captures(p).unwrap();
            let lower: u32 = std::str::FromStr::from_str(&cap[1]).unwrap();
            let upper: u32 = std::str::FromStr::from_str(&cap[2]).unwrap();
            let r = &reqchr[j].chars().nth(0).unwrap();
            let w = &passwd[j];

            let mut ch: char;

            for k in 0..w.len() {
                ch = w.chars().nth(k).unwrap();
                if ch == *r {
                    count += 1;
                }
            }
            if count < lower || count > upper {
                println!("{} failed policy! {}-{} {}", w, lower, upper, r);
            } else {
                println!("{} pass", w);
                validCount += 1;
            }
        }
    }

    println!("{} valid passwords", validCount);

    Ok(())
}
fn second(input: BufReader<File>) -> Result<()> {
    let mut validCount = 0;
    for line in input.lines() {
        let l = line.unwrap();
        //break each line into 3 parts, policy, required char, password
        //this is hacky, but it was the first way I thought to do this
        let mut i = 0usize;
        let mut policy: Vec<String> = Vec::new();
        let mut reqchr: Vec<String> = Vec::new();
        let mut passwd: Vec<String> = Vec::new();
        for word in l.split_whitespace() {
            //since we know each line has the same format, we can just match on field position
            match i % 3 {
                0 => {
                    policy.push(word.to_owned());
                },
                1 => {
                    reqchr.push(word.to_owned());
                },
                2 => {
                    passwd.push(word.to_owned());
                },
                _ => {
                    println!("FUCK!\n");
                    //ERROR
                }
            }
            i += 1;
        }
        let mut count = 0;
        for j in 0..policy.len() {
            let p = &policy[j];
            let reg = Regex::new(r"(\d+)-(\d+)").unwrap();
            let cap = reg.captures(p).unwrap();
            let lower: u32 = std::str::FromStr::from_str(&cap[1]).unwrap();
            let upper: u32 = std::str::FromStr::from_str(&cap[2]).unwrap();
            let r = &reqchr[j].chars().nth(0).unwrap();
            let w = &passwd[j];


            println!("{}, {}-{}",w, lower, upper);
            let lch = w.chars().nth(lower as usize - 1).unwrap();
            let uch = w.chars().nth(upper as usize - 1).unwrap();
            if (lch == *r) ^ (uch == *r) {
                println!("{} pass", w);
                validCount += 1;
            } else {
                println!("{} failed policy! {}-{} {}", w, lower, upper, r);
            }
        }
    }

    println!("{} valid passwords", validCount);

    Ok(())
}
