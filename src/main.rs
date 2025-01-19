use core::fmt;
use std::{fmt::{Debug, LowerExp},ops::{Add, Mul}};
use rand::{self, Rng};
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

#[derive(Clone, Copy)]
struct Vec2x1<T>(T, T);
impl<T> Vec2x1<T> where T: std::ops::Mul<Output = T> + std::clone::Clone, {
    fn new(a: T, b: T)->Self{
        Vec2x1(a, b)
    }
    // fn scale(mut self, k: T){
    //     self.0=self.0*k.clone();
    //     self.1=self.1*k;
    // }
}
impl<T> Add for Vec2x1<T> where T: Add<Output=T>, {
    type Output = Vec2x1<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2x1::<T>(self.0+rhs.0, self.1+rhs.1)
    }
}
impl<T> Mul for Vec2x1<T> where T: Mul<Output=T> + Add<Output = T>,{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0*rhs.0+self.1*rhs.1
    }

}
impl<T> Debug for Vec2x1<T> where T: fmt::Debug + fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)   
    }
}

#[derive(Copy)]
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
        if self.rep!=Forms::Exp{
            (self.values.0.powi(2)+self.values.1.powi(2)).sqrt()
        } else {
            self.values.0
        }
    }
    fn scale(&mut self, k: f32){
        if (self.rep==Forms::Bin || self.rep==Forms::Coords){
            self.values.0*=k; self.values.1*=k;
        } else {
            if k!=1.0{
                let (sign, shift): (f32, f32) = if k<0.0 {
                    (k*(-1.0), std::f32::consts::PI)
                } else {
                    (k, 0.0)
                };
                let value = self.to_owned()*Complex::new(sign, shift, Forms::Exp);
                *self=value;
            }
            self.values.1=self.values.1.rem_euclid(std::f32::consts::PI*2.0);
        }
    }
    fn translate_to(&mut self, to: Forms){
        if self.rep!=to{
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
impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        if self.rep==rhs.rep && (self.rep==Forms::Bin || self.rep==Forms::Coords){
            Complex::new(self.values.0+rhs.values.0, self.values.1+rhs.values.1, self.rep.clone())
        } else {
            assert!(false, "Cannot Add <Complex> in different forms: <{}> and <{}>", self.rep, rhs.rep);
            Complex::new(1.0, 1.0, Forms::Bin)
        }
    }
}
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        if (self.rep==Forms::Bin && rhs.rep==Forms::Bin) || (self.rep==Forms::Coords && rhs.rep==Forms::Coords) {
            Complex::new(self.values.0*rhs.values.1-self.values.1*rhs.values.1, self.values.0*rhs.values.1+self.values.1*rhs.values.0, self.rep)
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
    fn init(form: Forms)->Self {
        let mut a = rand::thread_rng();
        let state1: f32 = a.gen();
        let mut get_arg = ||->f32{a.gen_range(0.0..=(std::f32::consts::PI*2.0))};
        let arg1: f32 = get_arg();
        let mut toreturn = Qubit::new(Complex::new(state1, arg1, Forms::Exp), Complex::new((1.0-state1.powi(2)).sqrt(), get_arg(), Forms::Exp));
        match form {
            Forms::Bin=> {
                toreturn.0.0.translate_to(Forms::Bin);
                toreturn.0.1.translate_to(Forms::Bin);
            }
            Forms::Coords=> {
                toreturn.0.0.translate_to(Forms::Coords);
                toreturn.0.1.translate_to(Forms::Coords);
            }
            Forms::Exp => {
                toreturn.0.0.scale(1.0); //Seems to be unnecesary, but it does theta % pi*2
            }
        }
        toreturn
    }
    fn collapse(&self)->Self{
        let mut thrd = rand::thread_rng();
        let decider: f32 = thrd.gen();
        if decider<self.0.0.norm(){
            Qubit::new(Complex::new(1.0, 0.0, Forms::Bin), Complex::new(0.0, 0.0, Forms::Bin))
        }else if decider > self.0.0.norm(){
            Qubit::new(Complex::new(0.0, 0.0, Forms::Bin), Complex::new(1.0, 0.0, Forms::Bin))
        } else {
            let pool: [(f32, f32); 2] = [(0.0, 1.0), (1.0, 0.0)];
            let decider2: usize = thrd.gen_range(1..=2);
            let modifier = pool[decider2-1];
            Qubit::new(Complex::new(modifier.0, 0.0, Forms::Bin), Complex::new(modifier.1, 0.0, Forms::Bin))
        }
    }
    fn hadamard(&mut self){
        let alpha = self.0.0;
        let mut beta = self.0.1;

        let k = 1.0/(2.0_f32.sqrt());
        self.0.0=alpha+beta;
        beta.scale(-1.0);
        self.0.1=alpha+beta;
        let norm = (self.0.0+self.0.1).norm();
        self.0.0.scale(k*norm);
        self.0.1.scale(k*norm);
    }
    fn px(&mut self){
        let saved = self.0.0;
        self.0.0=self.0.1;
        self.0.1=saved;
    }
    fn py(&mut self){
        let saved = self.0.0;
        self.0.0=self.0.1*Complex::new(0.0, -1.0, Forms::Bin);
        self.0.1=saved*Complex::new(0.0, 1.0, Forms::Bin);
    }
    fn pz(&mut self){
        if self.0.1.rep==Forms::Exp {
            self.0.1=self.0.1*Complex::new(1.0, std::f32::consts::PI, Forms::Exp);
        } else {
            self.0.1.scale(-1.0);
        }
    }
    fn phaseshift(&mut self, phi: f32){
        self.0.1.translate_to(Forms::Exp);
        self.0.1=self.0.1 * Complex::new(1.0, phi, Forms::Exp);
        self.0.1.translate_to(self.0.0.rep);
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
    let mut cmplx1: Complex = Complex::new(2.0, 3.14, Forms::Exp);
    cmplx1.scale(-4.0);
    println!("{:#?}", cmplx1);
    //Ive to solve translate_to()
}
