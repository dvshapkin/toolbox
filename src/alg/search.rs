/// Binary search in slice.
///
pub fn binary<T>(list: &[T], item: T) -> Option<usize>
where
    T: PartialOrd,
{
    if list.is_empty() {
        return None;
    }

    let mut lhs = 0;
    let mut rhs = list.len() - 1;

    while lhs <= rhs {
        let mid = (lhs + rhs) / 2;
        if list[mid] > item {
            rhs = mid - 1;
        } else if list[mid] < item {
            lhs = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_integer() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(super::binary(&list, 7).unwrap(), 6);

        let list = [5];
        assert_eq!(super::binary(&list, 5).unwrap(), 0);

        let list = [5, 5, 5];
        assert_eq!(super::binary(&list, 5).unwrap(), 1);

        let list = [];
        assert_eq!(super::binary(&list, 0), None);
    }
}
