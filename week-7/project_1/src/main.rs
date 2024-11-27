use std::io;

fn a_o_t() {
    println!("Input value for height");
    let mut z = String::new();
   io::stdin().read_line(&mut z).expect("Invalid input");
    let _z:i32 = z.trim().parse().expect("Invalid input"); 


    println!("Input value for base 1");
    let  mut x = String::new();
    io::stdin().read_line(&mut x).expect("Invalid input");
    let _x:i32 = x.trim().parse().expect("Invalid input"); 
    

    println!("Input value for base 2");
    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("Invalid input");
    let _v:i32 = v.trim().parse().expect("Invalid input"); 


     
    let first_1 = _z/2;
    let second_1 = _x + _v;

    let aot = first_1 * second_1;
    println!("Area of Trapezium is: {}", aot);
    
}

fn  a_o_r(){
    println!("Input value for Diagonal 1");
    let mut d1 = String::new();
   io::stdin().read_line(&mut d1).expect("Invalid input");
    let _d1:i32 = d1.trim().parse().expect("Invalid input"); 

    println!("Input value for Diagonal 2");
    let mut d2 = String::new();
   io::stdin().read_line(&mut d2).expect("Invalid input");
    let _d2:i32 = d2.trim().parse().expect("Invalid input");


    let aor = _d1 * _d2/2;

    println!("Area of rhombus is: {}", aor); 
}

fn a_o_p(){
    println!("Input value for base");
    let mut b = String::new();
   io::stdin().read_line(&mut b).expect("Invalid input");
    let _b:i32 = b.trim().parse().expect("Invalid input");

    println!("Input value for altitude");
    let mut al = String::new();
   io::stdin().read_line(&mut al).expect("Invalid input");
    let _al:i32 = al.trim().parse().expect("Invalid input");

    let aop = _al * _b;
    println!("Area of parallelogram is: {}", aop);  
}
fn a_o_c(){
    println!("Input length of sides");
    let mut l = String::new();
   io::stdin().read_line(&mut l).expect("Invalid input");
    let _l:i32 = l.trim().parse().expect("Invalid input"); 


   let aoc = 6 * _l *_l;
   println!("Area of cube is {}", aoc);

}
fn v_o_c(){
    println!("Input value for radius of cylinder");
    let mut r = String::new();
   io::stdin().read_line(&mut r).expect("Invalid input");
    let _r:f64 = r.trim().parse().expect("Invalid input"); 

    println!("Input value for height");
    let mut h = String::new();
   io::stdin().read_line(&mut h).expect("Invalid input");
    let _h:f64 = h.trim().parse().expect("Invalid input"); 
 
    let m:f64 = 3.142;
    let voc = m * _r * _r * _h;
    println!("Volume of Cylinder is: {}", voc);
}
fn main() {
   let mut input = String::new();
   println!("Input:- Calculation ");
   println!("1:-Area of Trapezium" );
   println!("2:-Area of rhombus");
   println!("3:-Area of parallelogram");
   println!("4:-Area of cube");
   println!("5:-Volume of Cylinder");
   io::stdin().read_line(&mut input).expect("Invalid input");
   let response:i32 = input.trim().parse().expect("Invalid input");

   if response == 1 {
    println!("Area of Trapezium");
    a_o_t();
   }
   if response == 2 {
    println!("Area of rhombus");
    a_o_r();
   }
   if response == 3 {
    println!("Area of parallelogram");
    a_o_p();
   }
   if response == 4{
    println!("Area of cube");
    a_o_c();
   }
   if response == 5  {
       println!("Volume of Cylinder");
       v_o_c();
   }
}
