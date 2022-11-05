#![forbid(unsafe_code)]

pub fn go_combinations(offset: usize, k: usize, result: &mut Vec<Vec<i32>>, src: &[i32], combination: &mut Vec<i32>) {
    if k == 0 {
        result.push(combination.clone());
        return;
    }

    for i in offset..=src.len() - k {
        combination.push(src[i]);
        go_combinations(i + 1, k - 1, result, src, combination);
        combination.pop();
    }

}


pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 || arr.len() == 0{
        return vec![vec![]];
    }

    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut first_combination: Vec<i32> = Vec::new();

    go_combinations(0, k, &mut res, arr, &mut first_combination);

    res
}
