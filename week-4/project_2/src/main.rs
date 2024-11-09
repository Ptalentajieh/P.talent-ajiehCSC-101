use std::io;


 fn main() {
     let mut experience = String::new();
          let mut age = String::new();
    println!("Is the employee experienced? (yes/no): ");
     io::stdin().read_line(&mut experience).expect("Not a valid number");
     
      let experience = experience.trim().to_lowercase(); println!("Enter the employee's age: ");
       io::stdin().read_line(&mut age).expect("Not a valid number");

        let age: u32 = age.trim().parse().expect("Please enter a valid number");
         let incentive = if experience == "yes" {
          if age >= 40 { 1_560_000 
          } else if age >= 30 { 
            1_480_000 
          } else if age < 28 { 
            1_300_000 * 12 
          } else {
           0
          }
          } else {100_000};

println!("The annual incentive for the employee is: {}", incentive);

 }
         