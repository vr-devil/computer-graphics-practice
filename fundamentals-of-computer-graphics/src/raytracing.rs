use crate::math::{Vector3};

struct Ray {
    pub e: Vector3,
    pub d: Vector3,
}

impl Ray {
    pub fn point(&self, t: f32) -> Vector3 {
        &self.e + &(t * &self.d)
    }
}

#[test]
fn test_ray_sphere_intersection() {
    // sphere
    let r = 1.0f32;
    let o = Vector3::new(0.0, 0.0, 0.0);

    // ray
    let e = Vector3::new(1., 1., 1.);
    let d = Vector3::new(-1., -1., -1.);

    let j = &e - &o;
    let a = d.dot(&d);
    let b = 2. * d.dot(&j);
    let c = j.dot(&j) - r * r;
    let discriminant = b * b - 4. * a * c;

    println!("a:{}, b:{}, c:{}, discriminant:{}", a, b, c, discriminant);
    assert!(discriminant > 0.);

    let t1 = (-b - discriminant.sqrt()) / (2. * a);
    let t2 = (-b + discriminant.sqrt()) / (2. * a);

    println!("t1:{}, t2: {}", t1, t2);

    let p1 = &e + &(t1 * &d);
    let p2 = &e + &(t2 * &d);
    println!("p1:{:?} p2:{:?}", p1, p2);

    let p1 = p1.norm();
    let p2 = p2.norm();
    println!("p1 norm:{:?} p2 norm:{:?}", p1, p2);

    assert_eq!(p1.round(), 1., "p1");
    assert_eq!(p2.round(), 1., "p2");
}

#[test]
fn test_ray_triangle_intersection() {

    use crate::math::vec3;

    // ray
    let ray = Ray {
        e: vec3(1., 1., 1.),
        d: vec3(-1., -1., -1.),
    };

    // triangle
    let a = vec3(1., 0., 0.);
    let b = vec3(0., 1., 0.);
    let c = vec3(0., 0., 1.);

    {
        let ((a, b, c), (d, e, f), (g, h, i), (j, k, l)) = (
            (a.x - b.x, a.y - b.y, a.z - b.z),
            (a.x - c.x, a.y - c.y, a.z - c.z),
            (ray.d.x, ray.d.y, ray.d.z),
            (a.x - ray.e.x, a.y - ray.e.y, a.z - ray.e.z),
        );

        let m = a * (e * i - h * f) + b * (g * f - d * i) + c * (d * h - e * g);
        let beta = (j * (e * i - h * f) + k * (g * f - d * i) + l * (d * h - e * g)) / m;
        let gamma = (i * (a * k - j * b) + h * (j * c - a * l) + g * (b * l - k * c)) / m;
        let alpha = 1. - beta - gamma;
        let t = -(f * (a * k - j * b) + e * (j * c - a * l) + d * (b * l - k * c)) / m;
        let p = ray.point(t);

        println!("alpha:{alpha}, beta:{beta}, gamma:{gamma}, t:{t}");
        println!("p:{:?}", p);

        let (t0, t1) = (0., 1.);

        let hit = if t < t0 || t > t1 {
            false
        } else if gamma < 0. || gamma > 1. {
            false
        } else if beta < 0. || beta > 1. - gamma {
            false
        } else {
            true
        };

        assert_eq!(hit, true);
    }
}
