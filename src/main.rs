fn main() {
    let option:Option<&str>=Some("Salami");
    let food=option.unwrap_or_else(|| "Pizza");
    println!("{}", food);

    let option:Option<&str>=None;
    let food=option.unwrap_or_else(|| "Pizza");
    println!("{}", food);
}
