use num_traits::Float;


pub trait TComplex<K> where K: Float{
    fn new(a: K, b: K)->Self;
    fn norm(&self)->K;
    fn normalize(&mut self);
    fn print(&self);
    fn get_angle(&self)->K;
}
