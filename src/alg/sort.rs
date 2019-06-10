use super::search;

/// Selection sort.
///
pub fn selection<T>(list: &[T]) -> Option<Vec<T>>
where T: PartialOrd + Clone
{
    let mut sorted = Vec::<T>::with_capacity(list.len());
    let mut used = vec![false; list.len()];
    let mut count = list.len();
    while count > 0 {
        let idx = search::min(&list)?;
        sorted.push(list[idx].clone());
    }
    Some(sorted)
}

/// Quick sort.
///
pub fn quick<T>(list: &mut [T])
where
    T: PartialOrd,
{
    unimplemented!()
}
