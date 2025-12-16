#![allow(unused)]

fn main(){
    let names:[String;3]=[
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris")
    ];

    let name_lengths:Vec<usize>=names.iter()
    .map(|x| x.to_lowercase())
    .map(|x| { x.len()}).collect::<Vec<usize>>();

    println!("{name_lengths:?}");
}
    



