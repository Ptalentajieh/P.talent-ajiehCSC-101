use std::io;

fn main() {

   println!("Menu:");
    println!("Type of food=>price ");
     println!("P: Poundo Yam/Edinkaiko Soup => N3,200");
      println!("F: Fried Rice & Chicken => N3,000");
       println!("A: Amala & Ewedu Soup => N2,500");
        println!("E: Eba & Egusi Soup => N2,000");
     println!("W: White Rice & Stew => N2,500");


    // meal_option was used in place of menu
    let meal_option = [
     ("Poundo Yam/Edinkaiko Soup", 3200),
      ("Fried Rice & Chicken", 3000),
       ("Amala & Ewedu Soup", 2500),
        ("Eba & Egusi Soup", 2000),
         ("White Rice & Stew", 2500),
          ];


     let mut f_t = String::new();
      let mut q = String::new();


      println!("Enter Food Type Initials ");
      io::stdin().read_line(&mut f_t).expect("Failed to read input");

      println!("Enter needed quantity:");
      io::stdin().read_line(&mut q).expect("Failed to read input");


      let _f_t = f_t.trim();
      let _q:i32 = q.trim().parse().expect("Invalid Quantity");


      let (food_name, p) = match _f_t { 
        "P" => meal_option[0],
         "F" => meal_option[1],
          "A" => meal_option[2],
           "E" => meal_option[3],
       "W" => meal_option[4],
       _ => {
        println!("Invalid food type");
        return;
       }
   };
     let y:i32 = 95/100;
    let t_c = p * _q;
    let x:i32 = (t_c as i32) * y;
    let f_c = if t_c > 10000 {
        x
    } else {
        t_c 
    };




    println!( "You ordered: {} Plate(s) of {}", _q, food_name );
     println!("Total cost: {}", t_c);
     println!("Final cost after Price break (if applicable) is: {}", f_c);





}
