use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;
use std::collections::HashMap;
use num::integer::lcm;

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

#[derive(Debug)]
struct Node{
    node : u64,
    left : u64,
    right : u64
}


fn convert_to_u64(b : &str) -> u64 {
    let mut da : u64 = 0;
    for i in b.bytes(){
        da = da*0x100 + (i as u64);
    }
    da
}

fn parse_line(s : &String) -> Node{
    let mut a = s.split_once(" = ").unwrap();
    let b = a.0;
    let node = convert_to_u64(b);
    // (BBB, CCC)
    let c = a.1.split_once(", ").unwrap();
    let left = convert_to_u64(c.0.replace("(", "").as_str());
    let right = convert_to_u64(c.1.replace(")", "").as_str());

    let G = Node{node,left,right};
    G
}

// C <- A -> B
//      

pub fn solve(){
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day8.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day8.txt";

    let lines = read_lines(file_path);

    // node value -> graph 
    let mut graph : HashMap<u64,Node> = HashMap::new();


    let directions = lines.get(0).unwrap().as_bytes();



    for i in 2..lines.len() {

        let n = lines.get(i).unwrap();
        let n = parse_line(n);
        graph.insert(n.node, n);
    }

    let mut cur  = &graph[&0x414141];
    let mut i  = 0;
    while cur.node != 0x5a5a5a {
        let a = directions[i % directions.len()];
        let L = cur.left;
        let R = cur.right;
        if a == b'L'{
            cur  = &graph[&L];
        }
        else if a == b'R'{
            cur  = &graph[&R];
        }
        else {
            panic!("Huh_cat.gif");
        }
        i+=1;
    } 

    println!("Part 1 solution is {}",i);

    // Part 2

    let mut curs : Vec<&Node> = Vec::new();
    graph.iter().filter(|k| k.0 & 0x0000ff == 0x41)
    .for_each(|v| curs.push(v.1));

    let mut j = 0;
    let mut vec : Vec<i64> = Vec::new();
    while j<curs.len(){
        let mut x = curs[j];
        i = 0;
        while x.node & 0x0000ff != 0x5a{
            let a = directions[i % directions.len()];
            let L = x.left;
            let R = x.right;
            if a == b'L'{
                x  = &graph[&L];
            }
            else if a == b'R'{
                x  = &graph[&R];
            }
            else {
                panic!("Huh_cat.gif");
            }
            i+=1;
        }
        vec.push(i as i64);
        j+=1;
    }
              
    println!("Part 2 solution is {}",    
    vec.iter().fold(1, |a, &b| lcm(a, b)));

}
