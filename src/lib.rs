#[derive(Debug)]
pub enum LevError {
    XWordOutOfBound,
    YWordOutOfBound,
}

use std::cell::RefCell;
// wagner-fischer
#[derive(Debug)]
pub struct Lev {
    xlen: usize,
    ylen: usize,
    matrix: RefCell<Vec<Vec<i32>>>,
}

impl Lev {
    pub fn new(xlen: usize, ylen: usize) -> Lev {
        let mut matrix = vec![vec![0; xlen]; ylen];
        for y in 0..ylen {
            matrix[y][0] = y as i32;
        }
        for x in 0..xlen {
            matrix[0][x] = x as i32;
        }
        Lev {
            xlen,
            ylen,
            matrix: RefCell::new(matrix),
        }
    }

    fn neighbors(&self, bmatrix: &Vec<Vec<i32>>, y: usize, x: usize) -> i32 {
        std::cmp::min(
            bmatrix[y - 1][x - 1],
            std::cmp::min(bmatrix[y][x - 1], bmatrix[y - 1][x]),
        )
    }

    pub fn compare(&self, xword: &str, yword: &str) -> Result<i32, LevError> {
        if xword.len() >= self.xlen {
            return Err(LevError::XWordOutOfBound);
        }
        if yword.len() >= self.ylen {
            return Err(LevError::YWordOutOfBound);
        }
        let mut matrix = self.matrix.borrow_mut();
        let mut ychars = yword.chars();
        for y in 0..self.ylen - 1 {
            let ychar = ychars.next();
            let mut xchars = xword.chars();
            for x in 0..self.xlen - 1 {
                let xchar = xchars.next();
                let neib = self.neighbors(&matrix, y + 1, x + 1);
                let matched = ychar.map_or(false, |yc| xchar.map_or(false, |xc| yc == xc));
                matrix[y + 1][x + 1] = neib + (if matched { 0 } else { 1 });
            }
        }
        Ok(matrix[yword.len()][xword.len()])
    }
}

#[derive(Debug)]
pub struct Table<T> {
    pub col_count: usize,
    content: Vec<Vec<T>>,
    compare: Lev,
}

impl<T> Table<T> {
    pub fn new(max_search: usize, col_count: usize, content: Vec<Vec<T>>) -> Option<Table<T>> {
        if content
            .iter()
            .map(Vec::len)
            .filter(|a| *a != col_count)
            .next()
            .is_some()
        {
            return None;
        }
        Some(Table {
            col_count,
            content,
            compare: Lev::new(max_search, col_count + 1),
        })
    }
    pub fn compare(&self, against: &str, take: i32) -> Vec<(i32, Vec<T>)> {
        todo!()
    }
}
