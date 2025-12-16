#![allow(unused)]


fn main(){
    let v=vec![1,2,30,40];
    let r:Vec<(&str)>=v.iter().enumerate()
    .flat_map(|(i,n)| {if n%2==0 {Some("Parell")} else{Some("Senar")}}).collect();
    println!("vector inicial: {v:?}, vector final: {r:?}");
}
    



