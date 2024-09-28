use std::mem;

fn use_slice(slice: &mut[i32]) {
    println!("first element: {}", slice[0]);
    slice[0] = 152;
    println!("first element: {}", slice[0]);
    println!("all array {:?}", slice);
}

fn main() {
    let mut a: [i32;5] = [1,2,3,4,5];

    println!("{:?} and len of a : {}", a, a.len());

    if a == [1,2,3,4,5] {
        println!("equal");
    }

    let b = [1u8; 10];

    println!("b = {:?} and took up {} bytes", b, mem::size_of_val(&b));

    let mtx:[[f32;3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len(){
            println!("{}", mtx[i][j]);
        }
    }


    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);


    let five = 5;
    let two = 2;

    let sp: (i32, i32) = (five+two, five*two);
    println!("{:?}", sp);
    println!("{0} + {1} = {2} and {0} * {1} = {3}",
                five, two, five + two, five * two);

    let sp2 = (five - two, five / two);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("{}", (combined.0).1);

    //destructuring
    let (three, twot) = sp2;
}
