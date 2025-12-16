#![allow(unused)]

use std::collections::HashMap;
#[derive(Debug)]
struct Texte {
    content: String,
}
impl Texte{
    fn conta(&self) -> u32 {
        let txt=&self.content;
        let mut contador:u32=0;
        for word in txt.split_ascii_whitespace(){
            contador +=1;
        }
        contador
    }
    fn repetitions(&self, h: &mut HashMap<String,u32>) {
        let t=&self.content;
        for word in t.split_ascii_whitespace(){
            //println!("{word:?}");
            if h.contains_key(word){
                *h.entry(word.to_string()).or_insert(0) += 1;
            } else {
                h.insert(word.to_string(),1);
            }
            
        }
        //println!("{:?}",h.keys());
        
    }
}

fn main() {
/*
    let mut hsm= HashMap::new();
    hsm.insert(0,(1,2));
    hsm.insert(1,(3,5));
    hsm.insert(2,(5,8));
    println!("{:?}", hsm);
    for m in hsm.values_mut() {
        *m=(0,0);
        //println!("{:?} -> {:?}", k, m);
    }
    println!("{:?}",hsm);
    
    let d=String::from("Texte_paràgraf");
    let d1=&d;
    for k in d1.chars() {
        println!("{:?}", k);
    }*/

    let mut contaW:HashMap<String,u32>=HashMap::new();
    let mut t=Texte{content:"In this lesson, let's solve a classic programming problem that involves iteration. Let's write a function that will accept a sentence and count the number of times that each word in the sentence occurs, we can store the results in a HashMap where the keys are going to be the words and the values are going to be their counts. So above main, let's declare our function. It's going to be called count words and it will accept a single parameter called text which I will make a string slice, a ref stir and it's going to return a HashMap and that hash needs two concrete types for the generics. So our keys are going to be the words themselves and those will be string slices and the values will be their counts. And I'm going to model those as U 30 twos because the counts can only be positive. And in order to refer to this HashMap type, we of course need to bring it into scope.".to_string()};
    println!(" Texte: {:?}",t);
    t.content = t.content.chars().filter(|c| !c.is_ascii_punctuation()).collect();
    println!(" nº de paraules: {}",t.conta());
    t.repetitions(&mut contaW);
    let mut vec: Vec<(String, u32)> = contaW.into_iter().collect();

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:?}", vec);
    
    
    
}


