#![allow(unused)]

fn main(){

    let mut v=vec![1,2,3,4,5,6,7,8];
    println!("{:?} sorted? {}",v,v.is_sorted());
    v.reverse();
    println!("{:?}\n Is empty? {}\n Sorted? {}\n", v,v.is_empty(),v.is_sorted());

    
}
    



