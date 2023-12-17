use super::util::read_lines;

fn manhattan_distance(a: (i64,i64), b: (i64,i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}


pub fn solve(){
    // let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/day11.txt";
    let file_path = "/Users/thrypuro/Desktop/project/advent-of-code/aoc-2023/inputs/tests/test_day11.txt";
    let lines = read_lines(file_path);
    
    let mut extra_rows : Vec<i64> = Vec::new();
    let mut extra_cols : Vec<i64> = Vec::new();
    let mut galaxy : Vec<(i64,i64)> = Vec::new();
    for (i,a) in lines.iter().enumerate(){
            if !a.contains('#'){
                extra_rows.push(i as i64);
            }   
            else{
                let b = a.find('#').unwrap();
                if !galaxy.contains(&(i as i64,b as i64)){
                    galaxy.push((i as i64,b as i64));
                }
            }
        }

    for j in 0..lines[0].len() {
        let mut bb = true;
        for (i, a) in lines.iter()
        .map(|line| line.as_bytes()[j]).enumerate() {
                if a == b'#' {
                    bb = false;
                    let point = (i as i64, j as i64);
                    if !galaxy.contains(&point) {
                        galaxy.push(point);
                    }
                }
            }
            if bb {
                extra_cols.push(j as i64);
            }
    }

    let mut new_galaxy_col = vec![0; galaxy.len()];
    for &i in &extra_rows {
    for (j, &(x, _)) in galaxy.iter().enumerate() {
        if x > i {
            // replace with 1 for part 1
            new_galaxy_col[j] += 1000000-1;
        }
        }
    }
    let mut new_galaxy_row = vec![0; galaxy.len()];
    for &j in &extra_cols {
        for (i, &(_, y)) in galaxy.iter().enumerate() {
            if y > j {
            // replace with 1 for part 1 i.e.+=1
            new_galaxy_row[i] += 1000000-1;
         }
        }
    }
    galaxy.iter_mut().zip(new_galaxy_col.iter().zip(new_galaxy_row.iter()))
    .for_each(|(x,y)| {x.0 += y.0; x.1 += y.1});

    let mut total = 0;

    for i in 0..galaxy.len(){
        for j in i+1..galaxy.len(){
            let a = galaxy[i];
            let b = galaxy[j];
            total += manhattan_distance(a,b);
        }
        
    }
    println!("Total: {}", total);
    
}