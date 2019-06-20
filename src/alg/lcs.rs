use crate::ds::Matrix;

pub fn lcs<'a, T>(sa: &'a [T], sb: &'a [T]) -> Option<&'a [T]>
    where
        T: Copy + PartialEq
{
    let m = create_and_fill_matrix(sa, sb);

    None
}

fn create_and_fill_matrix<'a, T>(sa: &'a [T], sb: &'a [T]) -> Matrix<u32>
where
    T: Copy + PartialEq
{
    let mut m = Matrix::<u32>::new(sa.len() + 1, sb.len() + 1);

    m.fill(0);

    let mut count = 0;

    for i in 0..sa.len() {
        for j in 0..sb.len() {
            if sa[i] == sb[j] {
                count = m.get(i, j) + 1;
            }
            if count > 0 {
                m.set(i + 1, j + 1, count);
            }
        }
    }

    m
}

#[cfg(test)]
mod tests {
    use crate::alg::lcs::create_and_fill_matrix;

    # [test]
    fn create_and_fill_matrix_ok() {
        let m = create_and_fill_matrix("XXXaXXXbXXXcXX".as_ref(), "YYaYYYYbYcYYYYY".as_ref());
        println!("{:?}", m);
    }
}