use std::io::stdin;
#[derive(Debug)]
struct Vault {
    password:String,
    treasure: String,
}
impl Vault {
    
    fn unlock<F>(self, procedure: F) -> Option<String>
    where F: FnOnce() -> String {
        let user_password= procedure();
        if user_password == self.password {
            Some(self.treasure)
        }else{
            None
        }

    }

}

fn main() {
    
    let user=Vault{password:String::from("22"),treasure: String::from("GOLD MINE")};
    let mut user_input=String::new();
    println!("Please, enter the password to open the treasure:");
    stdin().read_line(&mut user_input);
    user_input = user_input.trim().to_string();
    let hack=|| user_input;
    let extraction=user.unlock(hack);
    match extraction {
        Some(_) => println!("User validat amb èxit!"),
        None => println!("Usuari rebutjat, password incorrecte!"),
    }
    
    
}
