use std::io;

fn prime() -> (String, String, i32) {
    println!("Enter your full name");
    println!("==>");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    name.trim().to_string();

     println!("Where do you work?");
     println!("==>");
     let mut w = String::new();
    io::stdin().read_line(&mut w).expect("Failed to read input");
    w.trim().to_string();
    let w = if w.is_empty() { "Self-employed".to_string() } else { w };

    println!("How many months/years of programming experience do you have? ");
    println!("==>");
    let mut ex = String::new();
    io::stdin().read_line(&mut ex).expect("Failed to read input");
    let _ex:i32 = ex.trim().parse().expect("Not a valid number");
    (name, w, _ex)


}

fn main(){
    let mut info_list: Vec<(String, String, i32)> = Vec::new();

    for _ in 0..3 {
        info_list.push(prime());
    }

    let max_experience = info_list.iter().max_by_key(|&(_, _, exp)| exp).unwrap().2;
    let most_experienced = info_list.iter().filter(|&&(_, _, exp)| exp == max_experience).collect::<Vec<_>>();

    println!("\nMost Experienced Programmer(s):");
    for d in most_experienced {
        println!("{:?}", d);
    }
}