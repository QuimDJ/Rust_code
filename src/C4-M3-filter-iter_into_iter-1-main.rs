#![allow(unused)]

fn main(){
    let numbers=vec![4,8,1,16,23,42];
    println!("{numbers:?}");
    
    // Aqui el iterador no consume el vector.
    let z1:Vec<i32> = numbers.iter().filter(|x| **x%2==0).copied().collect();
    println!("{z1:?}");

    // Aqui el iterador consume el vector.
    let z:Vec<i32> = numbers.into_iter().filter(|x| *x%2==0).collect();
    println!("{z:?}");
}
    



