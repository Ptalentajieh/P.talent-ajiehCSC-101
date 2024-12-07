use std::io;
fn oa() {
    println!("");
    println!("");
    let oaj: (&str, &str, &str, &str, &str, &str) = ("Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO");

    println!("Note: 1 should be subtracted from orginal years of experience");
    println!("And placed in the system");
    println!("Enter Year(s) of Experience between (0 - 15)");
    let mut v = String::new();
    println!("You have {} year(s) of experience", v);
    std::io::stdin().read_line(&mut v).expect("Failed to read input");
    let _v:i32 = v.trim().parse().expect("Invalid input");


    if _v >=1 && _v <=2     {
        println!("You are an {:?} in the Office Administrator department", oaj.0);
        println!("With an APS level between (1 - 2)");
    }
    if _v >=3 && _v <5    {
        println!("You are an {:?} in the Office Administrator department", oaj.1);
        println!("With an APS level between (3 - 5)");
    }
    if _v >=5 && _v <8  {
        println!("You are an {:?} in the Office Administrator department", oaj.2);
        println!("With an APS level between (5 - 8)");
    }
    if _v >=8 && _v <10  {
        println!("You are an {:?} in the Office Administrator department", oaj.3);
        println!("With (8 - 10) Years of Experience which makes you ");
        println!("A Member of the Executive Level 1");
    }
    if _v >=10 && _v <=13   {
        println!("You are an {:?} in the Office Administrator department", oaj.4);
        println!("With (10 - 13) Years of Experience which makes you ");
        println!("A Member of the Executive Level 2");
    }
    if _v >13    {
        println!("You are an {:?} in the Office Administrator department", oaj.5);
        println!("With at least 13 Years of Experience which makes you to become a ");
        println!("Member of the Senior Executive Service");
    }

}


fn a(){
println!("");
    println!("");
    let aj: (&str, &str, &str, &str, &str, &str) = ("–", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean");

   println!("Note: 1 should be subtracted from orginal years of experience");
    println!("And placed in the system");
    println!("Enter Year(s) of Experience between (0 - 15)");
    let mut n = String::new();
    println!("You have {} year(s) of experience", n);
    std::io::stdin().read_line(&mut n).expect("Failed to read input");
    let _n:i32 = n.trim().parse().expect("Invalid input");


    if _n >=1 && _n <=2     {
        println!("You are an {:?} in the Office Administrator department", aj.0);
        println!("With an APS level between (1 - 2)");
    }
    if _n >=3 && _n <5    {
        println!("You are an {:?} in the Academic department", aj.1);
        println!("With an APS level between (3 - 5)");
    }
    if _n >=5 && _n <8    {
        println!("You are an {:?} in the Academic department", aj.2);
        println!("With an APS level between (5 - 8)");
    }
    if _n >=8 && _n <10   {
        println!("You are an {:?} in the Academic department", aj.3);
        println!("With (8 - 10) Years of Experience which makes you ");
        println!("A Member of the Executive Level 1");
    }
    if _n  >=10 && _n <=13   {
        println!("You are an {:?} in the Academic department", aj.4);
        println!("With (10 - 13) Years of Experience which makes you ");
        println!("A Member of the Executive Level 2");
    }
    if _n >13   {
        println!("You are an {:?} in the Academic department", aj.5);
        println!("With at least 13 Years of Experience which makes you to become a ");
        println!("Member of the Senior Executive Service");
    }
}


fn l(){
  println!("");
    println!("");
     let lj: (&str, &str, &str, &str, &str, &str) = ("Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner");

    println!("Note: 1 should be subtracted from orginal years of experience");
    println!("And placed in the system");
    println!("Enter Year(s) of Experience between (0 - 15)");
    let mut m = String::new();
    println!("You have {} year(s) of experience", m);
    std::io::stdin().read_line(&mut m).expect("Failed to read input");
    let _m:i32 = m.trim().parse().expect("Invalid input");


    if _m >=1 && _m <=2     {
        println!("You are an {:?} in the Department of Legal Professionals", lj.0);
        println!("With an APS level between (1 - 2)");
    }
    if _m >=3 && _m <5    {
        println!("You are an {:?} in the Department of Legal Professionals", lj.1);
        println!("With an APS level between (3 - 5)");
    }
    if _m >=5 && _m <8    {
        println!("You are an {:?} in the Department of Legal Professionals", lj.2);
        println!("With an APS level between (5 - 8)");
    }
    if _m >=8 && _m <10   {
        println!("You are an {:?} in the Department of Legal Professionals", lj.3);
        println!("With (8 - 10) Years of Experience which makes you ");
        println!("A Member of the Executive Level 1");
    }
    if _m  >=10 && _m <=13   {
        println!("You are an {:?} in the Department of Legal Professionals", lj.4);
        println!("With (10 - 13) Years of Experience which makes you ");
        println!("A Member of the Executive Level 2");
    }
    if _m >13   {
        println!("You are an {:?} in the Department of Legal Professionals", lj.5);
        println!("With at least 13 Years of Experience which makes you to become a ");
        println!("Member of the Senior Executive Service");
    }
}

fn t(){
  println!("");
    println!("");
     let tj: (&str, &str, &str, &str, &str, &str) = ("Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal");

    println!("Note: 1 should be subtracted from orginal years of experience");
    println!("And placed in the system");
    println!("Enter Year(s) of Experience between (0 - 15)");
    let mut k = String::new();
    println!("You have {} year(s) of experience", k);
    std::io::stdin().read_line(&mut k).expect("Failed to read input");
    let _k:i32 = k.trim().parse().expect("Invalid input");


    if _k >=1 && _k <=2     {
        println!("You are an {:?} in the Teaching department", tj.0);
        println!("With an APS level between (1 - 2)");
    }
    if _k >=3 && _k <5    {
        println!("You are an {:?} in the Teaching department", tj.1);
        println!("With an APS level between (3 - 5)");
    }
    if _k >=5 && _k <8    {
        println!("You are an {:?} in the Teaching department", tj.2);
        println!("With an APS level between (5 - 8)");
    }
    if _k >=8 && _k <10   {
        println!("You are an {:?} in the Teaching department", tj.3);
        println!("With (8 - 10) Years of Experience which makes you ");
        println!("A Member of the Executive Level 1");
    }
    if _k  >=10 && _k <=13   {
        println!("You are an {:?} in the Teaching department", tj.4);
        println!("With (10 - 13) Years of Experience which makes you ");
        println!("A Member of the Executive Level 2");
    }
    if _k >13   {
        println!("You are an {:?} in the Teaching department", tj.5);
        println!("With at least 13 Years of Experience which makes you to become a ");
        println!("Member of the Senior Executive Service");
    }
}

fn main() {
 let mut i1 = String::new();
  println!("What Public Service Professional Are You?");
  println!("Number to insert:- Public Service Profession");
  println!("1:- Office Administrator");
  println!("2:- Academic");
  println!("3:- Lawyer");
  println!("4:- Teacher");
  std::io::stdin().read_line(&mut i1).expect("Failed to read input");
  let _i1:i32 = i1.trim().parse().expect("Invalid input");


  if _i1 == 1 {
    println!("You are an Administrative Professional(Office Administrator)");
    oa();
   }
   if _i1 == 2 {
    println!("You are an Academic Professional");
    a();
   }
   if _i1 == 3 {
    println!("You are a Legal Professional(A Lawyer)");
    l();
   }
   if _i1 == 4{
    println!("You are a Teaching Professional");
    t();
   }
   
}
