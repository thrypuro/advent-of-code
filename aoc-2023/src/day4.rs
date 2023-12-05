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
fn parse_card(card_str : String) -> usize{
    let mut a = card_str.split(" | ");
    let mut h : HashSet<u64> = HashSet::new();
    let b = a.next().unwrap().split(" ");
    

    for i in b{
        if i == ""{
            continue;
        }
        // println!("{}",i);
        h.insert(i.trim().parse::<u64>().unwrap());
    }
    let b = a.next().unwrap().split(" ");

    let mut count: usize = 0;
    for i in b{
        // println!("{}",i);
        if i == ""{
            continue;
        }
        let bb = i.trim().parse::<u64>().unwrap();
        if h.contains(&bb){
            count+=1;
        }

    }
    count
}


pub fn part_one(){

    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day4.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;
    let mut j = 1;
    for i in file_lines{
        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };

        let line = line.split(":");
        // get 2nd element of iterator
        let line = line.skip(1).next().unwrap();
        // println!("{}",line); 
        let count = parse_card(line.to_string());
        if count >= 1{
            solve+= u64::pow(2,(count - 1) as u32 );
        }
        j+=1;

    }

    println!("Part 1 Summation equals = {}",solve)

}


pub fn part_two(){
   let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day4.txt";

//    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/tests/test_day4.txt";
    let file_lines = read_lines(file_path);

    let mut j = 1;

    let mut h: Vec<u32> = Vec::new();

    
    for i in file_lines{
      

        let line = match i {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        };
        let line = line.split(":");
        // get 2nd element of iterator
        let line = line.skip(1).next().unwrap();
        // println!("{}",line); 
        let count = parse_card(line.to_string());


        if h.get(j-1) == None{
            h.push(1);
        }
        // card 1 has 4 copies 1 h[2]+=1 h[3]+=1 h[4]+=1
        // card 2 has 2 copies 2
        // card 3 has _ copies original (1) +  1 (1) +2*1 () 
            for k in j..count+j{
                if h.get(k) == None{
                    h.push(1);
                }
                // copies of card j times 1
                h[k as usize] += h[j-1];
    }        
        j+=1;

    }

    // println!(" H array {:?}",h);

    println!(" H array sum {}",h.iter().sum::<u32>());

}