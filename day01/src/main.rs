use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    first(&input)?;
    second(&input)?;
    Ok(())
}

fn first(input: &str) -> Result<()> {
    let mut pair = Vec::new();
    let mut entry = 0u32;
    'outer: for line in input.lines() {
        entry = line.parse()?;
        // put the number that would sum with this entry to make 2020 in pair array
        pair.push(2020-entry);
        // if entry is in pair array, we found our match
        for j in 0..pair.len() {
            if pair[j] == entry {
                //println!("entry {}", entry);
                break 'outer;
            }
        }
    }
    println!("{} x {} = {}", entry, 2020-entry, entry*(2020-entry));


    Ok(())
}
//this is gross as fuck, but done is good
fn second(input: &str) -> Result<()> {
    let mut data = Vec::new();
    let mut entry: u32;
    'outer: for line in input.lines() {
        entry = line.parse()?;
        data.push(entry);
        let dl = data.len();
        for x in 0..dl {
            for y in x as usize..dl {
                for z in y as usize..dl {
                    if data[x] + data[y] + data[z] == 2020 {
                        let a = data[x] as i32 * data[y] as i32 * data[z] as i32;
                        println!("{} x {} x {} = {}", x, y, z, a);
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(())
}
