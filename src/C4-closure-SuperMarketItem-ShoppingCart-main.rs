#![allow(unused)]
#[derive(Debug)]
struct SuperMarketItem{
    name:String,
    price:f64,
}
#[derive(Debug)]
struct ShoppingCart{
    items:Vec<SuperMarketItem>,
}
impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F) 
    where F: FnMut(&mut SuperMarketItem),
    {
        let final_index=self.items.len()-1;
        let mut current_index=0;
        while current_index <= final_index {
            let current_item = &mut self.items[current_index];
            operation(current_item);
            current_index +=1;
        } 
    }
    fn checkout<F>(self, operation:F) 
    where F:FnOnce(Self),
    { operation(self); }
}
fn main() {
    let mut item1=SuperMarketItem{name:"APPLE".to_string(), price:3.99};
    let mut item2=SuperMarketItem{name:"BANANA".to_string(), price:2.99};
    let mut sp1=ShoppingCart{ items:vec![item1, item2],};
    println!("{sp1:?}");
    sp1.traverse_items( |item: &mut SuperMarketItem| { item.price*=0.85; });
    println!("{sp1:?}");
    sp1.traverse_items(|item: &mut SuperMarketItem| 
        { item.name=item.name.to_lowercase(); });
    println!("{sp1:?}");
    let mut total_price=0.0;
    sp1.checkout(|mut cart| {
        println!("{cart:?}");
        cart.traverse_items(|x:&mut SuperMarketItem|{
            total_price+=x.price;
        });
    });
    println!("Total price= {total_price:.2} €");
}