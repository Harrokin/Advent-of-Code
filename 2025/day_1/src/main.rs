use std::fs;
use regex::Regex;
use substring::Substring;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mult_regex = Regex::new(r"(L|R)\d+").unwrap();
    let captures: Vec<&str> = mult_regex.find_iter(&input).map(|m| m.as_str()).collect();

    let mut current_place: i32 = 50;
    let mut total = 0;

    for string in captures
    {

        let dir = if string.substring(0,1)=="R" {1} else {-1};//substring 0,1 = L? negative 1. else positive 1
        let mut amount = string.substring(1,string.len()).parse::<i32>().expect("NaN");//substring 2,end - converted to numbers TIMES dir
        println!("Moving {} from {}.", string, current_place);


        //Part 1

        //current_place += amount;
        /* 
        if(current_place%100==0)
        {
            total += 1;
        }
        */


        //Extra for part 2, there's again probably a cleaner solution with using divisions and modulo rather than looping

        total += amount / 100; 
        amount = amount%100; //So now we always have a value less than a full turn, see if it crosses or reaches 0

        if dir < 0
        {
            if amount >= current_place && current_place > 0
            {
                amount -= current_place;
                current_place = 0;
                total += 1;
            }
        }
        else
        {
            if (current_place + amount) >= 100
            {
                amount = current_place + amount - 100;
                current_place = 0;
                total += 1;
            }
        }

        current_place += amount*dir;

        while current_place > 99
        {
            current_place -= 100;
        }
        while current_place<0 
        {
            current_place += 100;
        }

        println!("Now at {}.  Current total: {}", current_place, total)

    }
    println!("{}", total);
}
