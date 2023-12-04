use std::fs::File;
use std::io:: {BufReader,BufRead};
use std::path::Path;
use std::io::Lines;
use std::collections::HashMap;


fn read_lines(file_path : &str) -> Lines<BufReader<File>>{
    let file = match File::open(Path::new(file_path)){
           Err(why) => panic!("Couldnt open file {}: {}", file_path,why),
           Ok(file) => file,
       };
       
       return BufReader::new(file).lines();
   }





fn parse_engine(file_lines : Lines<BufReader<File>>, vaa : &mut Vec<(usize,usize)>, gaa : &mut Vec<(usize,usize)> ) -> Vec<Vec<i32>>
{

    let mut array : Vec<Vec<i32>> = Vec::new();

    let mut i = 0;
    for la in file_lines {
        
        let line = match la {
            Err(why) => panic!("bruh {}",why), 
            Ok(line) => line,
        }; 
        let la = line.as_bytes();  
        let n = line.len();
        let mut j=0;
        let mut va : Vec<i32> = Vec::new();
        while j < n {
            // 456 -> 456 456 456
            // .*. -> .    *   . 
            let a = la[j];
            let mut ba = 0;
            if a>=0x30 && a<=0x39{
                let mut count = 0;
                while j+count<n && la[j+count]>= 0x30 && la[j+count] <= 0x39 {
                  ba = ba*10 + (la[j+count] as i32)%16;
                  count+=1;
                  }
                let s = j+count;
                while j < s {
                    va.push(ba);
                    j+=1;
                    // println!("j = {} , {}",j,count);
                }
                j-=1;     
            }
            else if a == b'.' {
               va.push(-1);
            }
            else{
                va.push(-2);
                if a ==b'*'{ 
                    gaa.push((i,j));
                }
                vaa.push((i,j));

            }
            j+=1;            
        }

        array.push(va);      
        i+=1;  

    }
    // println!("array {:?}",array);
    // println!("Vaaa {:?}",vaa);
    array

}

pub fn part_one(){
    let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day3.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : i32 = 0;
    // 456...
    // *.....
    // 0
   // time for ownership with hashmap

   let mut symbol_index : Vec<(usize,usize)> = Vec::new();

    let mut gear_index : Vec<(usize,usize)> = Vec::new();

    let a = parse_engine(file_lines,&mut symbol_index, &mut gear_index);
    for i in symbol_index{
        let xi = i.0 as i32;
        let yi = i.1 as i32;
        let baa : [i32;3] = [-1,0,1];


        for k in baa {


   let mut hah : HashMap<i32,bool> = HashMap::new();
         for j in baa{
        if k == 0 && j == 0 {
            continue; // Skip the current iteration if k and j are both 0
        }
        if xi + k >= 0 && xi + k < a.len() as i32 && yi + j >= 0 && yi + j < a[0].len() as i32 {
          
          let xi = (xi+k) as usize;
          let yi = (yi+j) as usize;
          if hah.contains_key(&a[xi][yi]){
            continue;
          }
          if a[xi][yi] != -2 && a[xi][yi] != -1{
            solve+=a[xi][yi];
            hah.insert(a[xi][yi],true);
            // println!("{} {} {}",xi,yi,a[xi][yi]);
          }
        }

        }
    
    }
    
    }    

    println!("Summation is equal to = {}",solve);
}


pub fn part_two(){
     let file_path = "/home/thrypuro/Desktop/projects/advent-of-code-2023/inputs/day3.txt";

    let file_lines = read_lines(file_path);
    
    let mut solve : u64 = 0;
    // 456...
    // *.....
    // 0
   // time for ownership with hashmap

   let mut symbol_index : Vec<(usize,usize)> = Vec::new();

    let mut gear_index : Vec<(usize,usize)> = Vec::new();

    let a = parse_engine(file_lines,&mut symbol_index, &mut gear_index);
   for i in gear_index{
        
        // the index stuff of the geatr
        let xi = i.0 as i32;
        let yi = i.1 as i32;
        let baa : [i32;3] = [-1,0,1];

        // the product and count 
        let mut da : u64 = 1;
        let mut count = 0;


        for k in baa {
        
        
            let mut hah : HashMap<i32,bool> = HashMap::new();
            for j in baa{
                if k == 0 && j == 0 {
                        continue; // Skip the current iteration if k and j are both 0
                }

                if xi + k >= 0 && xi + k < a.len() as i32 && yi + j >= 0 && yi + j < a[0].len() as i32 {
            
                let xi = (xi+k) as usize;
                let yi = (yi+j) as usize;
                if hah.contains_key(&a[xi][yi]){
                    continue;
                }
                if a[xi][yi] != -2 && a[xi][yi] != -1{
                    da*= (a[xi][yi] as u64);
                    hah.insert(a[xi][yi],true);
                    // println!("{} {} {}",xi,yi,a[xi][yi]);
                    count+=1;
                                 
                                 }
         }
        
        }
 
   }
      if count ==2 { 

        solve+=da;  
    }  
    
}

println!("Part 2 Summation is equal to = {}",solve);


}


