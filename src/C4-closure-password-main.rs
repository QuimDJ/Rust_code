use std::io::{read_to_string, stdin};
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
    let user=Vault{password:String::from("2345H"),treasure: String::from("500")};
    let mut user_input=String::new();
    stdin().read_line(&mut user_input);
    user_input = user_input.trim().to_string();
    let hack=|| user_input;
    let extraction=user.unlock(hack);
}
