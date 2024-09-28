mod stack_heap;

const UNIVERSE_VALUE: u8 = 12;
static mut DENE: u8 = 6;

fn main() {
    unsafe{
        DENE = 3;
        println!("{} {}",
                UNIVERSE_VALUE,DENE);
    }
    stack_heap::stack_and_heap();
}
