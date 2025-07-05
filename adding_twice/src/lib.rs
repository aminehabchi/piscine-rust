pub fn twice<T: std::ops::Add<Output = T> + Copy>(
    F: impl (Fn(T) -> T) + 'static
) -> impl (Fn(T) -> T) + 'static {
    let s = move |T| F(F(T));
    move |a: T| s(a)
}

pub fn add_curry<T: std::ops::Add<Output = T> + Copy>(x: T) -> impl Fn(T) -> T {
    move |a: T| a + x
}
