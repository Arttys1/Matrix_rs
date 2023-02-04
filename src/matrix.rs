use std::{
    fmt::Display,
    ops::{Index, Add, Sub, IndexMut, Mul, Div},
};
use anyhow::{Result, anyhow};

#[derive(Clone, Copy)]
pub struct Matrix<const N: usize, const M: usize> {
    tab: [[f32; M]; N],
}

///
/// method with matrix of size (N, M)
/// The numbre of line and column are not necessery equal
/// 
impl<const N: usize, const M: usize> Matrix<N, M> {
    ///
    /// Construct a matrix full of 0
    /// 
    /// let m = Matrix::<3; 3>::zero();
    ///  0  0  0 
    ///  0  0  0
    ///  0  0  0
    /// 
    pub fn zero() -> Self {
        Self { tab: [[0f32; M]; N] }
    }

    ///
    /// Construct a matrix full of a given value
    /// 
    /// let m = Matrix::<3; 3>::value(3);
    ///  3  3  3 
    ///  3  3  3
    ///  3  3  3
    /// 
    pub fn value(value : f32) -> Self  {
        Self { tab: [[value; M]; N] }
    }

    pub fn construct(tab: [[f32; M]; N]) -> Self {
        Self { tab }
    }

    pub fn from<const U: usize, const V: usize>(other : &Matrix<N, M>) -> Matrix<U, V> {
        let mut m = Matrix::<U, V>::zero();
        let line = U.min(N);
        let column = V.min(M);

        for i in 0..line {
            for j in 0..column {
                m[i][j] = other[i][j];
            }
        }

        m
    }

    ///
    /// Method who return a copy of the values of the matrix in a 2D Vector
    /// 
    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        let mut v = Vec::<Vec<f32>>::with_capacity(N);
        for i in 0..N {
            v.push(Vec::<f32>::with_capacity(M));
            for j in 0..M {
                v[i].push(self[i][j]);
            }
        }

        v
    }

    ///
    /// return the number of line
    /// 
    pub fn line(&self) -> usize { N }

    ///
    /// return the number of column
    /// 
    pub fn column(&self) -> usize { M }

    ///
    /// return the number of elements
    /// 
    pub fn len(&self) -> usize { M * N }

    pub fn add(this: &Matrix<N, M>, other: &Matrix<N, M>) -> Matrix<N, M> {
        let mut m = Matrix::zero();
        for i in 0..N {
            for j in 0..M {
                m.tab[i][j] = this.tab[i][j] + other.tab[i][j];
            }
        }
        m
    }

    pub fn negative(&self) -> Matrix<N, M> {
        let mut m = Matrix::zero();
        for i in 0..N {
            for j in 0..M {
                m[i][j] = -self[i][j];
            }
        }
        m
    }

    pub fn tranpose(&self) -> Matrix<M, N> {
        let mut tab: [[f32; N]; M] = [[0f32; N]; M];

        for i in 0..N {
            for j in 0..M {
                tab[j][i] = self[i][j];
            }
        }
        Matrix::<M, N>::construct(tab)
    }
    
}

impl<const N: usize> Matrix<N, N> {
    ///
    /// Return the identity matrix
    /// 
    pub fn identity() -> Self {
        let mut m = Matrix::zero();

        for i in 0..N {
            m[i][i] = 1f32;
        }

        m
    }

    ///
    /// method who return the determinant of the matrix
    /// 
    pub fn det(&self) -> f32 {
        let tab = self.to_vec();
        sub_det(&tab)
    }

    ///
    /// Return the comatrix associate to the matrix
    /// 
    pub fn comatrix(&self) -> Matrix<N, N> {
        let mut tab : [[f32; N]; N] = [[0f32; N]; N];
        for i in 0..N {
            for j in 0..N {
                let sub_matrix = extract(&self.to_vec(), i, j);
                let mut cofacteur = sub_det(&sub_matrix);
                cofacteur *= (-1i32).pow((i+j) as u32) as f32;
                tab[i][j] = cofacteur;
            }
        }
        Matrix::construct(tab)
    }

    ///
    /// Method who return the inverse Matrix
    /// 
    pub fn inverse(&self) -> Result<Matrix<N, N>> {
        let det = self.det();
        if det == 0.0 {
            return Err(anyhow!("matrix is uninversible. det == 0"));
        }

        let comatrix = self.comatrix();
        let trans_comatrix = comatrix.tranpose();
        Ok(trans_comatrix / det)
    }

}

pub(crate) fn sub_det(tab: &Vec<Vec<f32>>) -> f32 {
    if tab.len() == 2 {
        return tab[0][0] * tab[1][1] - tab[0][1] * tab[1][0];
    }

    let mut det = 0.0;
    for j in 0..tab.len(){ //taille n-1
        let b = extract(&tab, 0, j);
        let bsign = (if j % 2 == 0 { 1 } else { -1 }) as f32;
        let bvalue = tab[0][j];
        let bdet = sub_det(&b);
        det += bsign * bvalue * bdet;
    }
    det
}

pub(crate) fn extract(tab: &Vec<Vec<f32>>, line: usize, column: usize) -> Vec<Vec<f32>> {
    let l = tab.len();
    let col = tab[0].len();
    let mut v = Vec::<Vec<f32>>::with_capacity(l - 1);
    for _ in 0..v.capacity() {
        v.push(Vec::<f32>::with_capacity(col - 1));
    }
    let mut index_line = 0;
    for i in 0..l {
        if i != line {            
            for j in 0..col {                
                if j != column {                        
                    v[index_line].push(tab[i][j]); 
                }
            }
            index_line += 1;
        }            
    }
    v
}

impl<const N: usize, const M: usize> Display for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..N {
            for j in 0..M {
                let value = self.tab[i][j];
                let result = write!(f, " {} ", value);
                if let Some(error) = result.err() {
                    return Err(error);
                }
            }
            if let Some(error) = writeln!(f).err() {
                return Err(error);
            }
        }        

        Ok(())
    }
}

impl<const N: usize, const M: usize, const V: usize> Mul<Matrix<M, V>> for Matrix<N, M>
{
    type Output = Matrix<N, V>;
    ///
    /// Implement the matrix multiplication.
    /// The number of column of the first matrix must be equal 
    /// to the number of line of the second matrix.
    /// 
    fn mul(self, second: Matrix<M, V>) -> Self::Output {
        let mut m : Matrix<N, V> = Matrix::zero();

        for line_self in 0..N {           //for each line of the first matrix 
            for col_rhs in 0..V {         //for each column of the second matrix
                let mut value = 0f32;
                for current_index in 0..N {   //we browse the column of the first and the line of the second
                    value += self[line_self][current_index] * second[current_index][col_rhs];
                }
                m[line_self][col_rhs] = value;
            }
        }

        m
    }
}


impl<const N: usize, const M: usize> Add for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix::add(&self, &rhs)
    }
}

impl<const N: usize, const M: usize> Sub for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.negative()
    }
}

impl<const N: usize, const M: usize> Mul<f32> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn mul(self, rhs: f32) -> Self::Output {        
        let mut tab = [[0f32; M]; N];
        for i in 0..N {
            for j in 0..M {
                tab[i][j] = self[i][j] * rhs;
            }
        }
        Matrix::construct(tab)
    }
}  

impl<const N: usize, const M: usize> Div<f32> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn div(self, rhs: f32) -> Self::Output {        
        let mut tab = [[0f32; M]; N];
        for i in 0..N {
            for j in 0..M {
                tab[i][j] = self[i][j] / rhs;
            }
        }
        Matrix::construct(tab)
    }
}  

impl<const N: usize, const M: usize> Index<usize> for Matrix<N, M> {
    type Output = [f32; M];

    fn index(&self, line: usize) -> &Self::Output {
        &self.tab[line]
    }
}

impl<const N: usize, const M: usize> IndexMut<usize> for Matrix<N, M> {
    fn index_mut(&mut self, line: usize) -> &mut Self::Output {
        &mut self.tab[line]
    }
}