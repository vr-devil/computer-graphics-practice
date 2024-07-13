struct Ray {
    origin: vec3f,
    direction: vec3f,
}

struct Sphere {
    center: vec3f,
    radius: f32,
    color: vec3f,
    specular: f32,
    reflective: f32,
}

struct Light {
    light_type: u32, // 1-ambient|2-point|3-directional
    intensity: f32,
    position: vec3f,
    direction: vec3f,
}

struct Intersect {
    closet_t: f32,
    closet_i: i32,
}

struct Range {
    min: f32,
    max: f32,
}

const inf = 3.40282346638528859812e+38f;

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
    Sphere(vec3f(0.0, -1.0, 3.0), 1.0, vec3f(1, 0, 0), 500, 0.2), // red
    Sphere(vec3f(2.0, 0.0, 4.0), 1.0, vec3f(0, 0, 1), 500, 0.3), // green
    Sphere(vec3f(-2.0, 0.0, 4.0), 1.0, vec3f(0, 1, 0), 10, 0.4), // blue
    Sphere(vec3f(0.0, -5001.0, 0.0), 5000.0, vec3f(1, 1, 0), 1000, 0.5), // yellow
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

    return tracy_ray(ray, Range(1.0, inf), 3);
}

// return sphere color
fn tracy_ray(ray: Ray, range: Range, max_recursion_depth: u32) -> vec4f {
    var accumulated_color = vec3f(0.0, 0.0, 0.0);
    var current_ray = ray;
    var current_range = range;
    var depth = max_recursion_depth;
    var reflectance = 1.0;

    loop {
        let intersect = closet_intersection(current_ray, current_range);
        if(intersect.closet_t == -1) {
            break;
        }

        let sphere = spheres[intersect.closet_i];
        let P = current_ray.origin + intersect.closet_t * current_ray.direction;
        var N = P - sphere.center;
        N = normalize(N);

        let local_color = sphere.color * compute_lighting(P, N, -current_ray.direction, sphere.specular);
        accumulated_color += local_color * reflectance;

        let r = sphere.reflective;
        if(depth == 0 || r <= 0) {
            break;
        }

        reflectance *= r;
        let R = reflect_ray(-current_ray.direction, N);
        current_ray = Ray(P, R);
        current_range = Range(0.1, inf);
        depth--;
    }

    return vec4f(accumulated_color, 1.0);
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
            var t_max: f32;
            if (light.light_type == 1) { // point light
                L = light.position - P;
                t_max = 1;
            } else {
                L = light.direction;
                t_max = inf;
            }

            let shadow_intersect = closet_intersection(Ray(P, L), Range(0.001, t_max));
            if(shadow_intersect.closet_i != -1) {
                continue;
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

fn closet_intersection(ray: Ray, range: Range) -> Intersect {
    var closet_t: f32 = inf;
    var close_sphere_i = -1;

    for (var i: i32 = 0; i < 4; i++) {

        let result = intersect_ray_sphere(ray, spheres[i]);
        let t1 = result[0];
        let t2 = result[1];

        if range.min <= t1 && t1 <= range.max && t1 < closet_t {
            closet_t = t1;
            close_sphere_i = i;
        }

        if range.min <= t2 && t2 <= range.max && t2 < closet_t {
            closet_t = t2;
            close_sphere_i = i;
        }
    }

    return Intersect(closet_t, close_sphere_i);
}

fn reflect_ray(R: vec3f, N: vec3f) -> vec3f {
    return 2 * N * dot(N, R) - R;
}