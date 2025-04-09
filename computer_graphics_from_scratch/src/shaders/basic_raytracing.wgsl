struct Ray {
    origin: vec3f,
    direction: vec3f,
}

struct Sphere {
    center: vec3f,
    radius: f32,
    color: vec3f,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    return vec4<f32>(pos[vertex_index], 0.0, 1.0);
}

const cw: f32 = 500.0;
const ch: f32 = 500.0;

@fragment
fn fs_main(@builtin(position) frag_coord: vec4f) -> @location(0) vec4f {

    // canvas coord
    let cx: f32 = frag_coord.x - cw / 2.0;
    let cy: f32 = ch / 2.0 - frag_coord.y; // reverse y axis.

    // viewport coord
    let vx = cx * (1.0 / cw);
    let vy = cy * (1.0 / ch);

    let ray = Ray(
        vec3f(0.0, 0.0, 0.0),
        vec3f(vx, vy, 1.0)
    );

    let spheres = array<Sphere, 3>(
        Sphere(vec3f(0.0, -1.0, 3.0), 1.0, vec3f(1, 0, 0)), // red
        Sphere(vec3f(2.0, 0.0, 4.0), 1.0, vec3f(0, 0, 1)), // green
        Sphere(vec3f(-2.0, 0.0, 4.0), 1.0, vec3f(0, 1, 0)), // blue
    );

    return tracy_ray(ray, spheres);
}

// return sphere color
fn tracy_ray(ray: Ray, spheres: array<Sphere, 3>) -> vec4f {
    var closet_t: f32 = 3.40282346638528859812e+38f;
    var color = vec3f(0.0, 0.0, 0.0);

    for (var i: i32 = 0; i < 3; i++) {

        let result = intersect_ray_sphere(ray, spheres[i]);
        let t1 = result[0];
        let t2 = result[1];

        if t1 > 0.0 && t1 < closet_t {
            closet_t = t1;
            color = spheres[i].color;
        }

        if t2 > 0.0 && t2 < closet_t {
            closet_t = t2;
            color = spheres[i].color;
        }
    }

    return vec4<f32>(color, 1.0);
}

fn intersect_ray_sphere(ray: Ray, sphere: Sphere) -> array<f32, 2> {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return array<f32, 2>(-1.0, -1.0);
    } else {
        let t1 = (-b + sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b - sqrt(discriminant)) / (2.0 * a);

        return array<f32, 2>(t1, t2);
    }
}

