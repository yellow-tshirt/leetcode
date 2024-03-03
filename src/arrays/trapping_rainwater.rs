pub mod unoptimized{
    pub fn trapping_rainwater(arr: &[i32])->i32{
        let mut rw = 0;
        for (index, bar) in arr.iter().enumerate(){
            let mut max_left = 0;
            let mut max_right = 0;
            for i in 0..arr.len() {
                if i < index && max_left < arr[i] {
                    max_left = arr[i]
                } else if i > index && max_right < arr[i]{
                    max_right = arr[i]
                }else{
                    continue;
                }
            }
            let waterCurrent = i32::min(max_left, max_right) - bar;
            if waterCurrent > 0{
                rw += waterCurrent
            }
        }
        rw
    }
}