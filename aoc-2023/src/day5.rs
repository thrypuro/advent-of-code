use super::util::read_lines;

fn f(x : u64,functions : &Vec<Vec<(u64,u64,u64)>> ) -> u64{
    let mut temp_x = x;
    for map in functions{

        // println!("map is {:?}",map);

        for ranges in map.iter() {
            let source = ranges.0;
            let dest  = ranges.1;
            let range = ranges.2;
            // println!("source {} destination {} range {}",source,dest,range);
            if temp_x >= source && temp_x < source + range {
    
                temp_x = dest + (temp_x - source);
                // println!("temp_x is {}",temp_x);
                break;
            }
        }
   
    }
    temp_x
}

fn f_inverse(x : u64,functions : &Vec<Vec<(u64,u64,u64)>> ) -> u64{
    let mut temp_x = x;
    for map in functions.iter().rev(){

        // println!("map is {:?}",map);

        for ranges in map.iter() {
            let source = ranges.0;
            let dest  = ranges.1;
            let range = ranges.2;
            // println!("source {} destination {} range {}",source,dest,range);
            if temp_x >= dest && temp_x < dest + range {
    
                temp_x = source + (temp_x - dest);
                // println!("temp_x is {}",temp_x);
                break;
            }
        }
   
    }
    temp_x
}

pub fn part_one_and_naive_p2(){

    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day5.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day5.txt";


    // parsing step 
    let file_lines = read_lines(file_path);
    let mut seed : Vec<u64> = Vec::new();

    let mut maps : Vec<Vec<(u64,u64,u64)>> = Vec::new();
    let mut i = 0;
    while i < file_lines.len(){
        
        let line = file_lines.get(i).unwrap();
        
        if line.contains("seeds: "){
            let l = line.split_once(": ").unwrap().1.split(" ");
            for num in l {
                seed.push(num.parse::<u64>().unwrap());
            }
        }
        else if line.contains("map:") {
            i+=1;
            let mut temp_vec : Vec<(u64,u64,u64)> = Vec::new();
            while file_lines.get(i)!=None && file_lines.get(i).unwrap()!=""  {
                let s = file_lines.get(i).unwrap();
                let numbers: Vec<u64> = s.split_whitespace()
                         .map(|s| s.parse::<u64>().unwrap())
                         .collect();
                let source = numbers[1];
                let destination  = numbers[0];
                let range = numbers[2];
             
                temp_vec.push((source,destination,range));
                i+=1;
            }
            maps.push(temp_vec);
        }
        i+=1;
    }
    // println!("maps are {:?}",maps);
    println!("seeds are {:?}",seed);

    let m = &maps;
    let mut v : Vec<u64> = Vec::new();
    for i in &seed{
        let a = f(*i,m);
        println!("F(seed_i) = {}",a);
        v.push(a);
    }

    // get min value 
    
    let a = v.iter().min().unwrap();
    println!("min value is {}",a);

    // part 2
    let mut seed_range : Vec<(u64,u64)> = Vec::new();
    i = 0;
    while i < (seed).len()-1{
        seed_range.push((seed[i],seed[i]+seed[i+1]));
        i+=2;
    }
    

    // println!("v is {:?}",v);
    let mut x : u64 = 0;
    while x < 0xFFFFFFFF{

        let temp_inv = f_inverse(x, &maps);
        // check if temp_inv is in seed_range
        let mut flag = false;
        for i in &seed_range{
            if temp_inv >= i.0 && temp_inv <= i.1{
                flag = true;
                println!("x is {} and temp_inv is {}",x,temp_inv);
                break;
            }
        }

        if flag == true{
            break;
        }
        x += 100000;
    }

    let mut x1 = 0;
    while x1 < x {
        let temp_inv = f_inverse(x1, &maps);
        // check if temp_inv is in seed_range
        let mut flag = false;
        for i in &seed_range{
            if temp_inv >= i.0 && temp_inv <= i.1{
                flag = true;
                println!("x1 is {} and temp_inv is {}",x1,temp_inv);
                break;
            }
        }

        if flag == true{
            println!("Smallest value is {} and in range is {}",x1,temp_inv);
            break;
        }
        x1 += 1;
    }
}
pub fn part_two(){

    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day5.txt";
    // let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/tests/test_day5.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;
    let mut j = 0;
    for i in file_lines{
        
        j+=1;
    }
    println!("{}",solve);
}

