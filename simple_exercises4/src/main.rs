enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

union Number {
    small: u8,
    normal: u32,
    big: u64
}

fn main() {
    let c: Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r, g, b),
        _ => (),
    }


    let mut num = Number { normal: 1000 };

    unsafe {
        println!("u32: {}",num.normal);

        num.big = 123456789;
        println!("u64: {}",num.big);
    }


    let x = 3.0;
    let y = 0.0;

    let result =
    if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(res) => println!("{}",res),
        None => println!("cannot divide by zero"),
    }

    if let Some(z) = result {
        println!("{}", z);
    }

    let mut chars = "hello".chars();

    while let Some(c) = chars.next() {
        println!("the charecter is {}",c);
    }
}
