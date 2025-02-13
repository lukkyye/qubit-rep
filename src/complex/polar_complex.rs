use std::ops::{Add, Mul};
pub use crate::complex::tcomplex::TComplex;
use num_traits::{Float, FloatConst, One, Zero};
use crate::CartesianComplex;


#[derive(Clone, Copy)]
pub struct PolarComplex<T> {
    pub(crate) norm: T, pub(crate) arg: T
}
impl<T: std::fmt::Display> std::fmt::Display for PolarComplex<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}e^({}i)", self.norm, self.arg)
    }
}
impl<T: Float + std::fmt::Display + num_traits::FloatConst> TComplex<T> for PolarComplex<T>{
    fn new(a: T, b: T)->Self {
        PolarComplex::<T>{
            norm: a,
            arg: b%num_traits::FloatConst::TAU()
        }
    }
    fn norm(&self)->T {
        self.norm
    }
    fn normalize(&mut self) {
        self.norm=One::one();
    }
    fn print(&self) {
        println!("{}", self);
    }
    fn get_angle(&self)->T {
        self.arg
    }
}

impl<T: Float + std::fmt::Display> From<CartesianComplex<T>> for PolarComplex<T>{
    fn from(value: CartesianComplex<T>) -> Self {
        PolarComplex::<T>{
            norm: value.norm(),
            arg: value.im.atan2(value.re)
        }
    }
}

impl<T: Float + std::fmt::Display> Add for PolarComplex<T>{
    type Output = PolarComplex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let pivot: CartesianComplex<T> = self.into();
        let pivot2: CartesianComplex<T> = rhs.into();
        return (pivot+pivot2).into();
    }
}
impl<T: Float + std::fmt::Display + FloatConst> Mul for PolarComplex<T>{
    type Output = PolarComplex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let tau: T = num_traits::FloatConst::TAU();
        let mut to_return = PolarComplex::<T>::new(self.norm*rhs.norm, (self.arg+rhs.arg)%tau);
        if to_return.norm<Zero::zero(){
            to_return.norm=-to_return.norm;
            to_return.arg=to_return.arg+num_traits::FloatConst::PI();
        }
        to_return
    }
}
