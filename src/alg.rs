pub fn quick_sort<T>(seq: &mut [T])
    where T: PartialOrd + Clone
{
    let len = seq.len();
    let mid_idx = len / 2;
    let mid_val = seq[mid_idx].clone();
    let mut i = 0_usize;
    let mut j = len - 1;
    while i <= j {
        while seq[i] < mid_val { i += 1 }
        while seq[j] > mid_val { j -= 1 }
        if i <= j {
            let i_val = seq[i].clone();
            let j_val = seq[j].clone();
            std::mem::replace(&mut seq[i], j_val);
            std::mem::replace(&mut seq[j], i_val);
            i += 1;
            j -= 1;
        }
    }
    if mid_idx > 2 {
        quick_sort(&mut seq[..mid_idx]);
        quick_sort(&mut seq[mid_idx..]);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn quick_sort_test() {
        let mut m = [1, 12, 5, 0, 4, -6, 12, 8, 9, 10];
        super::quick_sort(&mut m);
        println!("m = {:?}", m);
        assert_eq!(m, [-6, 0, 1, 4, 5, 8, 9, 10, 12, 12]);
    }
}