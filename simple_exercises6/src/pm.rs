enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn how_many(x: i32) -> &'static str { // lifetimes
	match x {
		0 => "no",
		1 | 2 => "one or two",
		12 => "a dozen",
		z @ 9..11 => "lors of",
		_ if(x % 2 == 0) => "some",
		_ => "a few"
	}
}



pub fn pattern_matching() {
	for x in 0..13 {
		println!("{}: I have {} oranges", x, how_many(x));
	}

	let point = (3,7);

	match point {
		(0,0) => println!("origin"),
		(0,y) => println!("x axis, y = {}",y),
		(x,0) => println!("y axis, x = {}",x),
		(_,y) => println!("(?,{})",y)
	}

	let c: Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

	match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{black: 255, ..} => println!("black"), // .. other cases
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r, g, b),
        _ => (),
    }
}