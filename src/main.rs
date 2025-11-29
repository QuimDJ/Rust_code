fn main() {
    let first_name=String::from("Alice");
    let last_name=String::from("Wonder Woman");

    let capture_string=move || {
        println!("{first_name} {last_name}");
    };

    capture_string();
    //println!("{first_name}");
    capture_string();
    //println!("{first_name}");

}
