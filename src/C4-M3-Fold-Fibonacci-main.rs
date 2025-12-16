#![allow(unused)]


fn main(){
    let mut v:Vec<i128>=vec![1,1];
    let max_num=15;
    for i in 2..max_num {
        v.insert(i, v[i-2]+v[i-1]);
    }
    println!("Sèrie de Fibonacci ({max_num} elements): {v:?}");

    let sum=v.iter().fold(0,|acc,x|{acc+x});
    println!("Suma Total= {sum}");

}
    



