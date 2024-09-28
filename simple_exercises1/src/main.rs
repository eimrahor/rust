use std::mem;

fn main() {
   let a: u8 = 123;
   println!("{}",a);


   // u : unsigned 0 - 255
   // i : signed -128 - 0 - +127
   let mut b: i8 = 0;
   println!("before {}",b);
   b = -42;
   println!("after {}",b);

   let c: u8 = 12;
   println!("size {} bytes",mem::size_of_val(&c));


   let d: char = 'a';
   println!("d is {} and mem size {} bytes",
                        d,mem::size_of_val(&d));

   // f32 ve f64
   let e: f32 = 2.5;
   println!("{} and mem is {} bytes",
               e,mem::size_of_val(&e));

   let g: bool = true;
   println!("{} and {} is bytes",g,mem::size_of_val(&g));


   println!("--------------- Arithmetics ---------------");

   let mut a = 2 + 3 * 4; // 14  +-*/

   a += 1; // OR a = a + 1  a++ yok

   let a_cubed = i32::pow(a,3);
   println!("a cubed is {}",a_cubed);

   let b: f32 = 2.5;
   let b_cubed = f32::powi(b,3);
   let b_to_pi = f32::powf(b,std::f32::consts::PI);
   println!("{} cubed = {}, {}^pi = {}",
            b,b_cubed,b,b_to_pi);

   // bitwise
   let c = 1 | 2; // 01 | 10 = 11 = 3(decimal)
   let two_to_10 = 1 << 10;
   println!("2^10 = {}",two_to_10);


   // İlginç
   {
      println!("inside a: {}",a);
      let a = 5;
      println!("again inside a: {}",a);
   }
   println!("outside a: {}",a);
}
