use std::ops::{Index, IndexMut};
use std::{alloc, mem};

/// Rectangular table of elements (two-dimensional array).
///
pub struct Matrix<'a, T>
where
    T: Default + Clone,
{
    cols: usize,
    buffer: &'a mut [T],
}

impl<'a, T> Matrix<'a, T>
where
    T: Default + Clone,
{
    /// Creates new Matrix and fills it with default values.
    ///
    /// `rows` - rows number.
    /// `cols` - columns number.
    /// Panic, if memory allocation is not succesfully.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            buffer: Self::alloc(rows, cols),
        }
    }

    /// Fills matrix with a default values.
    ///
    pub fn clear(&mut self) {
        Self::fill_with(self.buffer, T::default());
    }

    /// Fills matrix with a `value`.
    ///
    pub fn fill(&mut self, value: T) {
        Self::fill_with(self.buffer, value);
    }

    /// Returns rows number.
    ///
    pub fn rows(&self) -> usize {
        self.buffer.len() / self.cols
    }

    /// Returns columns number.
    ///
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns number of elements in matrix.
    /// 
    pub fn elements_number(&self) -> usize {
        self.buffer.len()
    }

    /// Returns value at [row][col] position.
    ///
    /// There are bounds checking.
    /// If index out of range, then panic.
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.buffer[self.linear_index(row, col)]
    }

    /// Sets the `value` of element at [row][col] position.
    ///
    /// There are bounds checking.
    /// If index out of range, then panic.
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.buffer[self.linear_index(row, col)] = value;
    }

    /// Iterator over matrix.
    /// 
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter { collection: self.buffer, current: 0 }
    } 

    /// Memory allocation for data buffer.
    ///
    fn alloc(rows: usize, cols: usize) -> &'a mut [T] {
        unsafe {
            let buf = alloc::alloc(layout::<T>(rows * cols).unwrap()) as *mut T;
            let slice = std::slice::from_raw_parts_mut(buf, rows * cols);
            Self::fill_with(slice, T::default());
            slice
        }
    }

    /// Fills data buffer with a `value`.
    ///
    fn fill_with(buf: &mut [T], value: T) {
        for e in buf {
            *e = value.clone();
        }
    }

    fn linear_index(&self, row: usize, col: usize) -> usize {
        if row >= self.rows() || col >= self.cols {
            panic!("index out of bounds");
        }
        row * self.cols + col
    }
}

impl<'a, T> Drop for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.buffer.as_mut_ptr() as *mut u8,
                layout::<T>(self.buffer.len()).unwrap(),
            );
        }
    }
}

fn layout<T>(size: usize) -> Result<alloc::Layout, alloc::LayoutErr> {
    alloc::Layout::from_size_align(size * mem::size_of::<T>(), mem::align_of::<T>())
}

impl<'a, T> Index<usize> for Matrix<'a, T>
where
    T: Default + Clone,
{
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        &self.buffer[row * self.cols..(row + 1) * self.cols]
    }
}

impl<'a, T> IndexMut<usize> for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.buffer[row * self.cols..(row + 1) * self.cols]
    }
}

impl<'a, T> Clone for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn clone(&self) -> Self {
        let new_buf = Self::alloc(self.rows(), self.cols());
        for idx in 0..self.buffer.len() {
            new_buf[idx] = self.buffer[idx].clone();
        }
        Matrix { cols: self.cols, buffer: new_buf }
    }
}

pub struct Iter<'a, T>
where
    T: Default + Clone,
{
    collection: &'a [T],
    current: usize
}

impl<'a, T> Iterator for Iter<'a, T>
where
     T: Default + Clone,
{
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.collection.len() {
            self.current += 1;
            Some(&self.collection[self.current - 1])
        } else {
            None
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::Matrix;
    use std::fmt::Debug;

    #[test]
    fn new_ok() {
        let m = Matrix::<i32>::new(100, 100);
        assert_eq_all::<i32>(&m, 0);
    }

    #[test]
    fn fill_ok() {
        let mut m = Matrix::<i32>::new(100, 100);
        m.fill(1);
        assert_eq_all::<i32>(&m, 1);
    }

    #[test]
    fn clear_ok() {
        let mut m = Matrix::<i32>::new(100, 100);
        m.fill(1);
        assert_eq_all::<i32>(&m, 1);
        m.clear();
        assert_eq_all::<i32>(&m, 0);
    }

    #[test]
    fn get_set_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.set(1, 1, 777);
        assert_eq!(m.get(1, 1), &777);
    }

    #[test]
    fn index_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.set(1, 1, 777);
        assert_eq!(m[0][2], 0);
        assert_eq!(m[1][1], 777);
    }

    #[test]
    fn index_mut_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m[1][1] = 777;
        assert_eq!(m.get(1, 1), &777);
        m[0][0] = m[1][1] - 111;
        assert_eq!(m.get(0, 0), &666);
    }

    #[test]
    fn clone_ok() {
        // numbers
        let mut a = Matrix::<i32>::new(2, 3);
        a.fill(100);
        let b = a.clone();
        a.fill(200);
        assert_eq_all(&b, 100);
        assert_eq_all(&a, 200);
        
        // Strings
        let mut s1 = Matrix::<String>::new(2, 3);
        s1.fill(String::from("first"));
        let s2 = s1.clone();
        s1.fill(String::from("second"));
        assert_eq_all(&s2, String::from("first"));
        assert_eq_all(&s1, String::from("second"));
    }

    #[test]
    fn iter_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.fill(7);
        let mut count = 0;
        for e in m.iter() {
            assert_eq!(e, &7);
            count += 1;
        }
        assert_eq!(count, m.elements_number());
    }

    fn assert_eq_all<T: Default + Clone + PartialEq + Debug>(m: &Matrix<T>, value: T) {
        for i in 0..m.rows() {
            for j in 0..m.cols() {
                assert_eq!(m.get(i, j), &value);
            }
        }
    }
}
