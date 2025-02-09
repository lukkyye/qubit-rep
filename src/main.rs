#![allow(dead_code)]
use ::rand::{self, Rng};
use std::{f32::NAN, ops::{Add, Index, IndexMut, Mul, Sub}};
use macroquad::{miniquad::window, prelude::*};

#[derive(PartialEq, Clone, Copy)]
enum Forms {
    Bin, Exp, Coords
}
impl std::fmt::Display for Forms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bin => write!(f, "Bin"),
            Self::Coords=>write!(f, "Coords"),
            Self::Exp=>write!(f, "Exp")
        }
    }
}
#[derive(Clone, Copy)]
struct Complex {
    values: (f32, f32),
    rep: Forms
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
        self+pivot
    }
}
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.rep==rhs.rep{
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
impl Complex {
    fn new(a: f32, b: f32, _rep: Forms)->Self {
        Complex {
            values: (a, b),
            rep: _rep
        }
    }
    fn print(&self){
        println!("{}", self)
    }
    fn scale(&mut self, k: f32){
        if self.rep==Forms::Bin || self.rep==Forms::Coords {
            self.values.0=self.values.0*k; self.values.1=self.values.1*k;
        } else {
            if k!=1.0 && k!=-1.0{
                let (sign, shift): (f32, f32) = if k < 0.0 {
                    (k*(-1.0), std::f32::consts::PI)
                } else {
                    (k, 0.0)
                };
                let value = self.clone()*Complex::new(sign, shift, Forms::Exp);
                *self=value;
            }
            self.values.1=self.values.1.rem_euclid(std::f32::consts::PI*2.0);
        }
    }
    fn norm(&self)->f32 {
        if self.rep!=Forms::Exp{
            (self.values.0.powi(2)+self.values.1.powi(2)).sqrt()
        } else {
            self.values.0
        }
    }
    fn translate_to(&mut self, to: Forms){
        if self.rep!=to{
            if to==Forms::Exp {
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
    fn get_arg(&self)->f32 {
        assert!(self.rep==Forms::Exp);
        self.values.1
    }
}

struct Qubit([Complex; 2]);
impl Index<usize> for Qubit {
    type Output = Complex;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Qubit {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl Qubit {
    fn new(z: Complex, z2: Complex)->Self{
        Qubit([z, z2])
    }
    fn init(rep: Forms)->Self{
        let mut a = rand::thread_rng();
        let state1: f32 = a.gen();
        let complex = Complex::new(state1, a.gen_range(0.0..=std::f32::consts::PI*2.0), Forms::Exp);
        let complex2 = Complex::new((1.0-state1.powi(2)).sqrt(), a.gen_range(0.0..=std::f32::consts::PI*2.0), Forms::Exp);
        let mut toreturn = Qubit::new(complex, complex2);
        match rep {
            Forms::Bin => {
                toreturn.translate_to(Forms::Bin);
            }
            Forms::Coords => {
                toreturn.translate_to(Forms::Coords);
            }
            _=>{
                toreturn[0].scale(1.0);
                toreturn[1].scale(1.0);
            }
        }
        toreturn
    }
    fn translate_to(&mut self, form: Forms){
        self[0].translate_to(form);
        self[1].translate_to(form);
    }
    fn print(&self){
        println!("|ϕ⟩=({})|0⟩+({})|1⟩", self[0], self[1]);
    }
    fn measure(&self){
        let prob_0 = self[0].norm().powi(2);
        println!("P(|0⟩)= {}, P(|1⟩)= {}", prob_0, 1.0-prob_0)
    }
    fn collapse(&self)->Self{
        let mut thrd = rand::thread_rng();
        let decider: f32 = thrd.gen();
        if decider<self[0].norm(){
            Qubit::new(Complex::new(1.0, 0.0, Forms::Bin), Complex::new(0.0, 0.0, Forms::Bin))
        }else if decider > self[0].norm(){
            Qubit::new(Complex::new(0.0, 0.0, Forms::Bin), Complex::new(1.0, 0.0, Forms::Bin))
        } else {
            let pool: [(f32, f32); 2] = [(0.0, 1.0), (1.0, 0.0)];
            let decider2: usize = thrd.gen_range(1..=2);
            let modifier = pool[decider2-1];
            Qubit::new(Complex::new(modifier.0, 0.0, Forms::Bin), Complex::new(modifier.1, 0.0, Forms::Bin))
        }
    }
    fn hadamard(&mut self){
        let mut buf1 = self[0]+self[1]; buf1.scale(1.0/(2.0_f32.sqrt()));
        let mut buf2 = self[0]-self[1]; buf2.scale(1.0/(2.0_f32.sqrt()));
        self[0]=buf1;
        self[1]=buf2;
    }
    fn px(&mut self){
        let saved = self[0];
        self[0]=self[1];
        self[1]=saved;
    }
    fn py(&mut self){
        let saved = self[0];
        self[0]=self[1]*Complex::new(0.0, -1.0, Forms::Bin);
        self[1]=saved*Complex::new(0.0, 1.0, Forms::Bin);
    }
    fn pz(&mut self){
        if self[1].rep==Forms::Exp {
            self[1]=self[1]*Complex::new(1.0, std::f32::consts::PI, Forms::Exp);
        } else {
            self[1].scale(-1.0);
        }
    }
    fn phase_shift(&mut self, phi: f32){
        let actual_rep = self[0].rep;
        self[1].translate_to(Forms::Exp);
        self[1]=self[1] * Complex::new(1.0, phi, Forms::Exp);
        self[1].translate_to(actual_rep);
    }
}

async fn view_graph(nqbit: &mut Qubit){
    assert!(nqbit[0].rep==Forms::Exp, "Error: Qubit form must be Forms::Exp, you should try to view_graph(<qubit_instance>.translate_to(Forms::Exp))");
    let theta = 2.0*(nqbit[0].norm()).acos();
    let phi=nqbit[1].get_arg()-nqbit[0].get_arg();
    println!("{}", nqbit[0].norm().powi(2)+nqbit[1].norm().powi(2));
    nqbit.print();
    nqbit.measure();
    
    
    let vec = vec3(theta.sin() * phi.cos(),theta.sin() * phi.sin(),theta.cos());
    println!("({},{},{})",vec[0], vec[1], vec[2]);
    const ORIGIN3D: Vec3 = vec3(0.0, 0.0, 0.0);
    let mut camera = Camera3D {
        position: vec3(3.0, 3.0, 3.0),  // Camera pos (3.0, 3.0, 3.0)
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    // Default presets:
    //==============
    let camera_increment: f32= 0.1;
    let defined_bounds=vec![vec3(-10.0,0.0,0.0), vec3(0.0,-10.0,0.0), vec3(0.0,0.0,-10.0)];

    //==================
    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::W){camera.position.z+=camera_increment;}
        if is_key_down(KeyCode::S){camera.position.z-=camera_increment;}
        if is_key_down(KeyCode::D){camera.position.x+=camera_increment;}
        if is_key_down(KeyCode::A){camera.position.x-=camera_increment;}
        if is_key_down(KeyCode::Right){camera.position.y+=camera_increment;}
        if is_key_down(KeyCode::Left){camera.position.y-=camera_increment;}
        if is_key_down(KeyCode::Q) { window::quit(); break;}

        set_camera(&camera);
        //Axis
        draw_line_3d(defined_bounds[0],-defined_bounds[0], RED);
        draw_line_3d(defined_bounds[1], -defined_bounds[1], GREEN);
        draw_line_3d(defined_bounds[2], -defined_bounds[2], BLUE);

        //Vec fake arrow
        draw_sphere(vec, 0.05, None, RED);

        for i in -10..=10 {
            draw_sphere(vec3(i as f32, 0.0, 0.0), 0.02, None, RED);
            draw_sphere(vec3(0.0, i as f32, 0.0), 0.02, None, GREEN);
            draw_sphere(vec3(0.0, 0.0, i as f32), 0.02, None, BLUE);
        }
        
        // draw_line_3d(ORIGIN3D, vec3(3.0, 3.0, 3.0), MAGENTA); Camera position at v = 3.0 3.0 3.0
        
        //Bloch sphere vector
        draw_line_3d(ORIGIN3D, vec, WHITE);
        
        //Bloch sphere
        draw_sphere_wires(vec3(0.0, 0.0, 0.0), 1.0, None, Color::new(1.0, 1.0, 1.0, 0.5));
        set_default_camera();
        next_frame().await;
    };
    
}


#[macroquad::main("Bloch Sphere")]
async fn main() {
    let mut nqubit =  Qubit::init(Forms::Exp);
    let _ = view_graph(&mut nqubit).await;
}