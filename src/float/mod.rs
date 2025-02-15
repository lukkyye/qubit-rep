pub trait Floats: num_traits::Float + num_traits::FloatConst + rand::distr::uniform::SampleUniform + std::fmt::Display{}
impl Floats for f64{} impl Floats for f32{}