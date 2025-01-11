struct Laptop { 
    brand:String,
    price:u32
}
impl Laptop {
    fn cost(quantity:u32, price:u32) -> u32 {
        quantity * price
    }
}
fn main() {
let hp = Laptop {
brand: String::from("HP"),
price: 650_000,
};
let ibm = Laptop {
brand: String::from("IBM"),
price: 755_000,
}; 
let toshiba = Laptop {
brand: String::from("Toshiba"),
price: 550_000,
};
let dell = Laptop {
brand: String::from("Dell"),
price: 850_000, };

let hpcost = Laptop::cost(3, hp.price);
let ibmcost = Laptop::cost(3, ibm.price);
let toshibacost = Laptop::cost(3,toshiba.price);
let dellcost = Laptop::cost(3, dell.price);

let total_cost= hpcost+ibmcost+toshibacost+dellcost;

println!("The Cost of purchasing 3 Laptops of each branch is {}", total_cost);
}


