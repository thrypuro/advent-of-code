use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;

fn read_lines(file_path : &str) -> Vec<String>{
    let file = match File::open(Path::new(file_path)){
           Err(why) => panic!("Couldnt open file {}: {}", file_path,why),
           Ok(file) => file,
       };
       
       let mut v : Vec<String> = Vec::new();
       for i in BufReader::new(file).lines(){
        let line = i.unwrap();
            v.push(line)
       }

       return v;
   }

fn compute_sequence(line : &String, part2 : bool) -> i64 {

    let l = line.split_whitespace()
    .map( | x | x.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
    let mut last : Vec<i64> = Vec::new();
    let mut first : Vec<i64> = Vec::new();

    let mut v = vec![l];

    while let last_seq = v.last().unwrap(){
        let diffs : Vec<i64> = last_seq.windows(2)
                .map(|w| w[1] - w[0]).collect();
        
        last.push(*last_seq.last().unwrap());
        first.push(last_seq[0]);
        if diffs.iter().all(|&x| x == 0){
            break;
        }
        v.push(diffs);
    }

    let sa = last.iter().sum::<i64>();
    // subtract the first[0] from the rest of the firsts
    first.reverse();
    let first_sum : i64 = 
        first.iter().fold(0, |acc, x| x - acc);
    if part2 {
        first_sum
    }
    else{
        sa
    }
}


pub fn solve(){ 
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day9.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day9.txt";
    let lines = read_lines(file_path);

    let mut s : i64 = 0;

    for i in &lines{
        
        s+=compute_sequence(i,false);

    }

    println!("Part 1 Solution : {}",s);
    s = 0;
    for i in lines{
        
        s+=compute_sequence(&i,true);
        // println!("lol what {}",compute_sequence(&i,true));

    }
    println!("Part 2 Solution : {}",s);
}