#[derive(Debug)]
pub enum LevError {
    XWordOutOfBound,
    YWordOutOfBound,
}

use std::sync::Mutex;
// wagner-fischer
#[derive(Debug)]
pub struct Lev {
    xlen: usize,
    ylen: usize,
    matrix: Mutex<Vec<Vec<i32>>>,
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
            matrix: Mutex::new(matrix),
        }
    }

    fn neighbors(&self, bmatrix: &[Vec<i32>], y: usize, x: usize) -> i32 {
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
        let mut matrix = self.matrix.lock().unwrap();
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
pub struct Table {
    pub col_count: usize,
    content: Vec<Vec<String>>,
    lev_matrix: Lev,
}

impl Table {
    pub fn new(max_search: usize, col_count: usize, content: Vec<Vec<String>>) -> Option<Table> {
        if content.iter().map(Vec::len).any(|a| a != col_count) {
            return None;
        }
        let max_yword = content
            .iter()
            .map(|row| row.iter().map(|a| a.len()).max())
            .max()
            .flatten()
            .unwrap_or(0);
        Some(Table {
            col_count,
            content,
            lev_matrix: Lev::new(max_search, max_yword + 2),
        })
    }
    pub fn compare(&self, search: &str, take: usize) -> Vec<(&[String], i32)> {
        let mut a = self
            .content
            .iter()
            .map(|row| {
                (
                    row.as_slice(),
                    row.iter()
                        .map(|cmp| self.lev_matrix.compare(search, cmp))
                        .filter_map(Result::ok)
                        .min()
                        .unwrap_or(999),
                )
            })
            .collect::<Vec<_>>();
        a.sort_by_key(|(_, lev)| *lev);
        a.into_iter().take(take).collect()
    }
}

pub struct Rows<'a> {
    content: Vec<&'a [String]>,
}

impl<'a> From<Vec<(&'a [String], i32)>> for Rows<'a> {
    fn from(content: Vec<(&'a [String], i32)>) -> Rows<'a> {
        Rows {
            content: content.into_iter().map(|(row, _)| row).collect(),
        }
    }
}

use maud::{html, Markup, Render, PreEscaped};
impl Render for Rows<'_> {
    fn render(&self) -> Markup {
        html! {
            @for row in &self.content {
                tr {
                    @for col in row.iter() {
                        td { (PreEscaped(col)) }
                    }
                }
            }
        }
    }
}
