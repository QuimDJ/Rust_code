#![allow(unused)]

fn main(){

    let v = vec![2, 4, 5, 6, 7, 22];
    let res = v.iter().copied().reduce(|x, y| x + y).unwrap_or(0);
    println!("{res}");

}
    



