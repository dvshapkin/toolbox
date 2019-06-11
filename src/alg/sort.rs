use super::search;

/// Selection sort.
///
pub fn selection<T>(list: &mut [T])
    where T: PartialOrd
{
    if list.len() < 2 {
        return;
    }
    let mut cur_pos = 0;
    let max_idx = list.len() - 1;
    while cur_pos < max_idx {
        let idx = search::min(&list[cur_pos..]).unwrap() + cur_pos;
        if idx > cur_pos {
            list.swap(cur_pos, idx);
        }
        cur_pos += 1;

    }
}

/// Quick sort.
///
pub fn quick<T>(_list: &mut [T])
where
    T: PartialOrd,
{
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_ok() {
        let mut list = [1, 2, 12, 5, 43, 21, 0, 34, 100, 3];
        selection(&mut list);
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1]);
        }
    }
}