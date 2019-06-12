use super::search;

/// Selection sort.
///
pub fn selection<T>(list: &mut [T], ascending: bool)
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }

    let extrem = if ascending { search::min } else { search::max };
    let compare: fn(&T, &T) -> bool = if ascending {
        |a, b| a < b
    } else {
        |a, b| a > b
    };

    let mut current = 0;
    let upper = list.len() - 1;

    while current < upper {
        let found = extrem(&list[current..]).unwrap() + current;
        if compare(&list[found], &list[current]) {
            list.swap(current, found);
        }
        current += 1;
    }
}

/// Quick sort.
///
pub fn quick<T>(list: &mut [T])
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }

    recursion(list, 0, list.len() - 1);


    fn recursion<T>(list: &mut [T], lhs: usize, rhs: usize)
        where
            T: PartialOrd,
    {
        let pivot = (lhs + rhs) / 2;
        let mut i = lhs;
        let mut j = rhs;

        while i <= j {
            while list[i] < list[pivot] {
                i += 1
            }
            while list[j] > list[pivot] {
                j -= 1
            }

            if i <= j {
                list.swap(i, j);
                if i < rhs {
                    i += 1;
                }
                if j > lhs {
                    j -= 1
                };
            }
        }

        if j > lhs {
            recursion(list, lhs, j)
        }
        if i < rhs {
            recursion(list, i, rhs)
        }
    }
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

    #[test]
    fn quick_ok() {
        let mut list = [1, 2, 12, 5, 43, 21, 0, 2, 34, 100, 3];
        quick(&mut list);
        println!("{:?}", list);

        let mut list = [1, 2, 3, 4, 5, 6, 7];
        quick(&mut list);
        println!("{:?}", list);

        let mut list = [7, 6, 5, 4, 3, 2, 1];
        quick(&mut list);
        println!("{:?}", list);

        let mut list = [1, 1, 1, 4, 2, 2, 2];
        quick(&mut list);
        println!("{:?}", list);

        let mut list = [5, 5, 5, 1, 5, 5, 5];
        quick(&mut list);
        println!("{:?}", list);

        let mut list = [7];
        quick(&mut list);
        println!("{:?}", list);

        let mut list: [i32; 0] = [];
        quick(&mut list);
        println!("{:?}", list);
    }
}
