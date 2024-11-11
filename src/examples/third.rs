use crate::utils::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::{Lambertian, Metal},
    point3::Point3,
    sphere::Sphere,
};

pub fn run() {
    // Materials
    let material_ground = Lambertian {
        albedo: Color::color(0.8, 0.8, 0.),
    };
    let material_center = Lambertian {
        albedo: Color::color(0.1, 0.2, 0.5),
    };
    let material_left = Metal {
        albedo: Color::color(0.8, 0.8, 0.8),
    };
    let material_right = Metal {
        albedo: Color::color(0.8, 0.6, 0.2),
    };

    let sphere1 = Sphere {
        center: Point3::point(0., -100.5, -1.),
        radius: 100.0,
        material: Some(&material_ground),
    };
    let sphere2 = Sphere {
        center: Point3::point(0., 0.0, -1.2),
        radius: 0.5,
        material: Some(&material_center),
    };
    let sphere3 = Sphere {
        center: Point3::point(-1., 0.0, -1.),
        radius: 0.5,
        material: Some(&material_left),
    };
    let sphere4 = Sphere {
        center: Point3::point(1., 0., -1.),
        radius: 0.5,
        material: Some(&material_right),
    };

    // World
    let mut world = HittableList::default();
    world.add(&sphere1);
    world.add(&sphere2);
    world.add(&sphere3);
    world.add(&sphere4);

    let camera = Camera::initialize(16. / 9., 400, 100, 50);

    camera.render(&world);
}
