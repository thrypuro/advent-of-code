
use std::collections::HashMap;
use super::util::read_lines;


// Solution 1, only works for part 1, did it for funnies
// basically bruteforce every possible combination of # and .
fn get_bit(n : u64, i : u64) -> u64{
    (n >> i) & 1
}
fn check_answer(vec : &Vec<u8>, cons : &Vec<usize> ) -> bool{
    let mut j = 0;
    
    // split vec by "."
    let mut s = String::from_utf8((*vec).clone()).unwrap();

    s = s.trim_end_matches('.').to_string();
    let v = s.split(".").collect::<Vec<&str>>();
    let mut v2 = Vec::new();
    for i in v.iter(){
        if i.len() > 0{
            v2.push(i.len() as i64);
        }
    }
    if v2.len() != cons.len(){
        return false;
    }
    // compare v2 and cons
    for i in v2.iter(){
        if *i != cons[j] as i64{
            return false;
        }
        j += 1;
    }
    true
}

fn solve_line(line : String) -> i32{


    let mut index_so : Vec<usize> = Vec::new();

    let l = line.split_once(" ").unwrap();
    let la = l.0;
    let mut vec_copy = vec![0;la.len()];

    // for every ?
    for (j,i) in la.chars().enumerate(){
        if i == '?'{
            index_so.push(j);
        }
        vec_copy[j] = i as u8;
    }


    let cons  = l.1.split(",")
    .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    

    let brut = u64::pow(2,index_so.len() as u32);
    let mut total = 0;
    for i in 0..brut{
        for j in 0..index_so.len(){
            if get_bit(i,j as u64) == 1{
                vec_copy[index_so[j]] = '#' as u8;
            }else{
                vec_copy[index_so[j]] = '.' as u8;
            }
        }
        
        let a = String::from_utf8(vec_copy.clone()).unwrap();
        if check_answer(&vec_copy,&cons){
            println!("answer {}",a);
            total += 1;
        }
    }

    println!("total {}",total);
    total
}

// Solution 2, recursive
// decision tree for every ? and ., and then check if it matches the constraints
// ??.?? 2 , 1
// ##.?? 2 , 1 Check Pass
//  ^- Check here index = 1
// .#.? 2 , 1 Check Fail
fn recursive( vec_copy: &Vec<u8>,cons : &Vec<usize> ,i : usize ,g_i : usize,c_i : usize,
map : &mut HashMap<(usize,usize,usize),i64>
) -> i64 {
    if map.contains_key(&(i,g_i,c_i)){
        return map[&(i,g_i,c_i)];
    }
    else if g_i >= cons.len(){
        if  i >= vec_copy.len(){
            return 1;
        }
        // check if no more "#" left
        for j in i..vec_copy.len(){
            if vec_copy[j] == b'#'{
                return 0;
            }
        }
        return 1;

    }
    else if i >= vec_copy.len(){
        return 0 ;
    }

    let a = vec_copy[i];

    let mut ans = 0;

    match a {
        b'?' => {

            // behave like . 
            if c_i == 0 {
                ans +=recursive(vec_copy,cons, i+1, g_i, c_i,map);
             }
             // okay we counted something, make sure we consume the group
             else if c_i == cons[g_i] {

                 ans +=recursive(vec_copy, cons, i+1, g_i+1, 0,map);
             }
             else {
                 ans+= 0;
             }
            

            ans+=recursive(vec_copy, cons,i+1, g_i, c_i+1,map);


        }
        b'#' => {   

            ans+=recursive(vec_copy, cons,i+1, g_i, c_i+1,map) ;
        }
        b'.' => {
            // empty space keep going
            if c_i == 0 {
               ans+=recursive(vec_copy,cons, i+1, g_i, c_i,map);
            }
            // okay we counted something, make sure we consume the group
            else if c_i == cons[g_i] as usize {
                ans+=recursive(vec_copy, cons, i+1, g_i+1, 0,map);
            }
            else {
                ans+= 0;
            }

        }
        _ => {
            ans+= 0;
        }
    }

    map.insert((i,g_i,c_i),ans);

    ans
    
}

fn solve_line_recursion(line : &String) -> i64{
    let mut index_so : Vec<usize> = Vec::new();

    let l = line.split_once(" ").unwrap();
    let la = l.0;
    // copy repeat la 5 times seperated by "?"
    let mut vec_copy = vec![0;la.len()];

    // for every ?
    for (j,i) in la.chars().enumerate(){
        if i == '?'{
            index_so.push(j);
        }
        vec_copy[j] = i as u8;
    }

    vec_copy.push(b'.');

    let cons  = l.1.split(",")
    .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut total = 0;
    println!("total {}",total);
    // Group 
    // . -> consume group, if counter is 0, just keep going, otherwise, check if it is correct
    // # -> increase or start group counter
    // ? -> . or # behavior, 
    // g_i group index, c_i counter index


    let a = recursive(&vec_copy,&cons,0, 0,0,&mut HashMap::new());
    println!("a = {}",a);
    a
}

pub fn solve(){
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day12.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day12.txt";
    let lines = read_lines(file_path);
    let mut total = 0;
    for line in &lines {
        println!("line {}",line);   
        // total+=solve_line(line);
        total += solve_line_recursion(line);
        // break 
    }
    println!("Part 1 solution : {}",total);

    // part 2 

    total = 0;
    for (i,line) in (&lines).iter().enumerate() {
        println!("line {}",line);  
        // repeat line 5 times with 
        let a = line.split_once(" ").unwrap();
        let la = a.0;
        let ba = a.1;
        // copy repeat la 5 times seperated by "?"
        let a  = la.to_owned() + "?" + la + "?" + la + "?" + la + "?" + la; 
        let bb = ba.to_owned() + "," + ba + "," + ba + "," + ba + "," + ba;
        let b = a + " " + &bb;
        total += solve_line_recursion(&b);
    }

    println!("Part 2 solution : {}",total);

}