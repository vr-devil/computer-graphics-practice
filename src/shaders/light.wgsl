struct Ray {
    origin: vec3f,
    direction: vec3f,
}

struct Sphere {
    center: vec3f,
    radius: f32,
    color: vec3f,
    specular: f32,
}

struct Light {
    light_type: u32, // 1-ambient|2-point|3-directional
    intensity: f32,
    position: vec3f,
    direction: vec3f,
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

const spheres = array<Sphere, 4>(
    Sphere(vec3f(0.0, -1.0, 3.0), 1.0, vec3f(1, 0, 0), 500), // red
    Sphere(vec3f(2.0, 0.0, 4.0), 1.0, vec3f(0, 0, 1), 500), // green
    Sphere(vec3f(-2.0, 0.0, 4.0), 1.0, vec3f(0, 1, 0), 10), // blue
    Sphere(vec3f(0.0, -5001.0, 0.0), 5000.0, vec3f(1, 1, 0), 1000), // yellow
);

const lights = array<Light, 3>(
    Light(0, 0.2, vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.0, 0.0)), // ambient light
    Light(1, 0.6, vec3f(2.0, 1.0, 0.0), vec3f(0.0, 0.0, 0.0)), // point light
    Light(2, 0.2, vec3f(0.0, 0.0, 0.0), vec3f(1.0, 4.0, 4.0)), // directional light
);

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

    return tracy_ray(ray, spheres);
}

// return sphere color
fn tracy_ray(ray: Ray, spheres: array<Sphere, 4>) -> vec4f {
    var closet_t: f32 = 3.40282346638528859812e+38f;
    var close_sphere_i = -1;

    for (var i: i32 = 0; i < 4; i++) {

        let result = intersect_ray_sphere(ray, spheres[i]);
        let t1 = result[0];
        let t2 = result[1];

        if t1 > 0.0 && t1 < closet_t {
            closet_t = t1;
            close_sphere_i = i;
        }

        if t2 > 0.0 && t2 < closet_t {
            closet_t = t2;
            close_sphere_i = i;
        }
    }

    if(close_sphere_i == -1) {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }

    let sphere = spheres[close_sphere_i];

    let P = ray.origin + closet_t * ray.direction;
    var N = P - sphere.center;
    N = N / length(N);

    let color = sphere.color * compute_lighting(P, N, -ray.direction, sphere.specular);
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

fn compute_lighting(P: vec3f, N: vec3f, V: vec3f, s: f32) -> f32 {
    var i: f32 = 0.0;

    for (var j: u32; j < 3; j++) {
        let light = lights[j];

        if(light.light_type == 0) { // ambient light
            i += light.intensity;
        } else {
            var L: vec3f;
            if (light.light_type == 1) { // point light
                L = light.position - P;
            } else {
                L = light.direction;
            }

            // diffuse
            let n_dot_l = dot(N, L);
            if (n_dot_l > 0) {
                i += light.intensity * n_dot_l / (length(N) * length(L));
            }

            // specular
            if (s != -1.0) {
                let R = 2 * N * dot(N, L) - L;
                let r_dot_v = dot(R, V);
                if (r_dot_v > 0) {
                    i += light.intensity * pow(r_dot_v / (length(R) * length(V)), s);
                }
            }
        }
    }


    return i;
}
