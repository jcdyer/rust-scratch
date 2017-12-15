extern crate x86intrin;
use x86intrin::u16x16;

fn main() {
    let it = u16x16(0xabcd, 0xabcd, 0xabcd, 0xabcd, 0, 0, 0, 0, 0xabcd, 0xabcd, 0xabcd, 0xabcd, 0, 0, 0, 0);
    println!("it[3] = {:x}", it.extract(3));
    println!("{}", it.as_m256);
    println!("{}", it.as_array);
}
