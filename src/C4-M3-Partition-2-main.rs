#![allow(unused)]


fn main(){
    let v=vec!["Maria","Teresa","Juan","Pedro","Tamara","Toni"];
    let r:(Vec<&str>,Vec<&str>)=v.iter().partition(|x|{x[0..1].to_uppercase()=="T"});

    println!("{v:?}\n Empiezan o no por T: {r:?}");
}
    



