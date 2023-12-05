use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;
use std::io::Lines;
use std::collections::HashSet;

fn read_lines(file_path : &str) -> Lines<BufReader<File>>{
    let file = match File::open(Path::new(file_path)){
           Err(why) => panic!("Couldnt open file {}: {}", file_path,why),
           Ok(file) => file,
       };
       
       return BufReader::new(file).lines();
   }

pub fn part_one(){

    // let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day5.txt";
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/tests/test_day5.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;
    let mut j = 0;
    for i in file_lines{
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };

      
        j+=1;
    }
    println!("{}",solve);
}
pub fn part_two(){

    // let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day5.txt";
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/tests/test_day5.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;
    let mut j = 0;
    for i in file_lines{
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };

      
        j+=1;
    }
    println!("{}",solve);
}

