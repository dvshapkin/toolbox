pub fn quick_sort<T>(seq: &mut [T])
    where T: PartialOrd
{
    let middle = &seq[seq.len() / 2];
    let mut i = 0_usize;
    let mut j = seq.len() - 1;
    while i <= j {
        while seq[i] < *middle { i += 1 }
        while seq[j] > *middle { j -= 1 }
        if i <= j {
            std::mem::swap(&mut seq[i], &mut seq[j]);
        }
    }
}
