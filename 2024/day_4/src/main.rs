use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");
    let contents = contents.trim().replace("\r","");

    let all_inputs: Vec<&str> = contents.split("\n").collect();

    let lines = all_inputs.try_into().unwrap();

    let match_string = "XMAS";
    let mut matches = 0;

    for (i, string) in all_inputs.iter().enumerate(){
        for (j, char) in string.chars().enumerate() {
            if char == match_string.chars().next().unwrap() {

                if i > match_string.len()-1 {
                    if j > match_string.len()-1 {
                        //Top left
                        if all_inputs.iter()[i-1].chars().nth(j-1) == match_string.chars().nth(1) && true {
                            matches += 1;
                        }
                    }
                    // Top
                    if j <= string.len()-match_string.len() {
                        //Top right
                    }
                }

                if j > match_string.len()-1 {
                    //Left
                }
                if j <= string.len()-match_string.len() {
                    //Right
                }

                if i <= string.len()-match_string.len() {
                    if j > match_string.len()-1 {
                        //Bottom left
                    }
                    // Bottom
                    if j <= string.len()-match_string.len() {
                        //Bottom right
                    }
                }

            }
        }
    }
    
    println!("{} matches.", matches);
}
