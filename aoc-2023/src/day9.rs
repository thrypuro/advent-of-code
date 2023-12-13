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

    let mut v: Vec<Vec<i64>> = Vec::new();
    v.push(l);
    loop {
        // 0 3 6 9 12
        //  3 3 3 3
        //   0 0 0
        let mut s  = true;
        
        let last_seq = &v[v.len()-1];
        let mut a : Vec<i64> = Vec::new();
        let ba = last_seq[last_seq.len()-1];
        let da = last_seq[0];
        for i in 0..last_seq.len()-1{
            let d = last_seq[i+1]-last_seq[i];
            a.push(d);
            s = s && d==0;
        }
        v.push(a);
        last.push(ba);
        first.push(da);

        if s{
            break;
        }

    }
    let sa = last.iter().sum::<i64>();
    // subtract the first[0] from the rest of the firsts
    let first_sum = 2*first[0] - first.iter().sum::<i64>();
    if part2 {
        return first_sum;
    }
    return sa;
}


pub fn solve(){ 
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day9.txt";
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day9.txt";
    let lines = read_lines(file_path);

    let mut s : i64 = 0;

    for i in &lines{
        
        s+=compute_sequence(i,false);

    }

    println!("Part 1 Solution : {}",s);
    s = 0;
    for i in lines{
        
        s+=compute_sequence(&i,true);

    }
}