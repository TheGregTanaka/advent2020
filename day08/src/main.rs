use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    if args.len() > 1 {
        filename = String::from(&args[1]);
    } else {
        filename = String::from("input/_test.txt");
        println!("No filename given, defaulting to {}", filename);
    }
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut data = parser(&mut reader);
    /*for (s, i) in &data {
        println!("{} {}", s, i);
    }*/
    println!("First: Acc is {}", first(&mut data));
    let jumpLines = findJumps(&mut data);
    for j in jumpLines {
        println!("Jump on line {}", j);
    }
    
    //println!("Second: Acc is {}", second(&mut data));

    Ok(())
}

fn parser(reader: &mut BufReader<File>) -> Vec<(String, i32)> {
    let mut data = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let mut split = l.split_whitespace();
        data.push((String::from(split.next().unwrap()), 
            split.next().unwrap().parse::<i32>().unwrap()));
    }
    data
}

fn first(input: &mut Vec<(String, i32)>) -> i32 {
    let mut accum: i32 = 0;
    let mut i: isize = 0;
    let mut jump: isize;
    let mut visited = Vec::new();
    let mut done = false;
    let mut inst = &input[i as usize];
    while !done {
        visited.push(i);
        match inst.0.as_str() {
            "nop" => {
                //println!("Line {}: No op, continuing", i+1);
                i += 1;
            },
            "acc" => {
                accum += inst.1;
                //println!("Line {}: Accum + {} = {}", i+1, inst.1, accum);
                i += 1;
            },
            "jmp" => {
                jump = inst.1 as isize;
                //println!("Line {}: Jumping {}", i+1, jump);
                i += jump;
            },
            _ => {
                panic!("Line {}: HECKEROONI!", i+1);
            },
        }
        inst = &input[i as usize];
        //if the next instruction has been visited, break the loop
        if visited.iter().any(|&n| n == i) {
            println!("{} visited already", i);
            done = true;
        }
    }

    accum
}

fn findJumps(input: &mut Vec<(String, i32)>) -> Vec<usize> {
    let mut jmps = Vec::new();
    for i in 0..input.len() as usize {
        match (&input[i]).0.as_str() {
            "nop" => (),
            "acc" => (),
            "jmp" => {
                jmps.push(i);
            },
            _ => {
                panic!("Line {}: HECKEROONI!", i+1);
            },
        }
    }

    jmps
}
