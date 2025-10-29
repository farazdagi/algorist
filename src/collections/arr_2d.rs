//! A 2D array implementation with various utility methods.
//!
//! See the [`Arr`] documentation for more details.

use {
    crate::io::Scanner,
    std::{fmt::Debug, io::BufRead},
};

/// A 2D array implementation.
///
/// Often either incoming or outgoing data is in the form of a 2D array. Working
/// with such data is easier with this structure, which provides various utility
/// methods for accessing and manipulating it.
///
/// # Start with empty array
///
/// For a type `T` that implements `Default`, you can create an empty 2D array
/// with the specified number of rows and columns, using the `new` method:
///
/// ```
/// use algorist::collections::arr_2d::Arr;
///
/// let arr: Arr<bool> = Arr::new(2, 3);
/// assert_eq!(arr.as_ref(), &vec![
///     false, false, false, false, false, false
/// ]);
/// ```
/// # Create from vector
///
/// If you have a vector of data and know the number of rows and columns, you
/// can create a 2D array using the [`from_vec`](Arr::from_vec) method:
///
/// ```
/// use algorist::collections::arr_2d::Arr;
///
/// let arr = Arr::from_vec(vec![1, 2, 3, 4, 5, 6], 2, 3);
/// assert_eq!(arr[0], [1, 2, 3]);
/// assert_eq!(arr[1], [4, 5, 6]);
/// ```
///
/// # Create from input
///
/// You can create a 2D array from input using the `from_scan` method:
///
/// ```
/// use {
///     algorist::{collections::arr_2d::Arr, io::Scanner},
///     std::io::Cursor,
/// };
///
/// let mut scan = Scanner::new(Cursor::new(b"1 2 3\n4 5 6\n"));
/// let arr: Arr<usize> = Arr::from_scan(&mut scan, 2, 3);
/// assert_eq!(arr.as_ref(), &vec![1, 2, 3, 4, 5, 6]);
/// ```
/// In case where the input is a character table, you can use the
/// [`from_chars`](Arr::from_chars)
/// method.
///
/// # Create with a generator
///
/// If you need to create a 2D array with specific values, you can use the
/// `with_generator` method, which allows you to specify a function that
/// generates the values for each cell based on its row and column indices:
///
/// ```
/// use algorist::collections::arr_2d::Arr;
///
/// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
/// assert_eq!(arr[0], [0, 1, 2]);
/// assert_eq!(arr[1], [3, 4, 5]);
/// ```
///
/// # Accessing elements
///
/// You can access elements using tuple indexing:
///
/// ```
/// use algorist::collections::arr_2d::Arr;
///
/// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
/// assert_eq!(arr[(0, 0)], 0);
/// assert_eq!(arr[(1, 2)], 5);
/// ```
///
/// Additionally, you can access entire rows or columns:
///
/// ```
/// use algorist::collections::arr_2d::Arr;
///
/// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
///
/// // Row accessed either by index or using the `row` method:
/// assert_eq!(arr.row(0).collect::<Vec<_>>(), vec![&0, &1, &2]);
/// assert_eq!(arr[0], [0, 1, 2]);
///
/// // Column accessed using the `col` method:
/// assert_eq!(arr.col(1).collect::<Vec<_>>(), vec![&1, &4]);
/// ```
///
/// # Utility methods
///
/// To transpose the 2D array, you can use the [`transpose`](Arr::transpose)
/// method.
///
/// To get all "↙" (south-west) or "↘" (south-east) diagonals, you can use
/// [`diags_sw`](Arr::diags_sw) and [`diags_se`](Arr::diags_se) methods
/// respectively.
///
/// To find the minimum element by a key function, you can use the
/// [`min_by_key`](Arr::min_by_key) method.
///
/// To get the coordinates of adjacent cells, you can use the
/// [`adj_cells`](Arr::adj_cells) method, which allows you to specify whether
/// you want adjacent cells, diagonal cells, or both.
#[derive(Debug, PartialEq, Eq)]
pub struct Arr<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Arr<T>
where
    T: Clone + Debug + Default,
{
    /// Creates a new 2D array with the specified number of rows and columns,
    /// initializing all elements to the default value for type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr: Arr<i32> = Arr::new(2, 3);
    /// assert_eq!(arr.as_ref(), &vec![0, 0, 0, 0, 0, 0]);
    ///
    /// let arr: Arr<bool> = Arr::new(2, 3);
    /// assert_eq!(arr.as_ref(), &vec![
    ///     false, false, false, false, false, false
    /// ]);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    /// Transposes the 2D array, swapping rows and columns.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let mut arr: Arr<usize> = Arr::new(2, 3);
    ///
    /// for i in 0..arr.rows() {
    ///     for j in 0..arr.cols() {
    ///         arr[(i, j)] = i * arr.cols() + j;
    ///     }
    /// }
    /// assert_eq!(arr.as_ref(), &vec![0, 1, 2, 3, 4, 5]);
    /// assert_eq!(arr[0], [0, 1, 2]);
    /// assert_eq!(arr[1], [3, 4, 5]);
    ///
    /// let arr_t = arr.transpose();
    /// assert_eq!(arr_t.as_ref(), &vec![0, 3, 1, 4, 2, 5]);
    /// assert_eq!(arr_t[0], [0, 3]);
    /// assert_eq!(arr_t[1], [1, 4]);
    /// assert_eq!(arr_t[2], [2, 5]);
    /// ```
    #[must_use]
    pub fn transpose(&self) -> Self {
        Self::with_generator(self.cols, self.rows, |i, j| self[(j, i)].clone())
    }
}

impl<T: Clone + Debug + std::str::FromStr> Arr<T> {
    /// Creates a new 2D array from a [`Scanner`], reading the specified number
    /// of rows and columns.
    ///
    /// # Example
    ///
    /// ```
    /// use {
    ///     algorist::{collections::arr_2d::Arr, io::Scanner},
    ///     std::io::Cursor,
    /// };
    ///
    /// let mut scan = Scanner::new(Cursor::new(b"1 2 3\n4 5 6\n"));
    /// let arr: Arr<usize> = Arr::from_scan(&mut scan, 2, 3);
    /// assert_eq!(arr.as_ref(), &vec![1, 2, 3, 4, 5, 6]);
    /// ```
    pub fn from_scan<B: BufRead>(scan: &mut Scanner<B>, rows: usize, cols: usize) -> Self {
        Self::with_generator(rows, cols, |_, _| scan.next())
    }
}

impl<T: Debug> Arr<T> {
    /// Creates a new 2D array from a vector of data, specifying the number of
    /// rows and columns.
    ///
    /// # Panics
    ///
    /// Panics if the length of the vector does not match the product of `rows`
    /// and `cols`.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let data = vec![1, 2, 3, 4, 5, 6];
    /// let arr = Arr::from_vec(data, 2, 3);
    /// assert_eq!(arr.as_ref(), &vec![1, 2, 3, 4, 5, 6]);
    /// assert_eq!(arr[0], [1, 2, 3]);
    /// assert_eq!(arr[1], [4, 5, 6]);
    /// ```
    pub fn from_vec(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self { data, rows, cols }
    }

    /// Creates a new 2D array with the specified number of rows and columns,
    /// using a generator function to fill the elements.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(arr[0], [0, 1, 2]);
    /// assert_eq!(arr[1], [3, 4, 5]);
    /// ```
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

    /// Creates a new 2D array from a character table.
    ///
    /// Input is read from a [`Scanner`], filling the elements using a provided
    /// function `f` that maps characters to type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use {
    ///     algorist::{collections::arr_2d::Arr, io::Scanner},
    ///     std::io::Cursor,
    /// };
    ///
    /// let mut scan = Scanner::new(Cursor::new(b"abc\ndef\n"));
    /// let arr: Arr<char> = Arr::from_chars(&mut scan, 2, 3, |c| c);
    /// assert_eq!(arr[0], ['a', 'b', 'c']);
    /// assert_eq!(arr[1], ['d', 'e', 'f']);
    ///
    /// let mut scan = Scanner::new(Cursor::new(b"100\n010\n"));
    /// let arr: Arr<bool> = Arr::from_chars(&mut scan, 2, 3, |c| c == '1');
    /// assert_eq!(arr[0], [true, false, false]);
    /// assert_eq!(arr[1], [false, true, false]);
    /// ```
    ///    
    pub fn from_chars<B: BufRead, F>(scan: &mut Scanner<B>, rows: usize, cols: usize, f: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let data: Vec<Vec<_>> = (0..rows).map(|_| scan.chars()).collect();
        Self::with_generator(rows, cols, |i, j| f(data[i][j]))
    }

    /// Iterates over the elements of the 2D array, from top-left to
    /// bottom-right.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterates over the mutable elements of the 2D array, from top-left to
    /// bottom-right.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Gets the iterator over the elements in the row at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(arr.row(0).collect::<Vec<_>>(), vec![&0, &1, &2]);
    /// assert_eq!(arr.row(1).collect::<Vec<_>>(), vec![&3, &4, &5]);
    /// ```
    pub fn row(&self, idx: usize) -> impl Iterator<Item = &T> {
        assert!(idx < self.rows);
        self.iter().skip(idx * self.cols).take(self.cols)
    }

    /// Gets the mutable iterator over the elements in the row at the specified
    /// index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// for v in arr.row_mut(0) {
    ///     *v *= 2;
    /// }
    /// assert_eq!(arr[0], [0, 2, 4]);
    /// ```
    pub fn row_mut(&mut self, idx: usize) -> impl Iterator<Item = &mut T> {
        assert!(idx < self.rows);
        let cols = self.cols;
        self.iter_mut().skip(idx * cols).take(cols)
    }

    /// Gets the iterator over the elements in the column at the specified
    /// index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(arr.col(0).collect::<Vec<_>>(), vec![&0, &3]);
    /// assert_eq!(arr.col(1).collect::<Vec<_>>(), vec![&1, &4]);
    /// assert_eq!(arr.col(2).collect::<Vec<_>>(), vec![&2, &5]);
    /// ```
    pub fn col(&self, idx: usize) -> impl Iterator<Item = &T> {
        assert!(idx < self.cols);
        self.iter().skip(idx).step_by(self.cols)
    }

    /// Gets the mutable iterator over the elements in the column at the
    /// specified index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(arr.col(0).collect::<Vec<_>>(), vec![&0, &3]);
    ///
    /// for v in arr.col_mut(0) {
    ///     *v *= 2;
    /// }
    /// assert_eq!(arr.col(0).collect::<Vec<_>>(), vec![&0, &6]);
    /// assert_eq!(arr[0], [0, 1, 2]);
    /// assert_eq!(arr[1], [6, 4, 5]);
    /// ```
    pub fn col_mut(&mut self, idx: usize) -> impl Iterator<Item = &mut T> {
        assert!(idx < self.cols);
        let cols = self.cols;
        self.iter_mut().skip(idx).step_by(cols)
    }

    /// Gives all "↙" (south-west) diagonals.
    ///
    /// ``` bash
    /// # Given array:
    /// 0 1 2
    /// 3 4 5
    ///
    /// # Diagonals:
    /// 0
    /// 1 3
    /// 2 4
    /// 5
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(
    ///     arr.diags_sw()
    ///         .map(|d| d.collect::<Vec<_>>())
    ///         .collect::<Vec<_>>(),
    ///     vec![vec![&0], vec![&1, &3], vec![&2, &4], vec![&5],]
    /// );
    /// ```
    pub fn diags_sw(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
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

    /// Gives all "↘" (south-east) diagonals.
    ///
    /// ``` bash
    /// # Given array:
    /// 0 1 2
    /// 3 4 5
    ///
    /// # Diagonals:
    /// 2
    /// 1 5
    /// 0 4
    /// 3
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(
    ///     arr.diags_se()
    ///         .map(|d| d.collect::<Vec<_>>())
    ///         .collect::<Vec<_>>(),
    ///     vec![vec![&2], vec![&1, &5], vec![&0, &4], vec![&3],]
    /// );
    /// ```
    pub fn diags_se(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
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

    /// Given cell coordinates, return indexes of the south-west and south-east
    /// diagonals (zero based) that pass through that cell.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    ///
    /// // For cell (0, 0), the south-west diagonal is the first one
    /// // (index 0) and the south-east diagonal is the third one (index 2).
    /// assert_eq!(arr.cell_diags(0, 0), (0, 2));
    /// ```
    pub fn cell_diags(&self, row: usize, col: usize) -> (usize, usize) {
        (row + col, self.cols - (col + 1) + row)
    }

    /// Returns the coordinates of the cells adjacent to the specified one.
    ///
    /// The `cell_type` parameter specifies which adjacent cells to include:
    /// - `Adjacent` includes only the cells directly adjacent (up, down, left,
    ///   right).
    /// - `Diagonal` includes only the diagonal cells.
    /// - `Both` includes both adjacent and diagonal cells.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::{AdjacentCells, Arr, Cell};
    ///
    /// let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// // For cell (0, 0), the adjacent cells are (1, 0) and (0, 1),
    /// // and the diagonal cell is (1, 1) (only cells in bounds are considered).
    /// assert_eq!(arr.adj_cells(0, 0, AdjacentCells::Adjacent), vec![
    ///     Cell::new(&3, 1, 0),
    ///     Cell::new(&1, 0, 1)
    /// ]);
    /// assert_eq!(arr.adj_cells(0, 0, AdjacentCells::Diagonal), vec![
    ///     Cell::new(&4, 1, 1)
    /// ]);
    /// assert_eq!(arr.adj_cells(0, 0, AdjacentCells::Both), vec![
    ///     Cell::new(&3, 1, 0),
    ///     Cell::new(&1, 0, 1),
    ///     Cell::new(&4, 1, 1)
    /// ]);
    /// ```
    pub fn adj_cells(&self, row: usize, col: usize, cell_type: AdjacentCells) -> Vec<Cell<'_, T>> {
        use AdjacentCells::*;
        let max_size = if cell_type == Both { 8 } else { 4 };
        let mut cells = Vec::with_capacity(max_size);

        if matches!(cell_type, Adjacent | Both) {
            if row > 0 {
                cells.push(self.cell(row - 1, col));
            }
            if col > 0 {
                cells.push(self.cell(row, col - 1));
            }
            if row + 1 < self.rows {
                cells.push(self.cell(row + 1, col));
            }
            if col + 1 < self.cols {
                cells.push(self.cell(row, col + 1));
            }
        }

        if matches!(cell_type, Diagonal | Both) {
            if row > 0 && col > 0 {
                cells.push(self.cell(row - 1, col - 1));
            }
            if row > 0 && col + 1 < self.cols {
                cells.push(self.cell(row - 1, col + 1));
            }
            if row + 1 < self.rows && col > 0 {
                cells.push(self.cell(row + 1, col - 1));
            }
            if row + 1 < self.rows && col + 1 < self.cols {
                cells.push(self.cell(row + 1, col + 1));
            }
        }

        cells
    }

    /// Returns a cell at the specified coordinates in the 2D array.
    pub fn cell(&self, row: usize, col: usize) -> Cell<'_, T> {
        Cell::from_arr(self, (row, col))
    }

    /// Swaps the elements at the specified coordinates in the 2D array.
    ///
    /// # Panics
    /// Panics if the specified coordinates are out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::Arr;
    ///
    /// let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
    /// assert_eq!(arr.as_ref(), &vec![0, 1, 2, 3, 4, 5]);
    /// // Swap elements at (0, 0) and (1, 1).
    /// arr.swap(0, 0, 1, 1);
    /// assert_eq!(arr.as_ref(), &vec![4, 1, 2, 3, 0, 5]);
    /// // Swap the same element (no change).
    /// arr.swap(0, 0, 0, 0);
    /// assert_eq!(arr.as_ref(), &vec![4, 1, 2, 3, 0, 5]);
    /// ```
    pub fn swap(&mut self, row1: usize, col1: usize, row2: usize, col2: usize) {
        assert!(row1 < self.rows);
        assert!(row2 < self.rows);
        assert!(col1 < self.cols);
        assert!(col2 < self.cols);
        let idx1 = row1 * self.cols + col1;
        let idx2 = row2 * self.cols + col2;
        self.data.swap(idx1, idx2);
    }

    /// Returns the number of rows in the 2D array.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the 2D array.
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T: Debug + Ord> Arr<T> {
    /// Finds the coordinates of the minimum element in the 2D array.
    ///
    /// The comparison is based on a predicate function that accepts the element
    /// and its coordinates, and returns a reference to the element to compare
    /// for minimality.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::collections::arr_2d::{Arr, Cell};
    ///
    /// let arr: Arr<i32> = Arr::from_vec(vec![3, 1, 2, -4, 5, 0], 2, 3);
    /// let min_coords = arr.min_by_key(|cell| {
    ///     // Ignore the coordinates, just use value for comparison
    ///     cell.value().abs()
    /// });
    ///
    /// // The minimum absolute value is 0 at (1, 2)
    /// assert_eq!(min_coords, Some(Cell::new(&0, 1, 2)));
    /// ```
    pub fn min_by_key(&self, f: impl Fn(Cell<T>) -> T) -> Option<Cell<'_, T>> {
        let cols = self.cols;
        self.iter()
            .enumerate()
            .map(|(i, x)| (x, i / cols, i % cols))
            .min_by_key(|&(x, row, col)| f(Cell::new(x, row, col)))
            .map(|x| x.into())
    }
}

impl<T: Debug> std::fmt::Display for Arr<T> {
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

impl<T: Debug> core::ops::Index<(usize, usize)> for Arr<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        assert!(row < self.rows);
        assert!(col < self.cols);
        &self.data[row * self.cols + col]
    }
}

impl<T: Debug> core::ops::IndexMut<(usize, usize)> for Arr<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        assert!(row < self.rows);
        assert!(col < self.cols);
        &mut self.data[row * self.cols + col]
    }
}

impl<T: Debug> core::ops::Index<usize> for Arr<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < self.rows);
        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl<T: Debug> core::ops::IndexMut<usize> for Arr<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx < self.rows);
        &mut self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl<T: Debug> AsRef<Vec<T>> for Arr<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Debug> AsMut<Vec<T>> for Arr<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}

impl<T: Debug> IntoIterator for Arr<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T: Debug> IntoIterator for &'a Arr<T> {
    type IntoIter = std::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

/// Represents a cell value with its coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell<'a, T>(&'a T, usize, usize);

impl<'a, T: Debug> Cell<'a, T> {
    /// Creates a new cell with the specified value and coordinates.
    pub fn new(value: &'a T, row: usize, col: usize) -> Self {
        Self(value, row, col)
    }

    /// Creates a new cell from the value at the specified coordinates.
    pub fn from_arr(arr: &'a Arr<T>, coords: (usize, usize)) -> Self {
        Self(&arr[coords], coords.0, coords.1)
    }

    /// Returns the value of the cell.
    pub fn value(&self) -> &T {
        self.0
    }

    /// Returns the row index of the cell.
    pub fn row(&self) -> usize {
        self.1
    }

    /// Returns the column index of the cell.
    pub fn col(&self) -> usize {
        self.2
    }
}

impl<'a, T: Debug> From<(&'a Arr<T>, (usize, usize))> for Cell<'a, T> {
    fn from((arr, coords): (&'a Arr<T>, (usize, usize))) -> Self {
        Self::from_arr(arr, coords)
    }
}

impl<'a, T: Debug> From<(&'a T, usize, usize)> for Cell<'a, T> {
    fn from((val, row, col): (&'a T, usize, usize)) -> Self {
        Self(val, row, col)
    }
}

impl<'a, T: Debug> std::ops::Deref for Cell<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Type of adjacent cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjacentCells {
    Adjacent,
    Diagonal,
    Both,
}

#[cfg(test)]
mod tests {
    use {super::*, std::io};

    #[test]
    fn test_1() {
        let mut arr = Arr::new(2, 3);
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
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_3() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.iter().sum::<usize>(), 15);
    }

    #[test]
    fn test_4() {
        let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        arr.iter_mut().for_each(|x| *x += 1);
        assert_eq!(arr.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_5() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.row(0).sum::<usize>(), 3);
        assert_eq!(arr.row(1).sum::<usize>(), 12);
        assert_eq!(arr.row(0).collect::<Vec<_>>(), vec![&0, &1, &2]);
        assert_eq!(arr.row(1).collect::<Vec<_>>(), vec![&3, &4, &5]);
    }

    #[test]
    fn test_6() {
        let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        arr.row_mut(0).for_each(|x| *x += 1);
        assert_eq!(arr.data, vec![1, 2, 3, 3, 4, 5]);
    }

    #[test]
    fn test_7() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.col(0).sum::<usize>(), 3);
        assert_eq!(arr.col(1).sum::<usize>(), 5);
        assert_eq!(arr.col(0).collect::<Vec<_>>(), vec![&0, &3]);
        assert_eq!(arr.col(1).collect::<Vec<_>>(), vec![&1, &4]);
    }

    #[test]
    fn test_8() {
        let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        arr.col_mut(0).for_each(|x| *x += 1);
        assert_eq!(arr.data, vec![1, 1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_9() {
        let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        arr.swap(0, 0, 1, 1);
        assert_eq!(arr.data, vec![4, 1, 2, 3, 0, 5]);

        // swap the same element
        arr.swap(0, 0, 0, 0);
        assert_eq!(arr.data, vec![4, 1, 2, 3, 0, 5]);
    }

    #[test]
    fn test_10() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.rows(), 2);
        assert_eq!(arr.cols(), 3);
    }

    #[test]
    fn test_11() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr[(0, 0)], 0);
        assert_eq!(arr[(1, 2)], 5);
    }

    #[test]
    fn test_12() {
        let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        arr[(0, 0)] = 10;
        arr[(1, 2)] = 20;
        assert_eq!(arr.data, vec![10, 1, 2, 3, 4, 20]);
    }

    #[test]
    fn test_13() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr[0], [0, 1, 2]);
        assert_eq!(arr[1], [3, 4, 5]);
    }

    #[test]
    fn test_14() {
        let mut arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        arr[0].copy_from_slice(&[10, 11, 12]);
        for (got, want) in arr[1].iter_mut().zip(13usize..) {
            *got = want;
        }
        assert_eq!(arr.data, vec![10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn test_15() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.data, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(arr, arr.transpose().transpose());
        let arr_t = arr.transpose();
        assert_eq!(arr_t.data, vec![0, 3, 1, 4, 2, 5]);
        assert_eq!(arr_t.row(0).collect::<Vec<_>>(), vec![&0, &3]);
        assert_eq!(arr_t.row(1).collect::<Vec<_>>(), vec![&1, &4]);
    }

    #[test]
    fn test_16() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        let arr_t = arr.transpose();
        assert_eq!(arr_t.col(0).collect::<Vec<_>>(), vec![&0, &1, &2]);
        assert_eq!(arr_t.col(1).collect::<Vec<_>>(), vec![&3, &4, &5]);
    }

    #[test]
    fn test_17() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        assert_eq!(arr.iter().sum::<usize>(), 15);
        assert_eq!(arr.iter().sum::<usize>(), 15);
        assert_eq!(arr.into_iter().sum::<usize>(), 15);
    }

    #[test]
    fn test_18() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        let arr2 = Arr::from_scan(&mut Scanner::new(io::Cursor::new(b"0 1 2 3 4 5\n")), 2, 3);
        assert_eq!(arr, arr2);
    }

    #[test]
    fn test_neigh_coords() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        let cells = arr.adj_cells(0, 0, AdjacentCells::Adjacent);
        assert_eq!(cells, vec![Cell(&3, 1, 0), Cell(&1, 0, 1)]);

        let cells = arr.adj_cells(0, 0, AdjacentCells::Diagonal);
        assert_eq!(cells, vec![Cell(&4, 1, 1)]);

        let cells = arr.adj_cells(0, 0, AdjacentCells::Both);
        assert_eq!(cells, vec![Cell(&3, 1, 0), Cell(&1, 0, 1), Cell(&4, 1, 1)]);

        let cells = arr.adj_cells(0, 1, AdjacentCells::Adjacent);
        assert_eq!(cells, vec![Cell(&0, 0, 0), Cell(&4, 1, 1), Cell(&2, 0, 2)]);

        let cells = arr.adj_cells(0, 1, AdjacentCells::Diagonal);
        assert_eq!(cells, vec![Cell(&3, 1, 0), Cell(&5, 1, 2)]);

        let cells = arr.adj_cells(0, 1, AdjacentCells::Both);
        assert_eq!(cells, vec![
            Cell(&0, 0, 0),
            Cell(&4, 1, 1),
            Cell(&2, 0, 2),
            Cell(&3, 1, 0),
            Cell(&5, 1, 2)
        ]);
    }

    #[test]
    fn test_neigh_vals() {
        let arr = Arr::with_generator(2, 3, |i, j| i * 3 + j);
        let cells = arr.adj_cells(0, 0, AdjacentCells::Adjacent);
        assert_eq!(cells, vec![Cell(&3, 1, 0), Cell(&1, 0, 1)]);

        let cells = arr.adj_cells(0, 1, AdjacentCells::Both);
        assert_eq!(cells, vec![
            Cell(&0, 0, 0),
            Cell(&4, 1, 1),
            Cell(&2, 0, 2),
            Cell(&3, 1, 0),
            Cell(&5, 1, 2),
        ]);
    }

    #[test]
    fn test_right_diags() {
        let arr = Arr::with_generator(3, 3, |i, j| i * 3 + j);
        let diags = arr
            .diags_sw()
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
        let arr = Arr::with_generator(3, 3, |i, j| i * 3 + j);
        let diags = arr
            .diags_se()
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
        let arr = Arr::with_generator(3, 2, |i, j| i * 2 + j);
        let diags = arr
            .diags_sw()
            .map(|x| x.map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(diags, vec![vec![0], vec![1, 2], vec![3, 4], vec![5],]);

        let diags = arr
            .diags_se()
            .map(|x| x.map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(diags, vec![vec![1], vec![0, 3], vec![2, 5], vec![4],]);
    }

    #[test]
    fn test_cell_diags_pos() {
        let arr = Arr::with_generator(3, 3, |i, j| i * 3 + j);
        assert_eq!(arr.cell_diags(0, 0), (0, 2));
        assert_eq!(arr.cell_diags(0, 1), (1, 1));
        assert_eq!(arr.cell_diags(0, 2), (2, 0));
        assert_eq!(arr.cell_diags(1, 0), (1, 3));
        assert_eq!(arr.cell_diags(1, 1), (2, 2));
        assert_eq!(arr.cell_diags(1, 2), (3, 1));
        assert_eq!(arr.cell_diags(2, 0), (2, 4));
        assert_eq!(arr.cell_diags(2, 1), (3, 3));
        assert_eq!(arr.cell_diags(2, 2), (4, 2));
    }
}
