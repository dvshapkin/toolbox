use crate::ds::Matrix;

pub fn lcs<'a, T>(sa: &'a [T], sb: &'a [T]) -> Option<&'a [T]>
where
    T: Copy + PartialEq
{
    let mut m = Matrix::<T>::new(sa.len(), sb.len());
    None
}