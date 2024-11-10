use crate::utils::{camera::Camera, hittable::HittableList, point3::Point3, sphere::Sphere};

pub fn run() {
    // World
    let mut world = HittableList::default();
    world.add(&Sphere {
        center: Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
    });
    world.add(&Sphere {
        center: Point3 {
            x: 0.,
            y: -100.5,
            z: -1.,
        },
        radius: 100.,
    });

    let camera = Camera::initialize(16. / 9., 400, 10);

    camera.render(&world);
}
