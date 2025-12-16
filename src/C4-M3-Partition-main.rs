#![allow(unused)]


fn main(){
    let v=vec![1,2,30,40];
    let r:(Vec<i32>,Vec<i32>)=v.clone().into_iter().partition(|x| *x%2==0);
    println!("vector inicial: {v:?}, vector final: {r:?}");
}
    



