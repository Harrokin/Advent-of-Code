use std::fs;
use regex::Regex;
use substring::Substring;


        //So take the lower number
        //take first half of it and see if first half dup'd is in range ASSUMING IT CAN DUPE - SINGLE DIGITS ARE AN EDGE CASE
                                                            //If our lowest is a single digit - the first dupe is like 11 - so check 11 is in range
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
                        //also equal to 5050 * 10, plus 1+2+3+4+5+6+7+8+9, plus 100+200etcetc
                        //1+2+3+4+5+6+7+8+9 = 45
                        //3500 for the hundreds
                        //So unchanged digits * 10 plus 45 multiplied by the appropriate power of 10 for each changing digit
                        //To test: our example would be 5050 * 10 + 6060 * 10 + 4545 +4545 =50500 +60600+9090=111100+9090=120190! It works!
                        // in this case hundreds are paired with units, because we're 4 digits so our LSDigit for the first half IS the hundreds, and LSD for second half is units
                        //might be easier to just iterate and add
                        //But surely it's possible to do multi stuff...
        //would be 10x more per next significant digit
        //Since 6900 is but 7000 isn't, we need to check 6969 (which in this case is) if it isn't, we -1


        //What if we take difference and divide by 100 - that gives 
        //base case of 0-100
        //  11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99 = 450 + 45
        // 0-1000
        //  11-99 =495
        //  100-999 nothing!
        // 0-10000
        //  11-99=495
        //  100-999=0
        //  1000-10000
        //    1010+1111+1212+1313+1414+1515+1616+1717+1818+1919 = 4500 (45 times ten power of LSB) + 45 (is also 45*10^bit) + 10*MSB (in this case 10k)
        //    2020+2121+2222+2323+2424+2525+2626+2727+2828+2929 = 4500 (45*10^bit3) + 45 (45*10^bit0) + 10*(MSB=2000)
        //    ... = 4500 + 45 + 10*(3000)
        //    ... = 4500 + 45 + 10*4000
        //    ...
        //    ...
        //    ...
        //    ...
        //    9090+9191+9292+9393+9494+9595+9696+9797+9898+9999 = 4500 + 45 + 10*9000

        //Since it's sum, value of sum of invalid IDs between two ranges = sum of ALL invalid IDs up to upper - sum of ALL invalid IDs up to lower
        //Ergo if we can do a quick calc to sum invalid IDs we can do this super duper easily.


fn main() {

    let input = fs::read_to_string("example.txt")
        .expect("Should have been able to read the file");

    let ranges: Vec<&str> = input.split(",").collect();
    for str in ranges
    {
        let numbers: Vec<&str> = str.split("-").collect();
        if numbers[0].len()%2 == 0 && numbers[1].parse::<u64>().expect("NaN") > 11
        {
            let lowest_dupe: u64 = format!("{}{}",numbers[0].substring(0,numbers[0].len()/2),numbers[0].substring(0,numbers[0].len()/2)).parse::<u64>().expect("NaN");
            let lower_bound: u64 = numbers[0].parse::<u64>().expect("NaN");
            let upper_bound: u64 = numbers[1].parse::<u64>().expect("NaN");
            if lowest_dupe <= upper_bound && lowest_dupe >= lower_bound
            {
                println!("Potential invalids between {} and {}.", lower_bound, upper_bound);
            }
        }
    }
}
