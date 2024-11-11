use crate::utils::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    point3::Point3,
    sphere::Sphere,
    vec3::Vec3,
};

pub fn run() {
    // Materials
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Dielectric {
        refraction_index: 1.5,
    };
    let material_bubble = Dielectric {
        refraction_index: 1. / 1.5,
    };
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.);

    let sphere1 = Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.0,
        material: Some(&material_ground),
    };
    let sphere2 = Sphere {
        center: Point3::new(0., 0.0, -1.2),
        radius: 0.5,
        material: Some(&material_center),
    };
    let sphere3 = Sphere {
        center: Point3::new(-1., 0.0, -1.),
        radius: 0.5,
        material: Some(&material_left),
    };
    let sphere4 = Sphere {
        center: Point3::new(1., 0., -1.),
        radius: 0.5,
        material: Some(&material_right),
    };
    let sphere5 = Sphere {
        center: Point3::new(-1., 0., -1.),
        radius: 0.4,
        material: Some(&material_bubble),
    };

    // World
    let mut world = HittableList::default();
    world.add(&sphere1);
    world.add(&sphere2);
    world.add(&sphere3);
    world.add(&sphere4);
    world.add(&sphere5);

    let camera = Camera::initialize(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
    );

    camera.render(&world);
}
