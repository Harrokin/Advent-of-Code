use std::fs;
use regex::Regex;
use substring::Substring;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let ranges: Vec<&str> = contents.split(",").collect();
    for str in ranges
    {


        //So take the lower number
        //take first half of it and see if first half dup'd is in range ASSUMING IT CAN DUPE - SINGLE DIGITS ARE AN EDGE CASE
        //  if no, move on - no possible invalid IDs
        //  if it is, +1! (and maybe save for later?)
        //      Now if we increment first digit, is the dupe in range?
        //          If yes, then all increments of each other digit must also have been in range - so ten powers for each digit

        //so 5000-7000
        //5050 is in range! - this is the lowest invalid ID so if it's not in range we can skip
        //6000 is!
        //7000 is not
        //6900 is! - remember, we only care about the first half here
        //Therefore 5050, 5151, 5252, 5353, 5454, 5555, 5656, 5757, 5858, 5959, 6060, 6161, 6262, 6363, 6464, 6565, 6666, 6767, 6868, 6969  are all part 
                        //- which is 20, or ten per the second LSDigit 
        //would be 10x more per next significant digit
        //Since 6900 is but 7000 isn't, we need to check 6969 (which in this case is) if it isn't, we -1
    }
}
