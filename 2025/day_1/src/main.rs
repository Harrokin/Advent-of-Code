use std::fs;
use regex::Regex;
//use substring::Substring;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mult_regex = Regex::new(r"(L|R)\d+").unwrap();
    let captures: Vec<&str> = mult_regex.find_iter(&input).map(|m| m.as_str()).collect();

    let mut currentPlace = 50;
    let mut total = 0;

    for string in captures
    {

        let dir = //substring 0,1 = L? negative 1. else positive 1
        let amount = //substring 2,end - converted to numbers TIMES dir
        println!("Read {}, moving {} from {}.", string, amount, currentPlace);
        currentPlace += amount;
        if(currentPlace%100==0)
        {
            total += 1;
        }
    }
    println!("{}", total);
}
