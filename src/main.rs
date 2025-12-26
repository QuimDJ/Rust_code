#![allow(unused)]


fn main(){

    let v=vec![1,2,3,4,5];
    for i in v.iter(){
        println!("{i}");
    }
    println!("{:?}",v.iter().enumerate());
    for (i,num) in v.iter().enumerate(){
        println!("indice:{i}  valor:{num}");  
    }

    for k in v.iter().enumerate(){
        println!("{:?}", k);  
    }
    // transformació: multiplicar por 2
    let numeros=vec![1,2,3,4,5];
    let n2:Vec<i32>= numeros.iter().map(|x| x*2).collect();
    println!("{n2:?}");
    let n3=vec![2,4,6,8,10];
    // Filtrar parells
    let parells:Vec<&i32>=n3.iter().filter(|&x| x%2==0).collect();
    println!("{parells:?}");
    let result:Vec<i32>=numeros.iter().filter(|&x| x > &2).map(|x| x*3).collect();
    println!("{result:?}");

    // consumidors
    let m:i32=numeros.iter().sum();
    println!("{:?}", m);
}
    



