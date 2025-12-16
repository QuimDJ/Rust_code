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
    fn conta_paraules(&self, h: &mut HashMap<String,u32>) {
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

    let vtxt="In this lesson, let's solve a classic programming problem that involves iteration. 
    Let's write a function that will accept a sentence and count the number of times that each word in the sentence 
    occurs, we can store the results in a HashMap where the keys are going to be the words and the values are going 
    to be their counts. So above main, let's declare our function. It's going to be called count words and it will 
    accept a single parameter called text which I will make a string slice, a ref stir and it's going to return a 
    HashMap and that hash needs two concrete types for the generics. So our keys are going to be the words themselves 
    and those will be string slices and the values will be their counts. And I'm going to model those as U 30 twos 
    because the counts can only be positive. And in order to refer to this HashMap type, we of course need to bring 
    it into scope. It's going to be a very common pattern when we're dealing with methods that we invoke on the iterator 
    type itself. So I'm going to pass a closure to my for each method and what the for each method is going to give 
    every time it invokes the closure is going to be each successive element in the iterator. In other words, in this 
    scenario, each word. So basically what we have expressed right here is exactly what's happening each time we are 
    going to be invoking the closure. More specifically, it is the internals of the for each method that are going to 
    take care of auto invoking that closure, we just pass in the recipe to run for every word. That's why the method 
    name is for each, we are specifying the logic to run for each word in words, right. So here I have my word and then 
    what I want to do here is take my word and perform the exact same logic I have below.".to_string();
    let mut t=Texte{content:vtxt};
    println!(" Texte: {:?}",t);
    
    let mut contaW:HashMap<String,u32>=HashMap::new();
    
    t.content = t.content.chars().filter(|c| !c.is_ascii_punctuation()).collect();
    println!(" nº de paraules: {}",t.conta());
    t.conta_paraules(&mut contaW);
    let mut vec: Vec<(String, u32)> = contaW.into_iter().collect();

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:?}", vec);
    
    
    
}


