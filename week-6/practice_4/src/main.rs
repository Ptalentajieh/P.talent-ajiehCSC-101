fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlatic UNiversity";

    let mut school = "School of Science".to_string();
    school.push_str(" and Technology");

    println!("My name is: {}", fullname);
    println!("The length my fullname is: {}",fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);

}
