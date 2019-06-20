pub struct Matrix<T> {
    m: Vec<T>,
    width: usize,
    height: usize,
}

impl<T:Copy> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        Matrix { m: Vec::with_capacity(width * height), width, height }
    }

    pub fn count(&self) -> usize {
        self.width * self.height
    }

    pub fn fill(&mut self, value: T) {
        for _ in 0..self.count() {
            self.m.push(value);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        if row >= self.height || col >= self.width {
            panic!("Index out of range.");
        }
        self.m[row * self.width + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row >= self.height || col >= self.width {
            panic!("Index out of range.");
        }
        self.m[row * self.width + col] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::Matrix;

    #[test]
    fn new_and_count_ok() {
        let m = Matrix::<i32>::new(10, 30);
        assert_eq!(m.count(), 300);
    }

    #[test]
    fn fill_and_get_ok() {
        let mut m = Matrix::<i32>::new(7, 5);
        m.fill(21);
        for row in 0..m.height {
            for col in 0..m.width {
                assert_eq!(m.get(row, col), 21);
            }
        }
    }

    #[test]
    fn set_ok() {
        let mut m = Matrix::<i32>::new(7, 5);
        m.fill(0);
        m.set(3, 3, 21);
        assert_eq!(m.get(3,3), 21);
    }
}