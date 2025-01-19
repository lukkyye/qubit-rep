use core::fmt;
use std::{f32::NAN, fmt::Debug, ops::{Add, Mul, Sub}};
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

    /// Returns the norm of the Complex
    /// 
    /// # Example
    /// ```rust
    /// let obj: Complex = Complex::new(2.0, 3.0, Forms::Bin);
    /// assert_eq!(obj.norm(), 15);
    /// ```
    fn norm(&self)->f32{
        if self.rep!=Forms::Exp{
            (self.values.0.powi(2)+self.values.1.powi(2)).sqrt()
        } else {
            self.values.0
        }
    }
    /// Given k, scales the Complex
    /// # Example
    /// ```rust
    /// let cmplx: Complex = Complex::new(2.0, std::f32::consts::PI/4, Forms::Exp);
    /// cmplx.scale(-3.0);
    /// assert_eq!(cmplx.norm(), 6.0);
    /// ```
    fn scale(&mut self, k: f32){
        if (self.rep==Forms::Bin || self.rep==Forms::Coords){
            self.values.0*=k; self.values.1*=k;
        } else {
            if k!=1.0 && k!=-1.0{
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
    /// Converts a Complex number from one form of representation to another.
    /// ### Forms:
    /// ```rust
    /// Forms::Bin // -> Binomial notation
    /// Forms::Exp // -> Exponential notation
    /// Forms::Coords // -> Cartesian notation
    /// ```
    /// # Example
    /// 
    /// ```rust
    /// let complex1: Complex = Complex::new(1.0, 0.0, Forms::Coords);
    /// complex1.translate_to(Forms::Exp); // (1.0, 0.0) -> 1e^(0i)
    /// complex1.translate_to(Forms::Bin); // 1.0e^(0.0i) -> 1.0 + 0.0i
    /// ```
    fn translate_to(&mut self, to: Forms){
        if self.rep!=to{
            if (to==Forms::Exp){
                let a=self.values.0;
                self.values.0=self.norm();
                self.values.1=self.values.1.atan2(a);
            } else {
                let norm= self.norm();
                self.values.0=norm*self.values.1.cos();
                self.values.1=norm*self.values.1.sin();
            }
            self.rep=to;
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
        if self.rep==rhs.rep {
            let mut buf1: Complex;
            let mut buf2: Complex;
            if self.rep == Forms::Exp {
                buf1=self; buf1.translate_to(self.rep);
                buf2=rhs; buf2.translate_to(self.rep);
            } else {
                buf1=self;
                buf2=rhs;
            }
            Complex::new(buf1.values.0+buf2.values.0, buf1.values.1+buf2.values.1, self.rep.clone())
        } else {
            assert!(false, "Cannot Sub <Complex> in different forms: <{}> + <{}>", self.rep, rhs.rep);
            Complex::new(NAN, NAN, Forms::Bin)
        }
    }
}
impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut pivot = rhs;
        pivot.scale(-1.0);
        self.add(pivot)
    }
}
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        if (self.rep==rhs.rep){
            if self.rep==Forms::Exp{
                let mut pivot = rhs.clone();
                pivot.translate_to(Forms::Exp);
                let shift= if pivot.values.0*self.values.0<0.0 {
                    std::f32::consts::PI
                } else {
                    0.0
                };
                Complex::new((pivot.values.0*self.values.0).abs(), pivot.values.1+self.values.1+shift, Forms::Exp)
            } else {
                Complex::new(self.values.0*rhs.values.0-rhs.values.1*self.values.1, self.values.0*rhs.values.1+self.values.1*rhs.values.0, self.rep)
            }
        } else {
            assert!(false, "Cannot multiply two complex in different Forms: <{}> * <{}>", self.rep, rhs.rep);
            Complex::new(NAN, NAN, Forms::Bin)
        }
    }
}

/// Qubit object
struct Qubit(Vec2x1<Complex>);
impl Qubit {
    /// Create a Qubit manually setting by your own risk z and z2 (not recommended)
    /// If you will use this, make sure that |z1|²+|z2|²=1
    /// # Example
    /// ```rust
    /// let qubit1: Qubit = Qubit::new(Complex::new(-0.32, 0.0, Forms::Bin), Complex::new(0.68, 0.0, Forms::Bin));
    /// assert_eq!(Qubit.0.0.norm()+Qubit.0.1.norm(), 1.0);
    /// ```
    fn new(z: Complex, z2: Complex)->Self{
        return Qubit(Vec2x1(z, z2));
    }
    fn print(&self){
        println!("{:#?}", self);
    }
    /// Initialize a Qubit automatically (random Complex numbers satisfying |z1|²+|z2|²=1)
    /// # Example
    /// ```rust
    /// let qubit1: Qubit = Qubit::init(Forms::Bin);
    /// ```
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
                toreturn.0.0.scale(1.0); //Seems unnecessary, but it does theta % pi*2
            }
        }
        toreturn
    }

    /// Prints the probabilities of measuring the system in its basis states
    /// 
    /// # Example
    /// ```rust
    /// let qubit1: Qubit = Qubit::init(Forms::Bin);
    /// qubit1.measure() // Output looks like: P(|0⟩)= 0.4923013, P(|1⟩)= 0.5076987
    /// ```
    fn measure(&self){
        let prob_0 = self.0.0.norm().powi(2);
        println!("P(|0⟩)= {}, P(|1⟩)= {}", prob_0, 1.0-prob_0);
    }
    /// Collapse the Qubit state to one of both basis states.
    /// # Example
    /// ```rust
    /// let qubit1: Qubit = Qubit::init(Forms::Bin);
    /// qubit1.collapse() 
    /// println!("{:#?}", qubit1); // Output looks like: |ϕ⟩=(-0.38470358-0.45738223i, 0.49093932+0.6338637i)
    /// ```
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
    /// Apply Hadamard gate
    /// 
    /// # Example
    /// ```rust
    /// let mut qubit1: Qubit = Qubit::init(Forms::Bin);
    /// qubit1.collapse(); // Now, the Qubit collapsed to a basis state
    /// qubit1.hadarmard(); // Here, the probabilities of collapsing into |0⟩ or |1⟩, are 50:50. 
    /// ```
    /// If you use hadamard again, you will return to the basis state again.
    fn hadamard(&mut self){
        let mut buf1 = self.0.0+self.0.1; buf1.scale(1.0/(2.0_f32.sqrt()));
        let mut buf2 = self.0.0-self.0.1; buf2.scale(1.0/(2.0_f32.sqrt()));
        self.0.0=buf1;
        self.0.1=buf2;
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
    fn phase_shift(&mut self, phi: f32){
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
}
