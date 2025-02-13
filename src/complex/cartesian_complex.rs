use std::ops::{Add, Mul};
pub use crate::complex::tcomplex::TComplex;
use num_traits::Float;
use super::PolarComplex;

#[derive(Clone, Copy)]
pub struct CartesianComplex<T> {
    pub(crate)re: T, pub(crate) im: T
}
impl<T: PartialOrd + Float + std::fmt::Display> std::fmt::Display for CartesianComplex<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zero = T::zero();
        let sign: String = if self.im<zero{"-".to_string()}else{"+".to_string()};
        write!(f, "{}{}{}i", self.re, sign, self.im)
    }
}
impl<T: Float + std::fmt::Display> TComplex<T> for CartesianComplex<T>{
    fn new(a: T, b: T)->Self {
        CartesianComplex::<T>{re: a, im: b}
    }
    fn norm(&self)->T {
        return (self.im.powi(2)+self.re.powi(2)).sqrt()
    }
    fn normalize(&mut self) {
        let _norm = self.norm();
        self.im=self.im/_norm; self.re=self.re/_norm;
    }
    fn print(&self){
        println!("{}", self);
    }
    fn get_angle(&self)->T {
        (self.re/self.im).acos()
    }
}
impl<T: Float> From<PolarComplex<T>> for CartesianComplex<T> {
    fn from(value: PolarComplex<T>) -> Self {
        let theta = value.arg.tan();
        CartesianComplex::<T>{
            re: value.norm*theta.cos(),
            im: value.norm*theta.sin()
        }
    }
}
impl<T: Float + std::fmt::Display> Add for CartesianComplex<T>{
    type Output = CartesianComplex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        CartesianComplex::<T>::new(self.im+rhs.im, self.re+rhs.re)
    }
}
impl<T: Float + std::fmt::Display> Mul for CartesianComplex<T>{
    type Output = CartesianComplex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        CartesianComplex::<T>::new(self.re*rhs.re-self.im*rhs.im, self.re*rhs.im+self.im*rhs.re)
    }
}