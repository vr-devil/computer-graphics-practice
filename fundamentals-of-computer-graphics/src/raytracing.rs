use crate::math::Vector3;

struct Ray {
    pub e: Vector3,
    pub d: Vector3,
}

#[test]
fn test_intersect_sphere() {
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

    let t1 = (-b-discriminant.sqrt()) / (2. * a);
    let t2 = (-b+discriminant.sqrt()) / (2. * a);

    println!("t1:{}, t2: {}", t1, t2);

    let p1 = &e+&(t1 * &d);
    let p2 = &e+&(t2 * &d);
    println!("p1:{:?} p2:{:?}", p1, p2);

    let p1 = p1.norm();
    let p2 = p2.norm();
    println!("p1 norm:{:?} p2 norm:{:?}", p1, p2);

    assert_eq!(p1.round(), 1., "p1");
    assert_eq!(p2.round(), 1., "p2");
}
