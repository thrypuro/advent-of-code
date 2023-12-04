use std::collections::HashMap;
use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;
use std::io::Lines;


const DIGIT_NAMES: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1",
    "2", "3", "4", "5", "6", "7", "8", "9",
];


pub fn read_lines(file_path : &str) -> Lines<BufReader<File>>{
    let file = match File::open(Path::new(file_path)){
           Err(why) => panic!("Couldnt open file {}: {}", file_path,why),
           Ok(file) => file,
       };
       
       return BufReader::new(file).lines();
   }
   

pub fn part_one(){
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day1.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;
    for i in file_lines
    {
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };
        let mut count : u32 = 0;
        let mut br : u64 = 0;
        for c in line.chars(){

            if c.is_digit(10){
                
                br = br*10 + (c as u64 - 0x30);
                count+=1;
                
            }
        }
        if count > 1{
            let temp = br%10;
            let temp_2 = br/(u64::pow(10,count-1));
            br = temp_2*10 + temp;

        }
        else {
            br = br*10 + br;
        }

        solve += br;    

    }
    println!("Part 1 Summation equals = {}",solve)
}

fn p2_line(line : &str) -> u64{
    // 1two -> 1
    // two1 -> 21 -> 2 
    // aatwo1 -> aa21 -> 2
   
    let maps = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),

    ]);
    let mut ba = 0;
    let mut count = 0;
    for i in 0..line.len(){

        let la = &line[i..];
        for j in DIGIT_NAMES{
            if la.starts_with(j){
                ba = ba*10 + maps[j];
                count+=1;
            }
        }
    }
    if count > 1{
        let temp = ba%10;
        let temp_2 = ba/(u64::pow(10,count-1));
        ba = temp_2*10 + temp;

    }
    else {
        ba = ba*10 + ba;
    }
    
    // println!("{} -> {}",line,ba);

    ba
}

pub fn part_two(){
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day1.txt";

    let file_lines = read_lines(file_path);
    
    
    let mut solve : u64 = 0;

    for i in file_lines{
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };

        solve += p2_line(line.as_str());
    }

    println!("Part 2 Summation is = {}", solve);
    
}
