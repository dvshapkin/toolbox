/// Binary search in slice.
///
pub fn binary<T>(list: &[T], item: T) -> Option<usize>
    where
        T: PartialOrd,
{
    if list.len() == 0 {
        return None;
    }

    let mid = list.len() / 2;

    if list[mid] > item {
        return binary(&list[..mid], item);
    } else if list[mid] < item {
        return binary(&list[mid..], item);
    } else {
        return Some(mid);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_integer() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(super::binary(&list, 7).unwrap(), 6);
    }
}
