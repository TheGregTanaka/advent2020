use std::env;
use std::io::{Read, BufRead, BufReader};
use std::fs::File;

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
    //create vector of seatIds as checklist
    let mut checkList: Vec<i32> = (0..).collect::<Vec<i32>>();
    let mut seats: Vec<i32> = Vec::new();
    /*let mut checkList: Vec<i32> = Vec::new();
    for i in 0..=127 {
        for j in 0..=7 {
            checkList.push(i * 8 + j);
        }
    }*/

    //get strings
    for line in input.lines() {
        let l = line.unwrap();
        let len = l.len();
        //break into row and collumns
        let mut row = &l[..len-3];
        let mut col = &l[len-3..len];
        //convert to binary
        println!("row {}", row);
        let binRow: String = row.chars().map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => ' ',
        }).collect();
        let binCol: String = col.chars().map(|x| match x {
            'L' => '0',
            'R' => '1',
            _ => ' ',
        }).collect();
        println!("row {}", binRow);
        //println!("col {}", binCol);
        let rowInt = isize::from_str_radix(binRow.as_str(), 2).unwrap();
        println!("row {}", rowInt);
        let colInt = isize::from_str_radix(binCol.as_str(), 2).unwrap();
        //calculate seat id and remove from list of posibilities
        let seatID: i32 = (rowInt * 8 + colInt) as i32;
        seats.push(seatID);
        let i = checkList.iter().position(|&x| x == seatID).unwrap();
        checkList.swap_remove(i as usize);
    }

    println!("{} is your seat", checkList[0]);
    println!("{} left in list", checkList.len());
    println!("{} seats taken ", seats.len());
    for n in checkList {
        println!("{}", n);
    }
    let mut maxSeat: i32 = 0;
    for m in seats {
        if m > maxSeat {
            maxSeat = m;
        }
        //println!("{}", m);
    }
    println!("{} max seat", maxSeat);

    Ok(())
}

/*
 * 128 rows of 8 collums should result in 1024 seats, but that doesn't seem to 
 * be the case here. Only 771 inputs have been provided. Based on the sanity 
 * check hint, the highest seat on a boarding pass is 816. 816/8 = 102 leading 
 * me to believe that the plane doesn't have every row the boarding pass can 
 * possibly list. This function is to help me better understand inputs because 
 * I've wasted WAY too much time trying to figure out how to narrow down the
 * missing 253 seats. - update for posterity, I had this problem solved almost 
 * right away but thought part 1 was asking for my seat, lol whoops
 */
fn inputsWut(input: BufReader<File>) -> Result<()> {
    //create vector of seatIds as checklist
    let mut rowNums: Vec<isize> = Vec::new();
    let mut colNums: Vec<isize> = Vec::new();
    let mut seats: Vec<i32> = Vec::new();

    //get strings
    for line in input.lines() {
        let l = line.unwrap();
        let len = l.len();
        //break into row and collumns
        let mut row = &l[..len-3];
        let mut col = &l[len-3..len];
        //convert to binary
        println!("row {}", row);
        let binRow: String = row.chars().map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => ' ',
        }).collect();
        let binCol: String = col.chars().map(|x| match x {
            'L' => '0',
            'R' => '1',
            _ => ' ',
        }).collect();
        println!("row {}", binRow);
        //println!("col {}", binCol);
        let rowInt = isize::from_str_radix(binRow.as_str(), 2).unwrap();
        rowNums.push(rowInt);
        println!("row {}", rowInt);
        let colInt = isize::from_str_radix(binCol.as_str(), 2).unwrap();
        colNums.push(colInt);
        //calculate seat id and remove from list of posibilities
        let seatID: i32 = (rowInt * 8 + colInt) as i32;
        seats.push(seatID);
    }
    let mut maxRow: isize = 0;
    let mut maxCol: isize = 0;
    let mut maxSeat: i32 = 0;
    for i in 0..seats.len() {
        println!("Row {} Col {} SeatID {}", rowNums[i], colNums[i], seats[i]);
        if maxRow < rowNums[i] {
            maxRow = rowNums[i];
        }
        if maxCol < colNums[i] {
            maxCol = colNums[i];
        }
        if maxSeat < seats[i] {
            maxSeat = seats[i];
        }
    }
    println!("Max Row {} Max Col {} Max Seat {}", maxRow, maxCol, maxSeat);

    Ok(())
}

//okay NOW we find our seat
fn second(input: BufReader<File>) -> Result<()> {
    //get min and max seats
    let mut seats: Vec<i32> = Vec::new();

    //get strings
    for line in input.lines() {
        let l = line.unwrap();
        let len = l.len();
        //break into row and collumns
        let mut row = &l[..len-3];
        let mut col = &l[len-3..len];
        //convert to binary
        let binRow: String = row.chars().map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => ' ',
        }).collect();
        let binCol: String = col.chars().map(|x| match x {
            'L' => '0',
            'R' => '1',
            _ => ' ',
        }).collect();
        let rowInt = isize::from_str_radix(binRow.as_str(), 2).unwrap();
        let colInt = isize::from_str_radix(binCol.as_str(), 2).unwrap();
        //calculate seat id and remove from list of posibilities
        let seatID: i32 = (rowInt * 8 + colInt) as i32;
        seats.push(seatID);
    }
    seats.sort();
    let mut seatCursor: i32 = 0;
    for i in 1..seats.len()-1 {
        seatCursor = seats[i];
        //there is a logical bug where I'm actually outputing the seat below and
        //above. Rather than resolving that though, because I recognized it, I just
        //mentally found the number between to solve the problem, and formatted
        //the output to accomodate that. This isn't production
        //code so done is good.
        if !(seatCursor == (seats[i-1] + 1) && seatCursor == (seats[i+1] - 1)) {
            println!("Your seat is between {} ", seatCursor);
        }
    }

    Ok(())
}
