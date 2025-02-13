#![allow(dead_code)]
mod complex;
use std::ops::Index;
use std::ops::IndexMut;

use complex::CartesianComplex;
use complex::PolarComplex;
use complex::tcomplex::TComplex;

use num_traits::one;
use num_traits::zero;
use num_traits::Float;
use num_traits::FloatConst;

use ::rand;
use rand::rng;
use rand::Rng;

use macroquad::prelude::*;
use crate::miniquad::window;



struct Qubit<T>([T; 2]);

impl<T> Index<usize> for Qubit<T>{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<T> IndexMut<usize> for Qubit<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Float + FloatConst + rand::distr::uniform::SampleUniform + std::fmt::Display> Qubit<PolarComplex<T>>{
    fn init() -> Self {
        let mut rng = rng();
        let arg1 = rng.random_range(T::zero()..=T::TAU());
        let norm1_squared: T = rng.random_range(T::zero()..=T::one());
        let norm2_squared: T = T::one() - norm1_squared;
        let arg2 = rng.random_range(T::zero()..=T::TAU());

        Self([
            PolarComplex::<T>::new(norm1_squared.sqrt(), arg1),
            PolarComplex::<T>::new(norm2_squared.sqrt(), arg2),
        ])
    }
    fn print(&self){
        println!("|φ⟩={}|0⟩+{}|1⟩", self[0], self[1]);
    }
    fn measure(&self){
        let prob_0 = self[0].norm().powi(2);
        println!("P(|0⟩)= {}, P(|1⟩)= {}", prob_0, T::one()-prob_0)
    }
    fn collapse(&self)->Self{
        let mut thrd = rng();
        let decider: T = thrd.random_range(T::zero()..=one());
        if decider<self[0].norm(){
            Self([
                PolarComplex::<T>::new(T::one(), zero()),
                PolarComplex::<T>::new(zero(), zero()),
            ])
        }else if decider > self[0].norm(){
            Self([
                PolarComplex::<T>::new(zero(), zero()),
                PolarComplex::<T>::new(T::one(), zero()),
            ])
        } else {
            let pool: [(T, T); 2] = [(zero(), one()), (one(), zero())];
            let decider2: usize = thrd.random_range(1..=2);
            let modifier = pool[decider2-1];
            Self (
                [
                    PolarComplex::<T>::new(modifier.0, zero()),
                    PolarComplex::<T>::new(modifier.1, zero())
                ]
            )
        }
    }
}
impl<T: Float + FloatConst + rand::distr::uniform::SampleUniform + std::fmt::Display> Qubit<CartesianComplex<T>>{
    fn init()->Self{
        let mut rng = rng();
        let arg1 = rng.random_range(T::zero()..=T::TAU());
        let norm1_squared: T = rng.random_range(T::zero()..=T::one());
        let norm2_squared: T = T::one() - norm1_squared;
        let arg2 = rng.random_range(T::zero()..=T::TAU());

        let pivot = [
            PolarComplex::<T>::new(norm1_squared.sqrt(), arg1),
            PolarComplex::<T>::new(norm2_squared.sqrt(), arg2),
        ];
        return Qubit([pivot[0].into(), pivot[1].into()]);
    }
    fn print(&self){
        println!("|φ⟩={}|0⟩+{}|1⟩", self[0], self[1]);
    }
}

async fn view_graph<T: TComplex<K>, K: Float>(qbit: Qubit<T>){
    let theta = (2.0)*(qbit[0].norm()).acos().to_f32().unwrap();
    let phi=qbit[1].get_angle().to_f32().unwrap();
    let vec = vec3(theta.sin() * phi.cos(),theta.sin() * phi.sin(),theta.cos());
    const ORIGIN3D: Vec3 = vec3(0.0, 0.0, 0.0);
    let mut camera = Camera3D {
        position: vec3(3.0, 3.0, 3.0),  // Camera pos (3.0, 3.0, 3.0)
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };
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
        if is_key_down(KeyCode::Q) {window::quit(); break;}

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
async fn main(){
    let nqubit= Qubit::<PolarComplex<f64>>::init();
    nqubit.print();
    nqubit.measure();
    view_graph(nqubit).await;
}