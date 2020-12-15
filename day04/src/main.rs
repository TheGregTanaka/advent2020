extern crate regex;
use std::env;
use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;
use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1])?;
    let mut reader = BufReader::new(f);
    
    //first(&mut reader);
    second(&mut reader);
    Ok(())
}

fn first(input: &mut BufReader<File>) -> Result<()> {
    let mut validCount = 0;
    let mut fields: Vec<String> = Vec::new();
    for line in input.lines() {
        let l = line.unwrap();
        // if line is empty, check found fields, then go to next passport
        if l.is_empty() {
            //and then we do this the grossest laziest way possible
            if fields.contains(&"byr".to_string()) &&
                fields.contains(&"iyr".to_string()) &&
                fields.contains(&"eyr".to_string()) &&
                fields.contains(&"hgt".to_string()) &&
                fields.contains(&"hcl".to_string()) &&
                fields.contains(&"ecl".to_string()) &&
                fields.contains(&"pid".to_string()) {
                validCount += 1;
            }
            
            //clear our fields
            for i in 0..fields.len() {
                fields.remove(0);
            }
        } else {
            //match up to :
            let reg = Regex::new(r"(\w+):").unwrap();
            for word in l.split_whitespace() {
                let cap = reg.captures(word).unwrap();
                fields.push(cap[1].to_owned());
            }
        }
    }
    println!("Valid passports: {}", validCount);
    Ok(())
}

fn second(input: &mut BufReader<File>) -> Result<()> {
    let mut validCount = 0;
    let mut fields: Vec<String> = Vec::new();
    for line in input.lines() {
        let l = line.unwrap();
        // if line is empty, check found fields, then go to next passport
        if l.is_empty() {
            //and then we do this the grossest laziest way possible
            if fields.contains(&"byr".to_string()) &&
                fields.contains(&"iyr".to_string()) &&
                fields.contains(&"eyr".to_string()) &&
                fields.contains(&"hgt".to_string()) &&
                fields.contains(&"hcl".to_string()) &&
                fields.contains(&"ecl".to_string()) &&
                fields.contains(&"pid".to_string()) {
                validCount += 1;
            }
            
            //clear our fields
            for i in 0..fields.len() {
                fields.remove(0);
            }
        } else {
            for word in l.split_whitespace() {
                let v: Vec<&str> = word.split(':').collect();
                //only save a valid field
                match v[0] {
                    "byr" => {
                        let b = v[1].parse::<i32>().unwrap();
                        if b >= 1920 && b <= 2002 {
                            fields.push(v[0].to_owned());
                        }
                    },
                    "iyr" => {
                        let i = v[1].parse::<i32>().unwrap();
                        if i >= 2010 && i <= 2020 {
                            fields.push(v[0].to_owned());
                        }
                    },
                    "eyr" => {
                        let e = v[1].parse::<i32>().unwrap();
                        if e >= 2020 && e <= 2030 {
                            fields.push(v[0].to_owned());
                        }
                    },
                    "hgt" => {
                        let len = v[1].len();
                        match &v[1][len-2..len] {
                            "cm" => {
                                let h = (&v[1][..len-2]).parse::<i32>().unwrap();
                                if h >= 150 && h <= 193 {
                                    fields.push(v[0].to_owned());
                                }
                            },
                            "in" => {
                                let h = (&v[1][..len-2]).parse::<i32>().unwrap();
                                if h >= 59 && h <= 76 {
                                    fields.push(v[0].to_owned());
                                }
                            },
                            &_ => {},
                        }
                    },
                    "hcl" => {
                        let reg = Regex::new(r"^#(?:[0-9a-f]{6})").unwrap();
                        let cap = reg.captures(v[1]);
                        match cap {
                            Some(_inner) => fields.push(v[0].to_owned()),
                            None => (),
                        }
                    },
                    "ecl" => {
                        match v[1] {
                            "amb" => fields.push(v[0].to_owned()),
                            "blu" => fields.push(v[0].to_owned()),
                            "brn" => fields.push(v[0].to_owned()),
                            "gry" => fields.push(v[0].to_owned()),
                            "grn" => fields.push(v[0].to_owned()),
                            "hzl" => fields.push(v[0].to_owned()),
                            "oth" => fields.push(v[0].to_owned()),
                            _ => (),
                        }
                    },
                    "pid" => {
                        let reg = Regex::new(r"[0-9]{9}").unwrap();
                        let p = reg.captures(v[1]);
                        match p {
                            Some(_inner) => fields.push(v[0].to_owned()),
                            None => (),
                        }
                    },
                    &_ => {},
                }
            }
        }
    }
    println!("Valid passports: {}", validCount);
    Ok(())
}
