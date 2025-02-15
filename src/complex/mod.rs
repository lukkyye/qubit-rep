#![allow(dead_code)]
use std::ops::{Add, Mul, Sub};
use crate::float::Floats;

pub trait Complex<T>: Copy + Clone {
    fn new(a: T, b: T)->Self;
    fn print(&self);
    fn norm(&self)->T;
    fn scale(&mut self, k: T);
    fn arg(&self)->T;
}

#[derive(Clone, Copy)]
pub struct PolarComplex<T: Floats>(T, T);
impl<T: Floats> Complex<T> for PolarComplex<T>{
    fn new(a: T, b: T)->Self {
        Self(a, b%T::TAU())
    }
    fn norm(&self)->T {
        self.0
    }
    fn print(&self) {
        println!("{}e^({}i)", self.0, self.1);
    }
    fn scale(&mut self, k: T) {
        self.0=self.0*k;
    }
    fn arg(&self)->T {
        self.1
    }
}
impl<T: Floats> From<CartesianComplex<T>> for PolarComplex<T>{
    fn from(value: CartesianComplex<T>) -> Self {
        Self(value.norm(),value.1.atan2(value.0)%T::TAU())
    }
}
impl<T: Floats> Mul for PolarComplex<T>{
    type Output = PolarComplex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0*rhs.0, if self.0*rhs.0<T::zero(){
            self.1+rhs.1+T::PI()
        } else {self.1+rhs.1})
    }
}
impl<T: Floats> Add for PolarComplex<T>{
    type Output = PolarComplex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let piv1: CartesianComplex<T> = self.into();
        let piv2: CartesianComplex<T> = rhs.into();
        (piv1+piv2).into()
    }
}
impl<T: Floats> Sub for PolarComplex<T>{
    type Output = PolarComplex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let piv1: CartesianComplex<T> = self.into();
        let piv2: CartesianComplex<T> = rhs.into();
        (piv1-piv2).into()
    }
}
impl<T: Floats> std::fmt::Display for PolarComplex<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}e^({}i)", self.0, self.1)
    }
}

//=========================================
#[derive(Clone, Copy)]
pub struct CartesianComplex<T: Floats>(T, T);
impl<T: Floats> Complex<T> for CartesianComplex<T>{
    fn new(a: T, b: T)->Self {
        Self(a, b)
    }
    fn norm(&self)->T {
        (self.0*self.0+self.1*self.1).sqrt()
    }
    fn print(&self) {
        println!("({}, {})", self.0, self.1);
    }
    fn scale(&mut self, k: T) {
        self.0=self.0*k; self.1=self.1*k;
    }
    fn arg(&self)->T {
        self.1.atan2(self.0)
    }
}
impl<T: Floats> From<PolarComplex<T>> for CartesianComplex<T>{
    fn from(value: PolarComplex<T>) -> Self {
        Self(value.1.cos()*value.norm(),value.1.sin()*value.norm()) //(x, y) -> (rcos theta, rsin theta)
    }
}
impl<T: Floats> Mul for CartesianComplex<T>{
    type Output = CartesianComplex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0*rhs.0-self.1*rhs.1, self.0*rhs.1+self.1*rhs.0)
    }
}
impl<T: Floats> Add for CartesianComplex<T>{
    type Output = CartesianComplex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0+rhs.0, self.1+rhs.1)
    }
}
impl<T: Floats> Sub for CartesianComplex<T>{
    type Output = CartesianComplex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0-rhs.0, self.1-rhs.1)
    }
}
impl<T: Floats> std::fmt::Display for CartesianComplex<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}