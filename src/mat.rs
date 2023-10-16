use std::{fmt::{Debug, Display}, iter, ops::{Mul, Add, AddAssign, MulAssign, Neg}};


#[derive(Clone)]
pub struct Matrix<A> {
    width: usize,
    data: Vec<A>,
}
impl<A> Debug for Matrix<A>
where
    A: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_mat = self.map(|e| format!("{:?}", e));

        let widths: Vec<usize> = str_mat
            .cols()
            .iter()
            .map(|col| col.iter().map(String::len).max().unwrap_or(0))
            .collect();

        for y in 0..self.height() {
            write!(f, "[")?;
            for x in 0..self.width() {
                if x != 0 {
                    write!(f, " ")?;
                }
                let s = str_mat.get(y, x).unwrap();
                for _ in s.len()..widths[x] {
                    write!(f, " ")?;
                }
                write!(f, "{}", s)?;
            }
            writeln!(f, "]")?;
        }

        Ok(())
    }
}

impl<A> Display for Matrix<A>
where
    A: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_mat = self.map(|e| format!("{}", e));

        let widths: Vec<usize> = str_mat
            .cols()
            .iter()
            .map(|col| col.iter().map(String::len).max().unwrap_or(0))
            .collect();

        for y in 0..self.height() {
            write!(f, "[")?;
            for x in 0..self.width() {
                if x != 0 {
                    write!(f, " ")?;
                }
                let s = str_mat.get(y, x).unwrap();
                for _ in s.len()..widths[x] {
                    write!(f, " ")?;
                }
                write!(f, "{}", s)?;
            }
            writeln!(f, "]")?;
        }

        Ok(())
    }
}

impl<A> PartialEq for Matrix<A> 
where A: PartialEq{
    fn eq(&self, other: &Self) -> bool {
        if self.width() != other.width() || self.height() != other.height(){
            return false;
        }

        self.data == other.data
    }
}

impl<A> Matrix<A>
where
    A: Clone,
{
    pub fn filled(height: usize, width: usize, v: A) -> Matrix<A> {
        Matrix {
            width,
            data: iter::repeat(v).take(width * height).collect(),
        }
    }

    pub fn transpose(&self) -> Matrix<A> {
        Matrix::by_pos(self.width(), self.height(), |y, x| {
            self.get(x, y).unwrap().clone()
        })
    }
    pub fn cols(&self) -> Vec<Vec<A>> {
        self.transpose().rows().map(|col| col.to_vec()).collect()
    }
}

impl<A> Matrix<A> {
    pub fn data_slice(&self)->&[A]{
        &self.data
    }
    pub fn by_pos(height: usize, width: usize, f: impl Fn(usize, usize) -> A) -> Matrix<A> {
        let data = (0..height)
            .flat_map(|y| (0..width).zip(iter::repeat(y)))
            .map(|(x, y)| f(y, x))
            .collect();
        Matrix { width, data }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[A]> {
        self.data.as_slice().chunks(self.width)
    }
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [A]> {
        self.data.as_mut_slice().chunks_mut(self.width)
    }
    pub fn row_mut(&mut self, n: usize) -> impl Iterator<Item = &mut A> {
        self.rows_mut().nth(n).unwrap_or_default().iter_mut()
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn map<B>(&self, f: impl Fn(&A) -> B) -> Matrix<B> {
        Matrix {
            data: self.data.iter().map(f).collect(),
            width: self.width,
        }
    }
    pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut A> {
        if y >= self.height() {
            return None;
        }
        if x >= self.width() {
            return None;
        }

        let width = self.width();
        self.data.get_mut(y * width + x)
    }
    pub fn get(&self, y: usize, x: usize) -> Option<&A> {
        if y >= self.height() {
            return None;
        }
        if x >= self.width() {
            return None;
        }

        self.data.get(y * self.width() + x)
    }
}

impl<R: Ring + Clone> Matrix<R> {
    pub fn id(size: usize) -> Matrix<R> {
        Matrix::by_pos(size, size, |y, x| if x == y { R::ONE } else { R::ZERO })
    }

    fn mul_row(&mut self, y: usize, λ: R){
        for e in self.row_mut(y){
            *e *= λ.clone();
        }
    }
    fn add_row(&mut self, y_src: usize, y_dest: usize, λ: R){
        assert_ne!(y_src, y_dest);
        for i in 0..self.width(){
            let val = self.get(y_src, i).unwrap().clone();
            *self.get_mut(y_dest, i).unwrap() += val * λ.clone();
        }
    }

    pub fn solve(&mut self, rhs: &mut Matrix<R>) {
        for r in 0..self.height() {
            let el = self.get(r, r).unwrap().clone();
            if el != R::ZERO {
                let el = el.mul_inv();
                self.mul_row(r, el.clone());
                rhs.mul_row(r, el);
                for r2 in (r+1)..self.height(){
                    let el2 = -self.get(r2, r).unwrap().clone();
                    self.add_row(r, r2, el2.clone());
                    rhs.add_row(r, r2, el2);
                }
            }else{
                todo!("handle zeros")
            }
        }
        for r in (0..self.height()).rev(){
            for r2 in 0..r{
                let el2 = -self.get(r2, r).unwrap().clone();
                self.add_row(r, r2, el2.clone());
                rhs.add_row(r, r2, el2);
            }
        }
    }
}

impl<R: Ring + Clone> Mul<&Self> for Matrix<R> {
    type Output = Matrix<R>;
    fn mul(self, rhs: &Self) -> Self::Output {
        assert!(self.width() == rhs.height());
        let n = self.width();
        Matrix::by_pos(self.height(), rhs.width(), |i, j| {
            let mut acc = R::ZERO;
            for k in 0..n {
                acc +=
                    self.get(i, k).unwrap().to_owned() 
                    * rhs.get(k, j).unwrap().to_owned();
            }
            acc
        })
    }
}

impl<A> Add for &Matrix<A>
where
    A: Add + Clone,
{
    type Output = Matrix<<A as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.width() == rhs.width());
        assert!(self.height() == rhs.height());
        Matrix::by_pos(self.height(), self.width(), |y, x| {
            self.get(y, x).unwrap().clone() + rhs.get(y, x).unwrap().clone()
        })
    }
}

trait Ring: Sized + Add<Output = Self> + AddAssign + MulAssign + Mul<Output = Self> + PartialEq + Neg<Output = Self>{
    const ONE: Self;
    const ZERO: Self;
    fn mul_inv(&self)->Self;
}

impl Ring for f64 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;

    fn mul_inv(&self)->Self {
        1.0 / self
    }
}
impl Ring for f32 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;

    fn mul_inv(&self)->Self {
        1.0 / self
    }
}
