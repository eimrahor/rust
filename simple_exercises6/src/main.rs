mod pm;
use std::mem;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Point<T,V> {
    x: T,
    y: V
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    let idx: usize = 0;
    println!("{}", a[idx]);

    // options 
    match a.get(2) {
        Some(x) => println!("{}",x),
        None => println!("none")
    }

    for x in &a { println!("{}", x); }

    a.push(44);
    println!("{:?}",a);

    //let last_element = a.pop();
    while let Some(element) = a.pop() {
        println!("{:?}",element)
    }
}

fn shapes()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);
    shapes.insert("circle".into(), 10);
    println!("{:?}",shapes);

    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}",shapes);

    {
        let actual = shapes
        .entry("circle".into())
        .or_insert(2);
        *actual = 1;
    }

    println!("{:?}",shapes);
}

fn hash_sets()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamme");
    greeks.insert("delta");
    println!("{:?}",greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("added vega");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //subset
    println!("{}",_2_8.is_subset(&_1_10));

    //disjoint
    println!("{}",_1_5.is_disjoint(&_6_10));

    //intersection
    println!("{:?}",_2_8.intersection(&_6_10));

    //union
    println!("{:?}",_2_8.union(&_6_10));

    //difference
    println!("{:?}",_6_10.difference(&_2_8));
}

fn main() {
    pm::pattern_matching();

    let a: Point<i8,i8> = Point { x:5, y:4};
    println!("a = {:?} and a.x = {}, a.y = {}",a, a.x, a.y);
    println!("memory of a = {} bytes", mem::size_of_val(&a));


    vectors();
    shapes();
    hash_sets();

    let vec = vec![3,2,1];

    for x in &vec {
        println!("{}",*x);
    }

    for x in vec.iter()
    {
        println!("{}",*x);
    }

    let mut vec2 = vec![3,2,1];

    for x in vec2.iter_mut() 
    {
        *x*=2;
    }
    println!("{:?}",vec2);

    for x in vec2.iter().rev()
    {
        println!("{}",*x);
    }

    let it = vec.into_iter();
    vec2.extend(it);            // it variable is disseappered
    println!("{:?}",vec2);
}
