use crate::arrays::trapping_rainwater::unoptimized::trapping_rainwater;

mod arrays;

fn main() {
    let arr = [1,3,1,4];
    let arr2= [0,1,0,2,1,0,3,1,0,1,2];
    let solution = trapping_rainwater(&arr);
    println!("{}", solution);
}
