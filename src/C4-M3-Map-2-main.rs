#![allow(unused)]

fn main(){
    let numbers=vec![4,8,1,16,23,42];
    for number in numbers.into_iter().map(|x:i32| x.pow(2)){
        println!("{number:?}");
    }
    //println!("{numbers:?}");
}


