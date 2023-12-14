use std::collections::HashMap;
use std::collections::HashSet;

use super::util::read_lines;

fn inside_polygon(path : Vec<(i64,i64)>,point : (i64,i64) ) -> bool{
    
    let mut inside = false;
    let mut j = path.len() - 1;
    for i in 0..path.len() {
        if (path[i].1 > point.1) != (path[j].1 > point.1) &&
            point.0 < (path[j].0 - path[i].0) * (point.1 - path[i].1) / (path[j].1 - path[i].1) + path[i].0 {
                inside = !inside;
            }
        j = i;
    }
    inside

}

pub fn solve(){
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day10.txt";
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day10.txt";
    let lines = read_lines(file_path);

    let mut edges: HashMap<(i64, i64), Vec<(i64,i64)>> = HashMap::new();
    let mut start: (i64, i64) = (-1, -1);

    let mut dots : Vec<(i64,i64)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let a = line.as_bytes();
        let ii = i as i64;

        for (j, &item) in a.iter().enumerate() {
            let jj = j as i64;
            let mut bb: Vec<(i64, i64)> = Vec::new();

            let check = |i: usize, j: usize| -> Option<(i64, i64)> {
                if lines.get(i).and_then(|line| line.as_bytes().get(j)) != Some(&b'.') {
                    // make sure the direction is connected to the start                
                    Some((i as i64, j as i64))
                } else {
                    None
                }
            };
           
            match item {
                b'|' => {
                    let a = check(i.saturating_sub(1), j);
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i + 1, j);
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }
                }
                b'-' => {
                    
                    let a = check(i, j.saturating_sub(1));
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i, j + 1);
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }

                }
                b'J' => {
                   
                    let a = check(i.saturating_sub(1), j);
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i, j.saturating_sub(1));
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }
                }
                b'L' => {

                    let a = check(i.saturating_sub(1), j);
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i, j + 1);
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }
                }
                b'7' => {

                    let a = check(i + 1, j);
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i, j.saturating_sub(1));
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }

                }
                b'F' => {
                  
                    let a = check(i + 1, j);
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i, j + 1);
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }

                }
                b'S' => {
                    start = (ii, jj);
                    // make sure the direction is connected to the start


                    let a = check(i + 1, j);
                    if a.is_some() {
                        bb.push(a.unwrap());
                    }
                    let b = check(i, j.saturating_sub(1));
                    if b.is_some() {
                        bb.push(b.unwrap());
                    }
                    let c = check(i.saturating_sub(1), j);
                    if c.is_some() {
                        bb.push(c.unwrap());
                    }
                    let d = check(i, j + 1);
                    if d.is_some() {
                        bb.push(d.unwrap());
                    }
                }
                b'.' => {
                    dots.push((ii, jj));
                    continue;
                },
                _ => panic!("huh_cat gif"),
            }
            edges.insert((ii, jj), bb);
        }
    }
    // println!("{:?}", edges);

    // println!("{:?}", start);

    // 


    let mut paths : Vec<Vec<(i64,i64)>> = Vec::new();
   

    for i in edges[&start].iter() {

        let mut visited: HashMap<(i64, i64), bool> = HashMap::new();

        let mut path = vec![start, *i];

        visited.insert(start, true);

        visited.insert(*i, true);

        let mut end = i;

        while *end != start {
            let neig = &edges[end];  
            if neig.iter().all(|j| visited.get(j).is_some()) {
                break;
            }
            for j in neig.iter() {
                if visited.get(j).is_none() {
                    visited.insert(*j, true);
                    path.push(*j);
                    end = j;
                    break;
                }

            }
            
        }
        
        println!("dist is {}", (path.len())/2);
         

        paths.push(path);
    
    }

    let mut point_inside : HashSet<(i64,i64)> = HashSet::new();
        

    // println!("{:?}", (path.clone()));

    // check if all points in lines are inside the polygon
    for i in 0..lines.len() {
        let a = lines[i].as_bytes();
        for (j, &item) in a.iter().enumerate() {
            let jj = j as i64;
            let ii = i as i64;
            let point = (ii,jj);
            let mut inside = false;
            for path in paths.iter() {
                if path.contains(&(ii,jj)){
                    continue;
                }
                if inside_polygon(path.clone(),point) &&
                    point_inside.get(&point).is_none()
                 {
                        inside = true;
                        break;
                    }
                }
            if inside {
                    point_inside.insert(point);

                }
            
        }
    }
    // println!(" dots {:?}", dots);
    // println!("point inside {:?}", point_inside);  
    println!("point inside len {:?}", point_inside.len());  

}