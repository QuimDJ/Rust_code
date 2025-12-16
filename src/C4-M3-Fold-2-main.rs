#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
struct SupportStaff{
    day:String,
    employee:String,
}
fn main(){
    let week=[
        SupportStaff {
            day:String::from("Monday"),
            employee:String::from("Brian"),
        },
        SupportStaff {
            day:String::from("Tueday"),
            employee:String::from("Cameron"),
        },
        SupportStaff {
            day:String::from("Wednesday"),
            employee:String::from("Walter"),
        },
    ];

    let map= week.into_iter().fold(HashMap::new(),
    |mut data,entry:SupportStaff| 
    {
        data.insert(entry.day,entry.employee);
        data
    });

    println!("{map:?}");

}
    



