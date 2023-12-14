use super::util::read_lines;

fn find_bounds(d : i64, t : i64) -> i64{
    let f = | x : i64|{
        t*x-1*x*x -d > 0
    };
    let a : f64 = -1.0;
    let b : f64 = t as f64;
    let c : f64 = (d as f64) * -1.0;

    let mut lb = ((b - (b*b - 4.0*a*c).sqrt())/2.0).floor() as i64;
    let mut up = ((b + (b*b - 4.0*a*c).sqrt())/2.0).floor() as i64;
    if  !f(lb) {
        lb = lb +1; 
    }
    if !f(up){
        up = up -1;
    }
    up - lb + 1
}

pub fn solve(){
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day6.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day6.txt";
    let lines = read_lines(file_path);

    // Time : 5 6 7
    let time = lines[0].split_once(": ").unwrap().1
    .split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let distance = lines[1].split_once(": ").unwrap().1
    .split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    
    // zip time and distance as a tuple
    let total = time.iter().zip(distance.iter())
    .fold(1, |acc,y|  acc*find_bounds(*y.1,*y.0));
    println!("Part 1 solution : {}",total);

    let time2 = lines[0].split_once(": ").unwrap().1.split_whitespace()
    . fold(String::new(), |mut acc, x| {acc.push_str(x); acc}).parse::<i64>().unwrap();
    let distance2 = lines[1].split_once(": ").unwrap().1.split_whitespace()
    . fold(String::new(), |mut acc, x| {acc.push_str(x); acc}).parse::<i64>().unwrap();
    println!("Part 2 solution : {}",find_bounds(distance2,time2));
}
