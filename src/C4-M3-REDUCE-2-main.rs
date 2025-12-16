#![allow(unused)]

fn main(){

    let v = vec![2, 4, 5, 6, 7, 22];
    let res = v.iter().copied().reduce(|x, y| x + y);
    println!("{res:?}");

    let v = vec![];
    let res = v.iter().copied().reduce(|x:i32, y| x + y);
    println!("{res:?}");

}
    



