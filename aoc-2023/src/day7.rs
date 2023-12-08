use core::num;
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
fn find_matches(st : &str) -> (i64,i64){
    let h : HashMap<u8,i64> = HashMap::from([
        (b'A',0xe),
        (b'2',2),
        (b'3',3),
        (b'4',4),
        (b'5',5),
        (b'6',6),
        (b'7',7),
        (b'8',8),
        (b'9',9),
        (b'T',0xa),
        (b'J',0xb),
        (b'Q',0xc),
        (b'K',0xd),]
    );

    let mut ca : HashMap<u8,i64> = HashMap::new();

    let mut da : i64 = 0;
    let mut max = 0;
    for i in st.chars(){
        let key = (i as u8).clone();
        da = da*0x10 +  h[&key];
        // check in ca else add
        if ca.contains_key(&key){
            ca.insert(key,ca[&key]+1);
        }
        else{
            ca.insert(key,1);
        }
        if ca[&key] > max{
            max = ca[&key];
        }
    }

    (da,max)
}


pub fn solve(){
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day7.txt";
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day7.txt";
    let lines = read_lines(file_path);

    // 
    let mut one: Vec<i64> = Vec::new();
    let mut two: Vec<i64> = Vec::new();
    let mut three: Vec<i64> = Vec::new();
    let mut four: Vec<i64> = Vec::new();
    let mut five: Vec<i64> = Vec::new();
    let mut high: Vec<i64> = Vec::new();

    let mut h : HashMap<i64,i64> = HashMap::new();

    for i in lines{
      
        let v = i.as_str().split_once(" ").unwrap();
        let key = v.0.clone();
        let value = v.1.parse::<i64>().unwrap();
        let t  = find_matches(key);
        let number = t.0;
        let count = t.1;
        println!("{}",i);
        println!("{} {}",number,count);
        h.insert(number,value);
        match count {
            0 => high.push(number),
            1 => one.push(number),
            2 => two.push(number),
            3 => three.push(number),
            4 => four.push(number),
            5 => five.push(number),
            _ => println!("Error"),
        }

    }

    println!("High : {:?}",high);
    println!("One : {:?}",one);
    println!("Two : {:?}",two);
    println!("Three : {:?}",three);
    println!("Four : {:?}",four);
    println!("Five : {:?}",five);

    high.sort();
    one.sort();
    two.sort();
    three.sort();
    four.sort();
    five.sort();

    let mut total = 0;
    // combine all the vectors
    let mut v = Vec::new();
    v.append(&mut high);
    v.append(&mut one);
    v.append(&mut two);
    v.append(&mut three);
    v.append(&mut four);
    v.append(&mut five);

    for i in 0..v.len(){
        total += h[&v[i]]*(i as i64 + 1);
    }

    println!("Part 1 solution : {}",total);
}
