#![allow(unused)]
use std::env;
use std::process;
use std::collections::HashMap;
#[derive(Debug, PartialEq,Eq,Hash)]

enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge
}
#[derive(Debug)]
struct CustomerOrder{
    product: Product,
    quantity:u32,
    shipped: bool,
}
impl CustomerOrder{
    fn new(product:Product, quantity:u32, shipped:bool) -> Self{
        Self{product, quantity, shipped}
    }
}
#[derive(Debug)]
struct Customer{
    id:u32,
    orders:Vec<CustomerOrder>,
}
fn main(){

    let o1=CustomerOrder::new(Product::Blender,3,false);
    let o2=CustomerOrder{product:Product::Toaster,quantity:2,shipped:false};
    let o3=CustomerOrder::new(Product::Fridge,10,false);
    let o4=CustomerOrder{product:Product::Blender,quantity:1,shipped:false};
    let o5=CustomerOrder::new(Product::Microwave,1,true);
    let o6=CustomerOrder{product:Product::Microwave,quantity:5,shipped:true};
    
    let mut orders:Vec<CustomerOrder>=vec![o1,o2,o3,o4,o5,o6];

    let customers_ids_by_order=[2,1,2,3,4,1];

    // let product_quantities= orders.iter()
    // .filter(|order|order.shipped==false)
    // .fold(HashMap::new(),|mut data, order|{
    //     let entry=data.entry(&order.product).or_insert(0);
    //     *entry += order.quantity;
    //     data
    // });
    // println!("{:#?}",product_quantities);

    let mut customers= 
    orders.into_iter()
    .zip(customers_ids_by_order)
    .fold(HashMap::new(),
    |mut ids_to_orders,(order,customer_id)| 
    {
        let mut orders=ids_to_orders.entry(customer_id)
        .or_insert(vec![]);
        orders.push(order);
        ids_to_orders
    }).into_iter()
    .map(|(id,orders)| Customer {id, orders})
    .collect::<Vec<Customer>>();

    customers.sort_by_key(|customer| customer.id);

    println!("{:#?}",customers);

}
    



