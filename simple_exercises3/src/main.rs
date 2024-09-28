fn main() {
    let day = if true { "sunny"} else { "rainy"};
    println!("{}",day);


    let mut y = 1;
    loop {
        y *= 2;
        println!("{}",y);
        if y == 1<<10 { break };
    }

    for x in 1..11 {
        println!("{}",x);
    }

    for (pos, y) in (10..31).enumerate() {
        println!("{} {}",pos, y);
    }


    let country_code = 2000;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        90 => "Turkey",
        1 ..= 1000 => "unknown",
        _ => "invalid"
    };

    println!("{} is {}",
            country_code, country);
}
