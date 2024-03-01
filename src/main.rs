mod arrays;
use crate::arrays::biggest_water_container::container_with_most_water;

fn main() {
    let a = vec![7,1,2,100,99,9];
    let solution = container_with_most_water(&a) ;
    println!("{}", solution);
}
