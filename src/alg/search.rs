/// Binary search in slice.
///
pub fn binary<T>(list: &[T], item: T) -> Option<usize>
    where
        T: PartialOrd
{
    return partial(&list, 0, list.len(), item);

    fn partial<T>(list: &[T], lhs: usize, rhs: usize, item: T) -> Option<usize>
        where
            T: PartialOrd
    {
        if list[lhs..rhs].len() == 0 {
            return None;
        }

        let mid = (lhs + rhs) / 2;

        if list[mid] > item {
            return partial(&list, lhs, mid, item);
        } else if list[mid] < item {
            return partial(&list, mid, rhs, item);
        } else {
            return Some(mid);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_integer() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(super::binary(&list, 7).unwrap(), 6);

        let list = [5];
        assert_eq!(super::binary(&list, 5).unwrap(), 0);

        let list = [5,5,5];
        assert_eq!(super::binary(&list, 5).unwrap(), 1);

        let list = [];
        assert_eq!(super::binary(&list, 0), None);
    }
}
