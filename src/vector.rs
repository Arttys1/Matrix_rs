use std::{
    ops::{Index, IndexMut, Add, Sub, Mul, Div},
    fmt::Display
};

#[allow(dead_code)]

pub struct Vector<T, const N: usize> 
where T : Copy + Default
{
    tab: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where T : Copy + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn value(val: T) -> Self{
        Self { tab: [val; N], }
    }
    pub fn len(&self) -> usize {
        N
    }
    pub fn to_vec(&self) -> Vec<T> {
        self.tab.to_vec()
    }
    pub fn as_ref(&self) -> &[T] {
        self.tab.as_ref()
    }
}

impl<T, const N: usize> Add for Vector<T, N> 
where T : Copy + Default + Add<Output = T>
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vector::<T, N>::new();        
        for i in 0..N {
            res[i] = self[i] + rhs[i];
        }
        res
    }
}

impl<T, const N: usize> Sub for Vector<T, N> 
where T : Copy + Default + Sub<Output = T>
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vector::<T, N>::new();        
        for i in 0..N {
            res[i] = self[i] - rhs[i];
        }
        res
    }
}

impl<T, const N: usize> Mul for Vector<T, N> 
where T : Copy + Default + Mul<Output = T>
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Vector::<T, N>::new();        
        for i in 0..N {
            res[i] = self[i] * rhs[i];
        }
        res
    }
}

impl<T, const N: usize> Div for Vector<T, N> 
where T : Copy + Default + Div<Output = T>
{
    type Output = Vector<T, N>;

    fn div(self, rhs: Self) -> Self::Output {
        let mut res = Vector::<T, N>::new();        
        for i in 0..N {
            res[i] = self[i] / rhs[i];
        }
        res
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> 
where T : Copy + Default,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tab[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> 
where T : Copy + Default,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tab[index]
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where T : Display + Copy + Default
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;
        for i in 0..N {
            write!(f, "{} ", self[i])?;
        }
        writeln!(f, " ]")?;
        Ok(())
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where T : Default + Copy + Default
{
    fn default() -> Self {
        Self { tab: [T::default(); N], }
    }
}