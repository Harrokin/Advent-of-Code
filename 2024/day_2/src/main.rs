use std::fs;

#[derive(Debug)]
struct Reading {
    levels: Vec<i32>,
}

impl Reading {
    fn  safetycheck(&self, skip: usize) -> bool {
        let mut safe: bool = true;
        let ascending: bool = if skip == 0 { self.levels[1] < self.levels[2] } else if skip == 1 { self.levels[0] < self.levels[2] } else { self.levels[0] < self.levels[1] };
        let mut i: usize = if skip != 0 { 1 } else { 2 };
        let end = if skip == (self.levels.len()-1) { self.levels.len()-1 } else { self.levels.len() };
//println!("Comparing {} and {}", self.levels[i-1], self.levels[i]);

        while i < end{

            if i == skip {
                //println!("Too different!"); 
                if (self.levels[i+1] - self.levels[i-1]).abs() > 3 {safe = false; break;}
                //println!("No change!");
                if self.levels[i+1] == self.levels[i-1] {safe = false; break;}
                //println!("Ascending: {}, but {} > {} is opposite! Changed direction!", ascending, self.levels[i], self.levels[i-1]);
                if (self.levels[i+1] > self.levels[i-1]) != ascending {safe = false; break; }
            } else if i == (skip + 1) {
                if (self.levels[i] - self.levels[i-2]).abs() > 3 {safe = false; break;}
                if self.levels[i] == self.levels[i-2] {safe = false; break;}
                if (self.levels[i] > self.levels[i-2]) != ascending {safe = false; break; }
            } else {
                if (self.levels[i] - self.levels[i-1]).abs() > 3 {safe = false; break;}
                if self.levels[i] == self.levels[i-1] {safe = false; break;}
                if (self.levels[i] > self.levels[i-1]) != ascending {safe = false; break; }
            }

            i += 1;
        }

        //println!("{}, got to {}", safe, i);

        safe
    }
}

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let contents = contents.trim().replace("\r","");

    let all_inputs: Vec<&str> = contents.split("\n").collect();
    let mut all_readings: Vec<Reading> = Vec::new();

    for input_line in all_inputs {
        let level_strs: Vec<&str> = input_line.split(" ").collect();
        let mut level_nums: Vec<i32> = Vec::new();
        for string in &level_strs {
            level_nums.push(string.trim().parse().expect("Not a number."));
        }
        let input_reading: Reading = Reading {
            levels: level_nums,
        };
        all_readings.push(input_reading);
    };

    let mut safety_passes = 0; 

    for reading in all_readings {
        if reading.safetycheck(99){
            safety_passes += 1;
        } else {
            println!("Rechecking {reading:?}");
            for (i, _level) in reading.levels.iter().enumerate() {
                println!("Skipping {}", i);
                if reading.safetycheck(i) {
                    println!("Passes now!");
                    safety_passes += 1;
                    break;
                }
            }
        }
    }

    println!("Passes: {}", safety_passes);

}
