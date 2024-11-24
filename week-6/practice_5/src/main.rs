fn main() {
    let fullname = "Pan-Atlatic University";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before trim");
    println!("Length is {}",fullname.len());
    println!();
    println!("After trim");
    println!("Length is {}",fullname.trim().len());
}
