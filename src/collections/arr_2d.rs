#![allow(dead_code)]

use std::fmt::Debug;
use std::io::BufRead;

use crate::io::Scanner;

#[derive(Debug, PartialEq, Eq)]
pub struct Arr2d<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone + Debug> Arr2d<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        Self {
            data: vec![default; rows * cols],
            rows,
            cols,
        }
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        Self::with_generator(self.cols, self.rows, |i, j| self[(j, i)].clone())
    }
}

impl<T: Clone + Debug + std::str::FromStr> Arr2d<T> {
    pub fn from_scan<B: BufRead>(scan: &mut Scanner<B>, rows: usize, cols: usize) -> Self {
        Self::with_generator(rows, cols, |_, _| scan.next())
    }
}

impl<T: Debug> Arr2d<T> {
    pub fn from_char_table<B: BufRead, F>(
        scan: &mut Scanner<B>,
        rows: usize,
        cols: usize,
        f: F,
    ) -> Self
    where
        F: Fn(char) -> T,
    {
        let data: Vec<Vec<_>> = (0..rows).map(|_| scan.chars()).collect();
        Self::with_generator(rows, cols, |i, j| f(data[i][j]))
    }
}

impl<T: Debug> Arr2d<T> {
    pub fn with_generator<F>(rows: usize, cols: usize, mut generator: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                data.push(generator(i, j));
            }
        }
        Self { data, rows, cols }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn row(&self, idx: usize) -> impl Iterator<Item = &T> {
        assert!(idx < self.rows);
        self.iter().skip(idx * self.cols).take(self.cols)
    }

    pub fn row_mut(&mut self, idx: usize) -> impl Iterator<Item = &mut T> {
        assert!(idx < self.rows);
        let cols = self.cols;
        self.iter_mut().skip(idx * cols).take(cols)
    }

    pub fn col(&self, idx: usize) -> impl Iterator<Item = &T> {
        assert!(idx < self.cols);
        self.iter().skip(idx).step_by(self.cols)
    }

    pub fn col_mut(&mut self, idx: usize) -> impl Iterator<Item = &mut T> {
        assert!(idx < self.cols);
        let cols = self.cols;
        self.iter_mut().skip(idx).step_by(cols)
    }

    /// Gives all "/" diagonals(the first is at top left, the last is at bottom
    /// right position).
    pub fn right_diags(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        let rows = self.rows;
        let cols = self.cols;
        (0..rows + cols - 1).map(move |i| {
            let start_row = if i < cols { 0 } else { i - cols + 1 };
            let start_col = if i < cols { i } else { cols - 1 };
            let len = if i < cols {
                (i + 1).min(rows)
            } else {
                (cols + rows - i - 1).min(cols)
            };
            (0..len).map(move |j| &self[(start_row + j, start_col - j)])
        })
    }

    /// Gives all "\" diagonals (the first is at top right, the last is at
    /// bottom left position).
    pub fn left_diags(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        let rows = self.rows;
        let cols = self.cols;
        (0..rows + cols - 1).map(move |i| {
            let start_row = if i < cols { 0 } else { i - cols + 1 };
            let start_col = if i < cols { cols - i - 1 } else { 0 };
            let len = if i < cols {
                (i + 1).min(rows)
            } else {
                (cols + rows - i - 1).min(cols)
            };
            (0..len).map(move |j| &self[(start_row + j, start_col + j)])
        })
    }

    /// Given cell coordinates, return indexes of the right and left diagonals
    /// (zero based) that pass through the cell.
    pub fn cell_diags_pos(&self, row: usize, col: usize) -> (usize, usize) {
        (row + col, self.cols - (col + 1) + row)
    }

    pub fn neigh_coords(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        let mut neighs = Vec::with_capacity(4);
        if row > 0 {
            neighs.push((row - 1, col));
        }
        if col > 0 {
            neighs.push((row, col - 1));
        }
        if row + 1 < self.rows {
            neighs.push((row + 1, col));
        }
        if col + 1 < self.cols {
            neighs.push((row, col + 1));
        }
        neighs.into_iter()
    }

    pub fn diag_neigh_coords(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut neighs = Vec::with_capacity(4);
        if row > 0 && col > 0 {
            neighs.push((row - 1, col - 1));
        }
        if row > 0 && col + 1 < self.cols {
            neighs.push((row - 1, col + 1));
        }
        if row + 1 < self.rows && col > 0 {
            neighs.push((row + 1, col - 1));
        }
        if row + 1 < self.rows && col + 1 < self.cols {
            neighs.push((row + 1, col + 1));
        }
        neighs.into_iter()
    }

    pub fn all_neigh_coords(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        self.neigh_coords(row, col)
            .chain(self.diag_neigh_coords(row, col))
    }

    pub fn neigh_vals(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        self.neigh_coords(row, col)
            .map(|(i, j)| (i, j, &self[(i, j)]))
    }

    pub fn swap(&mut self, row1: usize, col1: usize, row2: usize, col2: usize) {
        assert!(row1 < self.rows);
        assert!(row2 < self.rows);
        assert!(col1 < self.cols);
        assert!(col2 < self.cols);
        let idx1 = row1 * self.cols + col1;
        let idx2 = row2 * self.cols + col2;
        self.data.swap(idx1, idx2);
    }

    pub fn rows(&self) -> core::ops::Range<usize> {
        0..self.rows
    }

    pub fn cols(&self) -> core::ops::Range<usize> {
        0..self.cols
    }
}

impl<T: Debug + Ord> Arr2d<T> {
    pub fn min_by_key(&self, f: impl Fn((usize, usize, &T)) -> &T) -> Option<(usize, usize)> {
        let cols = self.cols;
        self.iter()
            .enumerate()
            .map(|(i, x)| (i / cols, i % cols, x))
            .min_by_key(|&x| f(x))
            .map(|x| (x.0, x.1))
    }
}

impl<T: Debug> std::fmt::Display for Arr2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?}", self[(i, j)])?;
                if j + 1 < self.cols {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug> core::ops::Index<(usize, usize)> for Arr2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        assert!(row < self.rows);
        assert!(col < self.cols);
        &self.data[row * self.cols + col]
    }
}

impl<T: Debug> core::ops::IndexMut<(usize, usize)> for Arr2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        assert!(row < self.rows);
        assert!(col < self.cols);
        &mut self.data[row * self.cols + col]
    }
}

impl<T: Debug> core::ops::Index<usize> for Arr2d<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < self.rows);
        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl<T: Debug> core::ops::IndexMut<usize> for Arr2d<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx < self.rows);
        &mut self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl<T: Debug> AsRef<Vec<T>> for Arr2d<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Debug> AsMut<Vec<T>> for Arr2d<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}

impl<T: Debug> IntoIterator for Arr2d<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T: Debug> IntoIterator for &'a Arr2d<T> {
    type IntoIter = std::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn test_1() {
        let mut arr = Arr2d::new(2, 3, 0);
        arr.data[0] = 1;
        arr.data[1] = 2;
        arr.data[2] = 3;
        arr.data[3] = 4;
        arr.data[4] = 5;
        arr.data[5] = 6;
        assert_eq!(arr.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_2() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_3() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.iter().sum::<usize>(), 15);
    }

    #[test]
    fn test_4() {
        let mut arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        arr.iter_mut().for_each(|x| *x += 1);
        assert_eq!(arr.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_5() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.row(0).sum::<usize>(), 3);
        assert_eq!(arr.row(1).sum::<usize>(), 12);
        assert_eq!(arr.row(0).collect::<Vec<_>>(), vec![&0, &1, &2]);
        assert_eq!(arr.row(1).collect::<Vec<_>>(), vec![&3, &4, &5]);
    }

    #[test]
    fn test_6() {
        let mut arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        arr.row_mut(0).for_each(|x| *x += 1);
        assert_eq!(arr.data, vec![1, 2, 3, 3, 4, 5]);
    }

    #[test]
    fn test_7() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.col(0).sum::<usize>(), 3);
        assert_eq!(arr.col(1).sum::<usize>(), 5);
        assert_eq!(arr.col(0).collect::<Vec<_>>(), vec![&0, &3]);
        assert_eq!(arr.col(1).collect::<Vec<_>>(), vec![&1, &4]);
    }

    #[test]
    fn test_8() {
        let mut arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        arr.col_mut(0).for_each(|x| *x += 1);
        assert_eq!(arr.data, vec![1, 1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_9() {
        let mut arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        arr.swap(0, 0, 1, 1);
        assert_eq!(arr.data, vec![4, 1, 2, 3, 0, 5]);

        // swap the same element
        arr.swap(0, 0, 0, 0);
        assert_eq!(arr.data, vec![4, 1, 2, 3, 0, 5]);
    }

    #[test]
    fn test_10() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.rows().collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(arr.cols().collect::<Vec<_>>(), vec![0, 1, 2]);
    }

    #[test]
    fn test_11() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr[(0, 0)], 0);
        assert_eq!(arr[(1, 2)], 5);
    }

    #[test]
    fn test_12() {
        let mut arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        arr[(0, 0)] = 10;
        arr[(1, 2)] = 20;
        assert_eq!(arr.data, vec![10, 1, 2, 3, 4, 20]);
    }

    #[test]
    fn test_13() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr[0], [0, 1, 2]);
        assert_eq!(arr[1], [3, 4, 5]);
    }

    #[test]
    fn test_14() {
        let mut arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        arr[0].copy_from_slice(&[10, 11, 12]);
        for (got, want) in arr[1].iter_mut().zip(13usize..) {
            *got = want;
        }
        assert_eq!(arr.data, vec![10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn test_15() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(arr, arr.transpose().transpose());
        let arr_t = arr.transpose();
        assert_eq!(arr_t.data, vec![0, 3, 1, 4, 2, 5]);
        assert_eq!(arr_t.row(0).collect::<Vec<_>>(), vec![&0, &3]);
        assert_eq!(arr_t.row(1).collect::<Vec<_>>(), vec![&1, &4]);
    }

    #[test]
    fn test_16() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        let arr_t = arr.transpose();
        assert_eq!(arr_t.col(0).collect::<Vec<_>>(), vec![&0, &1, &2]);
        assert_eq!(arr_t.col(1).collect::<Vec<_>>(), vec![&3, &4, &5]);
    }

    #[test]
    fn test_17() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.iter().sum::<usize>(), 15);
        assert_eq!(arr.iter().sum::<usize>(), 15);
        assert_eq!(arr.into_iter().sum::<usize>(), 15);
    }

    #[test]
    fn test_18() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        let arr2 = Arr2d::from_scan(&mut Scanner::new(io::Cursor::new(b"0 1 2 3 4 5\n")), 2, 3);
        assert_eq!(arr, arr2);
    }

    #[test]
    fn test_neigh_coords() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        let neighs = arr.neigh_coords(0, 0).collect::<Vec<_>>();
        assert_eq!(neighs, vec![(1, 0), (0, 1)]);

        let neighs = arr.neigh_coords(0, 1).collect::<Vec<_>>();
        assert_eq!(neighs, vec![(0, 0), (1, 1), (0, 2)]);
    }

    #[test]
    fn test_neigh_vals() {
        let arr = Arr2d::with_generator(2, 3, |i, j| i * 3 + j);
        let neighs = arr.neigh_vals(0, 0).collect::<Vec<_>>();
        assert_eq!(neighs, vec![(1, 0, &3), (0, 1, &1)]);

        let neighs = arr.neigh_vals(0, 1).collect::<Vec<_>>();
        assert_eq!(neighs, vec![(0, 0, &0), (1, 1, &4), (0, 2, &2)]);
    }

    #[test]
    fn test_right_diags() {
        let arr = Arr2d::with_generator(3, 3, |i, j| i * 3 + j);
        let diags = arr
            .right_diags()
            .map(|x| x.map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(diags, vec![
            vec![0],
            vec![1, 3],
            vec![2, 4, 6],
            vec![5, 7],
            vec![8]
        ]);
    }

    #[test]
    fn test_left_diags() {
        let arr = Arr2d::with_generator(3, 3, |i, j| i * 3 + j);
        let diags = arr
            .left_diags()
            .map(|x| x.map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(diags, vec![
            vec![2],
            vec![1, 5],
            vec![0, 4, 8],
            vec![3, 7],
            vec![6]
        ]);
    }

    #[test]
    fn test_non_square_diags() {
        let arr = Arr2d::with_generator(3, 2, |i, j| i * 2 + j);
        let diags = arr
            .right_diags()
            .map(|x| x.map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(diags, vec![vec![0], vec![1, 2], vec![3, 4], vec![5],]);

        let diags = arr
            .left_diags()
            .map(|x| x.map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(diags, vec![vec![1], vec![0, 3], vec![2, 5], vec![4],]);
    }

    #[test]
    fn test_cell_diags_pos() {
        let arr = Arr2d::with_generator(3, 3, |i, j| i * 3 + j);
        assert_eq!(arr.cell_diags_pos(0, 0), (0, 2));
        assert_eq!(arr.cell_diags_pos(0, 1), (1, 1));
        assert_eq!(arr.cell_diags_pos(0, 2), (2, 0));
        assert_eq!(arr.cell_diags_pos(1, 0), (1, 3));
        assert_eq!(arr.cell_diags_pos(1, 1), (2, 2));
        assert_eq!(arr.cell_diags_pos(1, 2), (3, 1));
        assert_eq!(arr.cell_diags_pos(2, 0), (2, 4));
        assert_eq!(arr.cell_diags_pos(2, 1), (3, 3));
        assert_eq!(arr.cell_diags_pos(2, 2), (4, 2));
    }
}
