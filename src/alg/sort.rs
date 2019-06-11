use super::search;

/// Selection sort.
///
pub fn selection<T>(list: &mut [T], ascending: bool)
    where
        T: PartialOrd
{
    if list.len() < 2 {
        return;
    }

    let extrem = if ascending { search::min } else { search::max };
    let compare: fn(&T, &T) -> bool = if ascending {
        |a,b| a < b
    } else {
        |a,b| a > b
    };

    let mut cur_pos = 0;
    let max_idx = list.len() - 1;

    while cur_pos < max_idx {
        let idx = extrem(&list[cur_pos..]).unwrap() + cur_pos;
        if compare(&list[idx], &list[cur_pos]) {
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
        let mut list = [1, 2, 12, 5, 43, 21, 0, 2, 34, 100, 3];

        selection(&mut list, true);
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1]);
        }

        selection(&mut list, false);
        for i in 0..list.len() - 1 {
            assert!(list[i] >= list[i + 1]);
        }
    }
}