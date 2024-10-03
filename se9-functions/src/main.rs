fn increase(x: &mut i32) {
    *x += 10;
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn closures() {
    let a = 6;
    let plus_one = |x: i32| -> i32 { x + 1 };
    println!("{} plus one equal to {}",a ,plus_one(a));

    let plus_two = |x| x + 2;
    println!("{} + 2 = {}",a ,plus_two(a));
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}


fn main() {
    let mut a = 5;
    increase(&mut a);
    println!("{}", a);

    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let myLine = Line { start: p, end: p2 };

    println!("length = {}",myLine.len());

    closures();

    let greater_than_10 = greater_than(10);
    
    
    println!("{}", greater_than_10(15));
    println!("{}", greater_than_10(5)); 
}
