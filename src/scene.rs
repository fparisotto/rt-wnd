use crate::hittable::HittableList;
use crate::material::Materials;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use rand::prelude::*;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Materials::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.add_sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let rng_x: f64 = random();
            let rng_z: f64 = random();
            let center: Vec3 = Vec3::new(a as f64 + 0.9 * rng_x, 0.2, b as f64 + 0.9 * rng_z);

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // difuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Materials::Lambertian { albedo };
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Materials::Metal { albedo, fuzz };
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Materials::Dielectric { ir: 1.5 };
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Materials::Dielectric { ir: 1.5 };
    world.add_sphere(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Materials::Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.add_sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Materials::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add_sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
