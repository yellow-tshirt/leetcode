#![allow(dead_code)]
pub mod brute{
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
}

pub mod optimized {
    pub fn container_with_most_water(arr: &[i32]) -> i32{
        let mut max_area = 0;
        let mut left_pointer = 0;
        let mut right_pointer = arr.len()-1;
        let mut distance_between_pointers = right_pointer - left_pointer;
        while distance_between_pointers >= 1{
            let current_area = i32::min(arr[left_pointer], arr[right_pointer]) * distance_between_pointers as i32;
            if current_area > max_area {
                max_area =  current_area;
            }
            if arr[left_pointer] <= arr[right_pointer]{
                left_pointer += 1;
            }else{
                right_pointer -= 1;
            }
            distance_between_pointers = right_pointer - left_pointer;
        }
        max_area
    }
    pub fn container_with_most_water_leet(arr: Vec<i32>) -> i32{
        let mut max_area = 0;
        let mut left_pointer = 0;
        let mut right_pointer = arr.len()-1;
        let mut distance_between_pointers = right_pointer - left_pointer;
        while distance_between_pointers >= 1{
            let current_area = i32::min(arr[left_pointer], arr[right_pointer]) * distance_between_pointers as i32;
            if current_area > max_area {
                max_area =  current_area;
            }
            if arr[left_pointer] <= arr[right_pointer]{
                left_pointer += 1;
            }else{
                right_pointer -= 1;
            }
            distance_between_pointers = right_pointer - left_pointer;
        }
        max_area
    }
}