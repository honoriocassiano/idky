pub trait Vector: Copy {
    type Scalar;

    fn zero() -> Self;

    fn length(&self) -> Self::Scalar;

    fn sq_length(&self) -> Self::Scalar;

    fn normalize(&mut self) -> &mut Self;

    fn normalized(&self) -> Self;

    fn dot(&self, v: Self) -> Self::Scalar;

    fn cross(&self, v: Self) -> Self;
}
