use core::fmt;
use std::{fmt::Debug, ops::{Add, Mul}};

#[derive(PartialEq, Clone, Copy)]
enum Forms{
    Bin, Exp, Coords
}
impl std::fmt::Display for Forms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bin => write!(f, "Bin"),
            Self::Coords => write!(f, "Coords"),
            Self::Exp => write!(f, "Exp")
        }
    }
}

struct Vec2x1<T>(T, T);
impl<T> Vec2x1<T> where T: std::ops::Mul<Output = T> + std::clone::Clone, {
    fn new(a: T, b: T)->Self{
        Vec2x1(a, b)
    }
    fn scale(mut self, k: T){
        self.0=self.0*k.clone();
        self.1=self.1*k;
    }
}
impl<T> Add for Vec2x1<T> where T: Add<Output=T>, {
    type Output = Vec2x1<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2x1::<T>(self.0+rhs.0, self.1+rhs.1)
    }
}
impl<T> Debug for Vec2x1<T> where T: fmt::Debug + fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)   
    }
}


struct Complex {
    values: Vec2x1<f32>,
    rep: Forms
}
impl Complex {
    fn new(a: f32, b: f32, form: Forms)->Self{
        Complex {
            values: Vec2x1::<f32>::new(a, b),
            rep: form
        }
    }
    fn norm(&self)->f32{
        (self.values.0.powi(2)+self.values.1.powi(2)).sqrt()
    }
    fn translate_to(&mut self, to: Forms){
        if (self.rep!=to){
            let theta = (self.values.1).atan2(self.values.0);
            if (self.rep==Forms::Bin || self.rep==Forms::Coords) && to==Forms::Exp{
                self.values.0=self.norm();
                self.values.1=theta;
            } else if self.rep==Forms::Exp && ((Forms::Coords==to) || (Forms::Bin==to)){
                self.values.1=self.values.0*(theta.cos());
                self.values.0=self.values.0*(theta.sin());
            }
            self.rep=to.clone();
            /*Note that; Bin <-> Coords in both ways are unnecesary, because (a, b)<->a+bi */
        }
    }
}
impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rep {
            Forms::Bin => {
                let sign = if self.values.1>=0.0 {"+"} else {""};
                write!(f, "{}{}{}i", self.values.0, sign, self.values.1)
            },
            Forms::Exp => {write!(f, "{}e^({}i)", self.values.0, self.values.1)},
            Forms::Coords => {write!(f, "({}, {})", self.values.0, self.values.1)}
        }
    }
}
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rep {
            Forms::Bin => {
                let sign = if self.values.1>=0.0 {"+"} else {""};
                write!(f, "{}{}{}i", self.values.0, sign, self.values.1)
            },
            Forms::Exp => {write!(f, "{}e^({}i)", self.values.0, self.values.1)},
            Forms::Coords => {write!(f, "({}, {})", self.values.0, self.values.1)}
        }
    }
}
impl Clone for Complex {
    fn clone(&self) -> Self {
        Complex::new(self.values.0, self.values.1, self.rep)
    }
}
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        if (self.rep==Forms::Bin && rhs.rep==Forms::Bin) || (self.rep==Forms::Coords && rhs.rep==Forms::Coords) {
            Complex::new((self.values.0*rhs.values.1-self.values.1*rhs.values.1), (self.values.0*rhs.values.1+self.values.1*rhs.values.0), self.rep)
        } else if self.rep==Forms::Exp && rhs.rep==Forms::Exp{
            Complex::new((self.values.0*rhs.values.0).abs(), self.values.1+rhs.values.1, Forms::Exp)
        } else {
            assert!(false, "Cannot Multiply two Complex in different Forms: {} * {}", self.rep, rhs.rep);
            Complex::new(1.0, 1.0, Forms::Bin)
        }
    }
}

struct Qubit(Vec2x1<Complex>);
impl Qubit {
    fn new(z: Complex, z2: Complex)->Self{
        return Qubit(Vec2x1(z, z2));
    }
}
impl Debug for Qubit{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|φ⟩={:#?}", self.0)
    }
}
impl ToString for Qubit {
    fn to_string(&self) -> String {
        format!("|ϕ⟩=({})|0⟩+({})|1⟩", self.0.0, self.0.1)
    }
}
fn main() {
    let mut zerocomplex= Complex::new(1.0, 1.0, Forms::Bin);
    let mut onecomplex: Complex = Complex::new(-3.0, -2.0, Forms::Bin);
    let nqubit = Qubit::new(zerocomplex, onecomplex); // User maybe doesnt create a valid Qubit
    println!("{}", nqubit.to_string());
}
