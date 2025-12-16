#![allow(unused)]

fn main(){
    let numbers=vec![4,8,1,16,23,42];
    let numbers_f=vec![4.2,8.9,1.3,16.4,23.99,42.09];
    println!("{numbers:?}");
    
    // Aqui el iterador no consume el vector. 
    // si usamos Vec<_> sirve para cualquier tipo numérico de vector
    let z1:Vec<_> = numbers_f.iter().filter(|x| **x<10.0).copied().collect();
    println!("Menores que 10.0 :{z1:?}");

    // Aqui el iterador consume el vector.
    let z:Vec<i32> = numbers.into_iter().filter(|x| *x%2==0).collect();
    println!("Parells: {z:?}");
}
    



