#![allow(dead_code)]

pub mod brute {
    //first attempt
    pub fn find_two_sum_brute(a: Vec<i32>, target: i32) -> Option<[usize; 2]> {
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[i] + a[j] == target {
                    return Some([i, j]);
                }
            }
        }

         None
    }
    //leetcode adjusted
    pub fn two_sum_leet(a: Vec<i32>, target: i32) -> Vec<i32>{
        for i in 0..a.len() {
            for j in i+1..a.len() {
                if a[i]+a[j] == target{
                    return vec![i as i32,j as i32]
                }
            }
        }

        vec![0,0]
    }

}

pub mod optimized {
    use std::collections::HashMap;

    pub fn find_two_sum_optimized(a: Vec<i32>, target: i32) -> Vec<i32>{
        let mut hm = HashMap::new();
        for (i, current_number) in a.iter().enumerate(){
            match hm.get(current_number) {
                Some(&number) => return vec![number, i as i32],
                _ => {
                    hm.insert(target-a[i],i as i32);
                }
            }

        }
        vec![-1,-1]
    }

}