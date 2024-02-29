
pub fn container_with_most_water_brute(arr: &[i32]) -> i32{
    let mut max_area = 0;
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            let area_of_container = i32::min(arr[i], arr[j]) * (j-i) as i32;
            if max_area < area_of_container{
                max_area = area_of_container;
            }
        }
    }
    max_area
}