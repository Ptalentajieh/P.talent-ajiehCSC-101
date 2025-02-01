use std::io::Read;
use std::io;
fn admin(){
 println!("You Have Chosen Option 1, Indicating You Are An Administrator");
 let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 print!("{}", contents);
}
fn prom(){
 println!("You Have Chosen Option 2, Indicating You Are A Project Manager");
 let mut file = std::fs::File::open("project_tb.sql").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 print!("{}", contents);
}
fn employ(){
 println!("You Have Chosen Option 3, Indicating You Are An Employee");
 let mut file = std::fs::File::open("staff_tb.sql").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 print!("{}", contents);
}
fn cust(){
 println!("You Have Chosen Option 4, Indicating You Are A Customer ");
 let mut file = std::fs::File::open("customer_tb.sql").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 print!("{}", contents);
}
fn vend(){
 println!("You Have Chosen Option 5, Indicating You Are A Vendor");
 let mut file = std::fs::File::open("datap_tb.sql").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 print!("{}", contents);
}
fn main() {
 println!("************************************************************");
 println!("Welcome to the database management software of Globacom Ltd");
 println!("*******************");
 println!("What is your level of access?");
 println!("*************************************************************");
 println!("");
 println!("1:- Administrator");
 println!("2:-Project Manager");
 println!("3:-Employee");
 println!("4:-Customer");
 println!("5:-Vendor");
 println!("");
 let mut _reply = String::new();
 io::stdin().read_line(&mut _reply).expect("Not a valid string");
 let reply:f64 = _reply.trim().parse().expect("Not a valid number");
 if reply == 1.0 {
    admin();
 }
 if reply == 2.0 {
    prom();
 }
 if reply == 3.0 {
    employ();
 }
 if reply == 4.0 {
    cust();
 }
 if reply == 5.0 {
    vend(); 
 } else {
   return;
 }

}
