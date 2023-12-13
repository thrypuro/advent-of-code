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


pub fn solve(){

    
}