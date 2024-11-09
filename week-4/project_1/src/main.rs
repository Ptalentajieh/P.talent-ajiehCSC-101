use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please insert the value of a");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f64 = a.trim().parse().expect("Not a valid number");

    println!("Please insert the value of b");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f64 = b.trim().parse().expect("Not a valid number");

    println!("Please insert the value of c");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f64 = c.trim().parse().expect("Not a valid number");

let z = b * b - 4.0 * a * c;

if z > 0.0 {
 let root1 = (-b + z.sqrt()) / (2.0 * a); 
 let root2 = (-b - z.sqrt()) / (2.0 * a);
  println!("The roots are {} and {}", root1, root2);
   } else if z == 0.0
    {
     let root = -b / (2.0 * a);
    println!("The root is {}", root);
} else {
    
    println!("The equation has no real roots.");
}

   
}



