use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;
use std::io::Lines;

const red_constraint : u8 = 12;
const green_constraint : u8 = 13;
const blue_constraint : u8 = 14;

fn read_lines(file_path : &str) -> Lines<BufReader<File>>{
    let file = match File::open(Path::new(file_path)){
           Err(why) => panic!("Couldnt open file {}: {}", file_path,why),
           Ok(file) => file,
       };
       
       return BufReader::new(file).lines();
   }

fn parse_tuple_and_check(s : &str) -> (u8,u8,u8){
    let s = s.split(",");
    let mut r : u8 = 0;
    let mut g : u8 = 0;
    let mut b : u8 = 0;
    // parse "red" "green" "blue" in the form "3 red, 4 green, 5 blue"
    for i in s{
        let i = i.trim();
        let mut i = i.split(" ");
        let number = i.next().unwrap().parse::<u8>().unwrap();
        let color = i.next().unwrap();
        match color{
            "red" => r = number,
            "green" => g = number,
            "blue" => b = number,
            _ => panic!("bruh"),
        }
        
    }
    (r,g,b)
}

fn part_one_line(games : &str) -> bool {
    // println!("Line = {}",games);
    let sets = games.split(";");
    for i in sets{
        let a = parse_tuple_and_check(i);

        if (a.0 > red_constraint || a.1 > green_constraint || a.2 > blue_constraint) {
            return false;
        }
    }
    true
}

pub fn part_one(){
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day2.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;

    
    for i in file_lines{
        // one line represents one Game
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };
        let mut l = line.split(": ");
        // println!("line = {:?}",l); 
        let game_number = l.next().unwrap().replace("Game ", "").parse::<u64>().unwrap();

        if part_one_line(l.next().unwrap()){
            solve += game_number;
        }
    }

    println!("Part 1 Summation is = {}", solve);
    
}

fn part_two_line(games : &str) -> u64 {
    // println!("Line = {}",games);
    let sets = games.split(";");
    let mut r : u8 = 0;
    let mut g : u8 = 0;
    let mut b : u8 = 0;

    for i in sets{
        let a = parse_tuple_and_check(i);

        r = std::cmp::max(r,a.0);
        g = std::cmp::max(g,a.1);
        b = std::cmp::max(b,a.2);

       
    }
    (r as u64) * (g as u64) * (b as u64)
}

pub fn part_two(){
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day2.txt";
    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;

    
    for i in file_lines{
        // one line represents one Game
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };
        let mut l = line.split(": ");
        // println!("line = {:?}",l); 
        let game_number = l.next().unwrap().replace("Game ", "").parse::<u64>().unwrap();

        solve += part_two_line(l.next().unwrap());
        
    }

    println!("Part 2 Summation is = {}", solve);

}

