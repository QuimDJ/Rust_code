#![allow(unused)]

fn main() {

    let v=vec![1,2,3,4];
    let my_iterator=v.into_iter();
    println!("{my_iterator:?}");
    for m in my_iterator { println!("{m}");};
    // hemos consumido el vector con la iteración println!("{v:?}"); da borrow

    let v=vec![1,2,3,4];
    let my_iterator=v.iter();
    println!("{my_iterator:?}");
    for m in my_iterator { println!("{m}");};
    println!("{v:?}"); // con iter() no consumimos el vector pero es immutable

    let mut v=vec![1,2,3,4];
    let my_iterator=v.iter_mut();
    println!("{my_iterator:?}");
    for m in my_iterator { 
        *m=5; 
        println!("{m}"); 
    };
    println!("{v:?}"); // iteramos y ademas modificamos usando deferenciación

    // into_iter() consume el vector.
    // iter() es referencia immutable, no consume el vector.
    // iter_mut() es referencia mutable, permite modificar el vector.

}



