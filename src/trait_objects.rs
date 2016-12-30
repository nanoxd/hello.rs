// Trait objects

pub trait Bark {
    fn bark(&self) -> String;
}

pub struct Dog {
    pub breed: String
}

pub struct Wolf {
    color: String
}

impl Bark for Dog {
    fn bark(&self) -> String {
        format!("Dog {}: bark bark", self.breed)
    }
}

impl Bark for Wolf {
    fn bark(&self) -> String {
        format!("{} Wolf: woff woff", self.color)
    }
}

pub fn make_noise(x: &Bark) {
    x.bark();
}
