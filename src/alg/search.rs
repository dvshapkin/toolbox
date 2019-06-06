/// Binary search in slice.
///
pub fn binary<T>(list: &[T], item: &T) -> Option<usize>
    where
        T: PartialOrd
{
    return partial(&list, 0, list.len(), item);

    fn partial<T>(list: &[T], lhs: usize, rhs: usize, item: &T) -> Option<usize>
        where
            T: PartialOrd
    {
        if list[lhs..rhs].len() == 0 {
            return None;
        }

        let mid = list[lhs..rhs].len() / 2;

        if list[mid] > *item {
            return partial(&list, lhs, mid, item);
        } else if list[mid] < *item {
            return partial(&list, mid, rhs, item); //binary(&list[mid..], item);
        } else {
            return Some(mid);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_integer() {
        //let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let list = [7];
        assert_eq!(super::binary(&list, &7).unwrap(), 0);
        assert_eq!(super::binary(&list, &1), None);

        let list = [7,8,9];
        assert_eq!(super::binary(&list, &7).unwrap(), 0);
        assert_eq!(super::binary(&list, &8).unwrap(), 1);
        assert_eq!(super::binary(&list, &9).unwrap(), 2);
    }
}
