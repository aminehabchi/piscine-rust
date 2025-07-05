pub fn add_curry<T: std::ops::Add<Output = T> + Copy>(x: T) -> impl Fn(T) -> T {
    move |a: T| a + x
}
