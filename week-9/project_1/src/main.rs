use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    l();
    st();
    na();
}

fn na() {
    let na1 = "Maltina";
    let na2 = "Amstel Malta";
    let na3 = "Malta Gold";
    let na4 = "Fayrouz";
   

    
    file.write_all("Non-Alcoholic:\n".as_bytes())
         .expect("write failed");
         file.write_all(na1.as_bytes()).expect("write failed");
         file.write_all(na2.as_bytes()).expect("write failed");
         file.write_all(na3.as_bytes()).expect("write failed");
         file.write_all(na4.as_bytes()).expect("write failed");
         println!("Premium categories of Non-Alcoholic drinks");
}


fn st(){
    let s1 = "legend";
    let s2 = "Turbo King";
    let s3 = "Williams";

    file.write_all("Stout:\n".as_bytes())
         .expect("write failed");
         file.write_all(s1.as_bytes()).expect("write failed");
         file.write_all(s2.as_bytes()).expect("write failed");
         file.write_all(s3.as_bytes()).expect("write failed");
         println!("Premium categories of drinks classified as Stout");
}




fn l() {
 let a = "33 Export";
    let b = "Desperados";
    let c = "Goldberg";
    let d = "Gulder";
    let e = "Heineken";
    let f = "Star";

    file.write_all("Lager:\n".as_bytes())
         .expect("write failed");
         file.write_all(a.as_bytes()).expect("write failed");
         file.write_all(b.as_bytes()).expect("write failed");
         file.write_all(c.as_bytes()).expect("write failed");
         file.write_all(d.as_bytes()).expect("write failed");
         file.write_all(e.as_bytes()).expect("write failed");
         file.write_all(f.as_bytes()).expect("write failed");
         println!("Premium categories of drinks classified as lager");
}
