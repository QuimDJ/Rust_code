#![allow(unused)]

fn main(){

    let v = vec![2, 4, 5, 6, 7, 22];
    // Muy interesante el concepto de que Rust infiere tipos
    // apartir de la restricciones de tipo o de traits, por eso
    // que demos valores a v no ayuda al compilador
    // que la declaración que sigue para sum() es:
    //fn sum<S>(self) -> S
    //where
    //    S: Sum<Self::Item>
    // donde S es un genérico que puede ser i32,i64,f32,f64, f128
    // por ello se ha de especificar res:i32 o cualquier otro tipo para
    // EVITAR EL ERROR DE COMPILACIÓN
    let res:i128 = v.iter().sum();
    println!("{res:?}");

}
    



