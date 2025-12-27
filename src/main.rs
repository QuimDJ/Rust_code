#![allow(unused)]

use colored::Colorize;
use std::io;


fn main(){
    let word="trait";
    let input=io::stdin();
    for _ in 1..=6{
        let mut user_input=String::new();
        input.read_line(&mut user_input)
        .expect("Failed to provide input");
        let w:Vec<(char,char)> = word.chars()
        .zip(user_input.chars().take(5))
        .collect();
        println!("{:?}",w);
    
    }
    
}
    



