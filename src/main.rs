mod arrays;
use crate::arrays::biggest_water_container::container_with_most_water_brute;

fn main() {
    let a = vec![7,1,2,3,9];
    let solution = container_with_most_water_brute(&a) ;
    println!("{}", solution);
}
