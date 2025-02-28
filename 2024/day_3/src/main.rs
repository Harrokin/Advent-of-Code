use std::fs;
use regex::Regex;
use substring::Substring;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mult_regex = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let captures: Vec<&str> = mult_regex.find_iter(&input).map(|m| m.as_str()).collect();

    let mut mult_enable = true;
    let mut mult_args: Vec<(i32, i32)> = Vec::new();

    for string in captures{
        if string == "do()" {
            println!("Enabled!");
            mult_enable = true;
        } else if  string == "don't()" {
            println!("Disabled!");
            mult_enable = false;
        } else {
            if mult_enable {
                println!("Processing: {}", string);
                let comma = Regex::new(r",").unwrap().find(string).unwrap().start();//index of the comma
                let first_arg = string.substring(4,comma).trim();// substr from 4-comma
                let second_arg = string.substring(comma+1, string.len()-1).trim();
                //println!("{}, {}", first_arg, second_arg);
                mult_args.push((first_arg.parse::<i32>().expect("NaN"), second_arg.parse::<i32>().expect("NaN")));
            } else{
                println!("Skipped!");
            }
        }  
    }

    let mut total: i32 = 0;

    for (first, second) in mult_args{
        //println!("{}, {}", first, second);
        total += first*second;
    }
    println!("Done. Total: {}", total);
}
