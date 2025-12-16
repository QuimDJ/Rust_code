#![allow(unused)]

use std::fs;
use std::io;

fn main()-> io::Result<()>{

    let contents=fs::read_to_string("./src/t.txt")?;
    for line in contents.lines() {
        println!("{line}");
    }
    Ok(())
    
}
    



