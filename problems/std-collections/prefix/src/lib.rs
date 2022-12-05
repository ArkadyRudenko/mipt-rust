#![forbid(unsafe_code)]

use std::cmp::min;

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.len() == 0 {
        return "".to_string();
    }

    let mut final_pos = strs[0].chars().count();

    for i in 1..strs.len() {
        let mut iter = 0;
        let mut main_letter = strs[0].chars();
        for sub_letter in strs[i].chars() {
            let ch = match main_letter.next() {
                Some(c) => c,
                None => break
            };
            if sub_letter != ch {
                break
            }
            iter += 1;
        }
        final_pos = min(final_pos, iter);
    }

    strs[0].chars().take(final_pos).collect()
}