fn main() {
    let s:&'static str = "hello there!";
    println!("{}", s); 

    for c in s.chars() {
        println!("{}",c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("{}", first_char);
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a<=('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    //let z = letters + "abc";
    let z = letters.clone() + &letters;
    println!("{}",z);

    let mut abc = String::from("deneme");
    let mut ab = "deneme".to_string();
    ab.remove(0);
    ab.push_str("!!!");
    println!("{}",ab.replace("eneme", "deneme"));


    let name = "Emirhan";
    let greeting = format!("hello im {}, nice to meet you",name);
    println!("{}",greeting);

    let run = "run";
    let forest = "forest";
    let run_forest = format!("{0} {1} {0}", run, forest);
    println!("{}", run_forest);

    let info = format!(
        "my name is {last}, {first} {last}",
        first = "james",
        last = "bond"
        );

println!("{}", info);
}
