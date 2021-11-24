pub trait Reflect {
    fn reflect(&self, normal: Self) -> Self;
}

pub trait Refract {
    type Scalar;

    fn refract(&self, normal: Self, eta_in_over_eta_out: Self::Scalar) -> Self;
}
