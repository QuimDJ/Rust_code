#![allow(unused)]


fn main(){
    let v=vec!["Maria","Teresa","Juan","Pedro","Tamara","Toni"];
    let r:(Vec<&str>,Vec<&str>)=v.iter().partition(|x|{x[0..1].to_uppercase()=="T"});

    println!("{:?}",r.0);
    println!("{:?}",r.1);
    let zp=r.0.into_iter().zip(r.1.into_iter());
    let res:Vec<(&str,&str)>=zp.collect();
    println!("{:?}",res);
    
    
}
    



