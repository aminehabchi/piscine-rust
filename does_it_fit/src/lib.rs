mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let area = rectangle_area(x, y);
    match objects {
        GeometricalShapes::Square => square_area(a) * times > area,
        GeometricalShapes::Circle => circle_area(a) * times as f64 > area as f64,
        GeometricalShapes::Rectangle => rectangle_area(a, b) * times > area,
        GeometricalShapes::Triangle => triangle_area(a, b) * times as f64 > area as f64,
    }
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let box_v = parallelepiped_volume(x, y, z);
    match objects {
        GeometricalVolumes::Cube => cube_volume(a) * times > box_v,
        GeometricalVolumes::Sphere => sphere_volume(a) * times as f64 > box_v as f64,
        GeometricalVolumes::Cone => cone_volume(a, b) * times as f64 > box_v as f64,
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b) * times as f64 > box_v as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) * times > box_v,
    }
}
