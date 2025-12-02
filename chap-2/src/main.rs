/// Problem link: https://adventofcode.com/2025/day/2

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

fn is_repeat(num: i128) -> bool {
    let s = num.to_string();
    let len = s.len();
    
    for k in 1..=len/2 {
        if len % k != 0 {continue;}

        let pattern = &s[0..k];
        let repeats = len / k ;

        if pattern.repeat(repeats) == s {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn Error>>{
    let inputs_path = "input.txt";
    let f = File::open(inputs_path)?;
    let reader = BufReader::new(f);

    let mut sum :i128 = 0;
    for line in reader.lines() {
        let line = line?;

        let range_vec: Vec<&str>= line.split(',').collect();

        for range in range_vec {
            let range = range.trim();

            let parts: Vec<&str> = range.split('-').collect();

            if parts.len() != 2{
               continue 
            }

            let start: i128 = parts[0].parse::<i128>().unwrap();
            let end: i128 = parts[1].parse::<i128>().unwrap();

            for i in start..=end {
                if  is_repeat(i) {
                    sum += i;
                }
            } 
        }
    }

    println!("{:?}", sum);

    Ok(())
}
