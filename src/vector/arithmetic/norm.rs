use super::{Vector, Field};

impl<K: Field, const SIZE: usize> Vector<K, SIZE> {
    pub fn norm_1(&self) -> K {
        self.iter().fold(K::default(), |acc, x| acc + *x)
    }
    pub fn norm_inf(&self) -> K
        where K:Ord {
        self.iter().map(|x| {if *x < K::default()
            {*x - *x - *x} else {*x}
        }).max().unwrap()
    }
    pub fn norm(&self) -> f32
        where K: Into<f32>
    {
        self.iter().fold(K::default(), |acc, x| {acc + *x * *x}).into().sqrt()
    }
}

