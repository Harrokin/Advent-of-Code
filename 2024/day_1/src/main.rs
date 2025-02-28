use std::fs;

fn main() {

    let mut i;

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let contents = contents.replace("   ","|").replace("\n","|").replace("\r","");
    
    let all_nums: Vec<&str> = contents.split("|").collect();

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    i = 0;
    while i < all_nums.len(){
        list1.push(all_nums[i].parse().expect("Not a number!"));
        i = i + 1;
        list2.push(all_nums[i].parse().expect("Not a number!"));
        i = i + 1;
    }

    list1.sort();
    list2.sort();

    i = 0;
    let mut total_distance = 0;

    while i < list1.len(){
        total_distance += (list1[i]-list2[i]).abs();
        i += 1;
    }

    println!("The total distance is {}.", total_distance);

    i = 0;
    let mut j;
    let mut multi;
    let mut total_similarity = 0;

    while i < list1.len(){

        println!("Checking number {}...", i);

        multi = 0;
        j = 0;

        while j < list2.len(){
            if list2[j] > list1[i] {
                j = list2.len(); //if we're greater than our 'left' number, we're not going to see it ever again and we can move on.
            } else if list2[j] == list1[i] {
                multi += 1;
                println!("Found {}!", multi);
                j += 1;
            } else {
                j += 1;
            }
        }

        total_similarity += list1[i]*multi;
        i += 1;
        
    }

    println!("The total total_similarity is {}.", total_similarity)

}
