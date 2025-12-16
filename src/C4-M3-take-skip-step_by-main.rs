#![allow(unused)]

fn main(){

    let r = 1..=50;
    for number in r.clone().take(15).step_by(2).skip(1){
        println!("{number}");
    }
    
}
    



