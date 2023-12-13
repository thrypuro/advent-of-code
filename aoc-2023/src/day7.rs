use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;
use std::collections::HashMap;

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

   
fn find_matches(st : &str,part2 : bool) -> (i64,i64){
    let f  = | x : u8 | match x {
        b'A' => 0xe,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 0xa,
        b'J' => if part2 { 1 } else { 0xb },
        b'Q' => 0xc,
        b'K' => 0xd,
        _ => 0,
    };
    let mut ca : HashMap<u8,i64> = HashMap::new();
    let mut da : i64 = 0;
    ca.insert(b'J', 0);
    for i in st.bytes(){
        let key = i;
        da = da*0x10 + f(key);
        *ca.entry(key).or_insert(0) +=1;  
    }

    let jo = if part2 {
        let a = ca.remove(&b'J').unwrap_or(0);
        a
    } else {
        0
    };

    let mut a: Vec<i64> = ca.into_values().collect();
    a.sort_unstable_by(|a, b| b.cmp(a));

    // this means there are 5 jokers
    if a.len()==0{
        return (da,6)
    }

    match a[0] + jo {
        5 => (da, 6),
        4 => (da, 5),
        3 => match a.get(1) {
            Some(&2) => (da, 4),
            _ => (da, 3),
        },
        2 => match a.get(1) {
            Some(&2) => (da, 2),
            _ => (da, 1),
        },
        _ => (da, 0),
    }
}

fn part2(lines: &Vec<String>){

    let mut vectors: Vec<Vec<i64>> = vec![Vec::new(); 7];
    let mut h: HashMap<i64, i64> = HashMap::new();
    
    for line in lines {
        let (key, value) = line.as_str().split_once(" ").unwrap();
        let v = find_matches(key,true);
        let number = v.0;
        let count = v.1;
        h.insert(number, value.parse::<i64>().unwrap());
        match count {
            0..=6 => vectors[count as usize].push(number),
            _ => println!("Error"),
        }
    }
    vectors.iter_mut().for_each(|x| x.sort());
    let mut total = 0;
    let mut combined: Vec<i64> = Vec::new();
    vectors.iter().for_each(|x| combined.extend(x));
    
    combined.iter()
    .enumerate()
    .for_each(|(i, x)| total += h[x] * (i as i64 + 1));
    
    println!("Part 2 solution : {}", total);
}


pub fn solve(){
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day7.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day7.txt";
    
    let lines = read_lines(file_path);
    
    let mut vectors: Vec<Vec<i64>> = vec![Vec::new(); 7];
    let mut h: HashMap<i64, i64> = HashMap::new();
    
    for line in &lines {
        let (key, value) = line.as_str().split_once(" ").unwrap();
        let v = find_matches(key, false);
        let number = v.0;
        let count = v.1;
        h.insert(number, value.parse::<i64>().unwrap());
        match count {
            0..=6 => vectors[count as usize].push(number),
            _ => println!("Error"),
        }
    }
    
    vectors.iter_mut().for_each(|x| x.sort());

    let mut total = 0;
    let mut combined: Vec<i64> = Vec::new();
    vectors.iter().for_each(|x| combined.extend(x));
    
    combined.iter()
    .enumerate()
    .for_each(|(i, x)| total += h[x] * (i as i64 + 1));
    
    println!("Part 1 solution : {}", total);
    part2(&lines);
}
