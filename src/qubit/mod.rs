#![allow(dead_code)]
use crate::complex::CartesianComplex;
use crate::float::Floats;
use crate::complex::Complex;
use crate::complex::PolarComplex;
use rand::Rng;

pub trait TQubit<T>{
    fn init()->Self;
    fn init0()->Self;
    fn init1()->Self;
    fn print(&self);
    fn measure(&self);
    fn hadamard(&mut self);
    fn px(&mut self);
    fn py(&mut self);
    fn pz(&mut self);
}

#[derive(Clone, Copy)]
pub struct Qubit<T>(pub T, pub T);
impl<T: Floats> TQubit<PolarComplex<T>> for Qubit<PolarComplex<T>>{
    fn init()->Self{
        let mut rng  = rand::rng();
        let arg1 = rng.random_range(T::zero()..=T::TAU());
        let norm1_squared: T = rng.random_range(T::zero()..=T::one());
        let norm2_squared: T = T::one() - norm1_squared;
        let arg2 = rng.random_range(T::zero()..=T::TAU());
        Self(PolarComplex::<T>::new(norm1_squared.sqrt(), arg1), PolarComplex::<T>::new(norm2_squared.sqrt(), arg2))
    }
    fn init0()->Self {
        Self(PolarComplex::<T>::new(T::one(), T::zero()), PolarComplex::<T>::new(T::zero(), T::zero()))
    }
    fn init1()->Self{
        Self(PolarComplex::<T>::new(T::zero(), T::zero()), PolarComplex::<T>::new(T::one(), T::zero()))

    }
    fn print(&self){
        println!("|φ⟩={}|0⟩+{}|1⟩", self.0, self.1);
    }
    fn measure(&self){
        let prob_0 = self.0.norm().powi(2);
        println!("P(|0⟩)= {}, P(|1⟩)= {}", prob_0, T::one()-prob_0)
    }
    fn hadamard(&mut self){
        let v1 = self.0+self.1;
        let v2= self.0-self.1;
        self.0=v1;
        self.1=v2;
        self.0.scale(T::one()/(T::one()+T::one()).sqrt());
        self.1.scale(T::one()/(T::one()+T::one()).sqrt());
    }
    fn px(&mut self){
        let saved = self.0;
        self.0=self.1;
        self.1=saved;
    }
    fn py(&mut self){
        let saved = self.0;
        self.0=self.1*PolarComplex::new(T::one(), T::PI());
        self.1=saved*PolarComplex::new(T::one(), T::TAU());
    }
    fn pz(&mut self){
        self.1=self.1*Complex::new(T::one(), T::PI());
    }
}

impl<T: Floats> TQubit<CartesianComplex<T>> for Qubit<CartesianComplex<T>>{
    fn init()->Self {
        let qbit_pivot = Qubit::<PolarComplex<T>>::init();
        qbit_pivot.into()
    }
    fn init0()->Self {
        Self(CartesianComplex::<T>::new(T::one(), T::zero()), CartesianComplex::<T>::new(T::zero(), T::zero()))
    }
    fn init1()->Self{
        Self(CartesianComplex::<T>::new(T::zero(), T::zero()), CartesianComplex::<T>::new(T::one(), T::zero()))

    }
    fn measure(&self) {
        let prob_0 = self.0.norm().powi(2);
        println!("P(|0⟩)= {}, P(|1⟩)= {}", prob_0, T::one()-prob_0)
    }
    fn print(&self) {
        println!("{}|0⟩ + {}|1⟩", self.0, self.1);
    }
    fn hadamard(&mut self) {
        let mut pivot: Qubit<PolarComplex<T>> = self.clone().into();
        pivot.hadamard();
        self.0=pivot.0.into();
        self.1=pivot.1.into();
    }
    fn px(&mut self){
        let saved = self.0;
        self.0=self.1;
        self.1=saved;
    }
    fn py(&mut self){
        let saved = self.0;
        self.0=self.1*CartesianComplex::new(-T::one(), T::zero());
        self.1=saved;
    }
    fn pz(&mut self){
        self.1.scale(-T::one());
    }
}
impl<T: Floats> From<Qubit<PolarComplex<T>>> for Qubit<CartesianComplex<T>>{
    fn from(value: Qubit<PolarComplex<T>>) -> Self {
        Self(value.0.into(), value.1.into())
    }
}
impl<T: Floats> From<Qubit<CartesianComplex<T>>> for Qubit<PolarComplex<T>>{
    fn from(value: Qubit<CartesianComplex<T>>) -> Self {
        Self(value.0.into(), value.1.into())
    }
}