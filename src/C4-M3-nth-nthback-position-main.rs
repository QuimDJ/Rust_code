#![allow(unused)]

fn main(){

    let v = vec![2, 4, 5, 6, 7, 22];
    let res = v.iter().nth_back(0);
    println!("{res:?}");
    let res = v.iter().nth(1);
    println!("{res:?}");

    let res = v.iter().position(|elem|*elem==5);
    println!("{res:?}, {:?}",v[res.unwrap()]);
}
    



