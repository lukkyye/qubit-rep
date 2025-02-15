#![allow(dead_code)]
mod float;
mod complex;
mod qubit;
use complex::{Complex, PolarComplex};
use qubit::*;
use macroquad::{miniquad::window, prelude::*};


async fn view_graph(nqbit: &mut Qubit<PolarComplex<f32>>){
    let theta = 2.0*(nqbit.0.norm()).acos();
    let phi=nqbit.1.arg()-nqbit.0.arg();
    println!("{}", nqbit.0.norm().powi(2)+nqbit.1.norm().powi(2));
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
async fn main() {
    let mut nqubit: Qubit<PolarComplex<f32>> =  Qubit::init();
    nqubit.hadamard();
    let _ = view_graph(&mut nqubit).await;
}
