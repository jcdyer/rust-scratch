extern crate rand;

use std::fmt::Debug;
use rand::distributions::{Range, IndependentSample};


fn main() {
    let table = mapping_table(512);
    print_slice(&table);
    let perm = perm1(&table);
    print_slice(&perm)
}


fn mapping_table(elements: isize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    let mut table = vec![]; 
    for range in (0..elements).map(|el| Range::new(0, elements - el)) {
        table.push(range.ind_sample(&mut rng));
    }
    table
}


fn perm1(input: &[isize]) -> Vec<isize> {
    let mut mapping: Vec<_> = input.iter().map(|x| *x).collect();
    let mut perm = vec![];
    for i in 0..mapping.len() {
        for el in &mut mapping {
            if *el > 0 {
                *el -= 1;
            } else if *el == 0 {
                perm.push(i as isize);
                *el -= 1;
                break;
            }
        }
    }
    //perm.into_iter().collect::<Option<Vec<isize>>>().expect("All should be collected")
    perm
}


fn print_slice<T: Debug>(input: &[T]) {
    let spacing = format!("{}", input.len()).len() + 1;
    for el in input {
        print!("{:1$?}", el, spacing);
    }
    println!();
}
