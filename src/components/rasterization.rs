use crate::components::graphics::Light;
use nalgebra_glm::{identity, length, rotate_y, scaling, translation, triangle_normal, vec2, vec3, vec4, Mat4x4, Vec2, Vec3, Vec4};
use rgb::Rgb;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use log::info;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, Performance};
use yew::{html, Component, Context, Html, NodeRef};

pub struct RasterizationCanvas {
    canvas_ref: NodeRef,
}

impl Component for RasterizationCanvas {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas class="bg-black" ref={self.canvas_ref.clone()} width="300" height="300"/>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas: HtmlCanvasElement = self.canvas_ref.cast().unwrap();
            ctx.link().send_future(async move {
                let rasterizer = Rasterizer {
                    canvas,
                    performance: window().unwrap().performance().unwrap(),
                    texture: Texture::new("crate-texture.jpg"),
                };

                let a = Closure::<dyn Fn()>::new(move || {
                    rasterizer.render()
                });

                window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)
                    .expect("TODO: panic message");
                a.forget();
            })
        }
    }
}

#[wasm_bindgen]
struct Rasterizer {
    canvas: HtmlCanvasElement,
    performance: Performance,
    texture: Texture,
}

impl Rasterizer {
    fn render(&self) {
        let start = self.performance.now();

        let context = self.canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let w = self.canvas.width();
        let h = self.canvas.height();

        // info!("w{:}/h{:}", w, h);


        let s2 = 1.0 / 2.0f32.sqrt();
        let camera = Camera {
            position: Vec3::new(-3.0, 1.0, 2.0),
            rotation: -30.0,
            planes: vec![
                Plane { normal: vec3(0.0, 0.0, 1.0), distance: -1.0 }, // Near,
                Plane { normal: vec3(s2, 0.0, s2), distance: 0.0 }, // Left,
                Plane { normal: vec3(-s2, 0.0, s2), distance: 0.0 }, // Right,
                Plane { normal: vec3(0.0, -s2, s2), distance: 0.0 }, // Top,
                Plane { normal: vec3(0.0, s2, s2), distance: 0.0 }, // Bottom,
            ],
        };

        let lights = vec![
            Light::Ambient { intensity: 0.2 },
            Light::Directional { intensity: 0.2, direction: vec3(-1.0, 0.0, 1.0) },
            Light::Point { intensity: 0.6, position: vec3(-3.0, 2.0, -10.0) },
        ];

        let mut renderer = Renderer {
            camera,
            lights,
            data: context
                .get_image_data(0.0, 0.0, w as f64, h as f64)
                .unwrap()
                .data()
                .0,
            depth_buf: vec![0.0f32; (w * h) as usize],
            canvas_size: Rectangle { w: w as i32, h: h as i32 },
            viewport_size: Rectangle { w: 1, h: 1 },
        };

        let color = Rgb::new(255.0, 255.0, 255.0);
        renderer.draw_line(Vec2::new(-150.0, -150.0), Vec2::new(150.0, 150.0), color);
        renderer.draw_line(Vec2::new(-150.0, 150.0), Vec2::new(150.0, -150.0), color);
        renderer.draw_line(Vec2::new(-150.0, 0.0), Vec2::new(150.0, -0.0), color);
        renderer.draw_line(Vec2::new(0.0, 150.0), Vec2::new(0.0, -150.0), color);


        let red: Rgb<f32> = Rgb::new(255.0, 0.0, 0.0);
        let green: Rgb<f32> = Rgb::new(0.0, 255.0, 0.0);
        let blue: Rgb<f32> = Rgb::new(0.0, 0.0, 255.0);
        let yellow: Rgb<f32> = Rgb::new(255.0, 255.0, 0.0);
        let purple: Rgb<f32> = Rgb::new(255.0, 0.0, 255.0);
        let cyan: Rgb<f32> = Rgb::new(0.0, 255.0, 255.0);

        let cube = Model {
            texture: Some(self.texture.clone()),
            vertices: vec![
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
            ],
            triangles: vec![
                Triangle::new([0, 1, 2], red.clone(), [vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0)], [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]),
                Triangle::new([0, 2, 3], red.clone(), [vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0)], [vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]),
                Triangle::new([4, 0, 3], green.clone(), [vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]),
                Triangle::new([4, 3, 7], green.clone(), [vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]),
                Triangle::new([5, 4, 7], blue.clone(), [vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, -1.0)], [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]),
                Triangle::new([5, 7, 6], blue.clone(), [vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, -1.0)], [vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]),
                Triangle::new([1, 5, 6], yellow.clone(), [vec3(-1.0, 0.0, 0.0), vec3(-1.0, 0.0, 0.0), vec3(-1.0, 0.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]),
                Triangle::new([1, 6, 2], yellow.clone(), [vec3(-1.0, 0.0, 0.0), vec3(-1.0, 0.0, 0.0), vec3(-1.0, 0.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]),
                Triangle::new([1, 0, 5], purple.clone(), [vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]),
                Triangle::new([5, 0, 4], purple.clone(), [vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0)], [vec2(0.0, 1.0), vec2(1.0, 1.0), vec2(0.0, 0.0)]),
                Triangle::new([2, 6, 7], cyan.clone(), [vec3(0.0, -1.0, 0.0), vec3(0.0, -1.0, 0.0), vec3(0.0, -1.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]),
                Triangle::new([2, 7, 3], cyan.clone(), [vec3(0.0, -1.0, 0.0), vec3(0.0, -1.0, 0.0), vec3(0.0, -1.0, 0.0)], [vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)]),
            ],
            bounds_center: Default::default(),
            bounds_radius: 3.0f32.sqrt(),
        };

        let sphere = Model::generate_sphere(15.0, green.clone());


        let instances = vec![
            Instance { model: cube.clone(), position: vec3(-1.5, 0.0, 7.0), rotation: 0.0, scale: 0.75 },
            Instance { model: cube.clone(), position: vec3(1.25, 2.5, 7.5), rotation: 195.0, scale: 1.0 },
            Instance { model: sphere.clone(), position: vec3(1.75, -0.5, 7.0), rotation: 0.0, scale: 1.5 },
        ];

        renderer.render_scene(instances);

        let data = ImageData::new_with_u8_clamped_array(Clamped(renderer.data.as_slice()), w).unwrap();
        context.put_image_data(&data, 0.0, 0.0).expect("TODO: panic message");


        let end = self.performance.now();
        info!("execution: {:?}", end - start);
    }
}

struct Point {
    coord: Vec2,
    h: f32, //color intensity
}

struct Cube {
    // front vertexes
    pub af: Vec3,
    pub bf: Vec3,
    pub cf: Vec3,
    pub df: Vec3,

    // back vertexes
    pub ab: Vec3,
    pub bb: Vec3,
    pub cb: Vec3,
    pub db: Vec3,
}

struct Rectangle {
    pub w: i32,
    pub h: i32,
}

#[derive(Clone)]
struct Triangle {
    pub color: Rgb<f32>,
    pub indexes: [usize; 3],
    pub intensities: [f32; 3],
    pub normals: [Vec3; 3],
    pub uvs: [Vec2; 3],
}

impl Triangle {
    fn new(indexes: [usize; 3], color: Rgb<f32>, normals: [Vec3; 3], uvs: [Vec2; 3]) -> Self {
        Self { indexes, color, intensities: [1.0; 3], normals, uvs }
    }

    fn new_with_intensity(indexes: [usize; 3], color: Rgb<f32>, intensities: [f32; 3], normals: [Vec3; 3], uvs: [Vec2; 3]) {
        Self { indexes, intensities, normals, color, uvs };
    }

    fn points(&self, projected: &Vec<Vec2>) -> (Point, Point, Point) {
        let [a, b, c] = self.indexes.clone();
        let [h1, h2, h3] = self.intensities.clone();

        (
            Point { coord: projected[a].clone(), h: h1 },
            Point { coord: projected[b].clone(), h: h2 },
            Point { coord: projected[c].clone(), h: h3 },
        )
    }
}

struct Camera {
    pub position: Vec3,
    pub rotation: f32,
    pub planes: Vec<Plane>,
}

impl Camera {
    fn get_orientation(&self) -> Mat4x4 {
        rotate_y(&identity(), self.rotation.to_radians()).transpose()
    }

    fn get_matrix(&self) -> Mat4x4 {
        self.get_orientation().transpose() * translation(&-self.position)
    }
}

struct Plane {
    normal: Vec3,
    distance: f32,
}


#[derive(Clone)]
struct Model {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub bounds_center: Vec3,
    pub bounds_radius: f32,
    pub texture: Option<Texture>,
}

impl Model {
    fn generate_sphere(divs: f32, color: Rgb<f32>) -> Model {
        let mut vertices = vec![];
        let mut triangles = vec![];

        let delta_angle = 2.0 * PI / divs;

        for d in 0..divs as i32 + 1 {
            let y = (2.0 / divs) * (d as f32 - divs / 2.0);
            let radius = (1.0 - y * y).sqrt();
            for i in 0..divs as i32 {
                vertices.push(vec3(
                    radius * (i as f32 * delta_angle).cos(),
                    y,
                    radius * (i as f32 * delta_angle).sin(),
                ))
            }
        }

        for d in 0..divs as i32 {
            for i in 0..divs as i32 {
                let i0 = (d as f32 * divs + i as f32) as usize;
                let i1 = ((d + 1) as f32 * divs + (i + 1) as f32 % divs) as usize;
                let i2 = (divs * d as f32 + (i + 1) as f32 % divs) as usize;

                let tri0 = [i0, i1, i2];
                let tri1 = [i0, i0 + divs as usize, i1];
                let uvs = [vec2(0.0, 0.0), vec2(0.0, 0.0), vec2(0.0, 0.0)];
                triangles.push(Triangle::new(tri0, color.clone(), [vertices[tri0[0]], vertices[tri0[1]], vertices[tri0[2]]], uvs.clone()));
                triangles.push(Triangle::new(tri1, color.clone(), [vertices[tri1[0]], vertices[tri1[1]], vertices[tri1[2]]], uvs.clone()));
            }
        }

        Model {
            texture: None,
            vertices,
            triangles,
            bounds_center: vec3(0.0, 0.0, 0.0),
            bounds_radius: 1.0,
        }
    }
}


#[derive(Clone)]
struct Instance {
    pub model: Model,

    pub scale: f32,
    pub rotation: f32,
    pub position: Vec3,
}

impl Instance {
    fn transform(&self) -> Mat4x4 {
        let s = scaling(&vec3(self.scale, self.scale, self.scale));
        let r = self.get_orientation();
        let t = translation(&self.position);

        t * r * s
    }

    fn get_orientation(&self) -> Mat4x4 {
        rotate_y(&identity(), self.rotation.to_radians()).transpose()
    }
}

#[derive(Clone)]
struct Texture {
    texels: Rc<RefCell<Option<(Vec<u8>, f32, f32)>>>,
}

impl Texture {
    pub fn new(src: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(src);

        let canvas = window().unwrap()
            .document().unwrap()
            .create_element("canvas").unwrap()
            .dyn_into::<HtmlCanvasElement>().unwrap();

        let texture = Self { texels: Rc::new(RefCell::new(None)) };
        let texels = Rc::clone(&texture.texels);

        let image_ref = image.clone();
        let handler = Closure::<dyn FnMut()>::new(move || {
            let (w, h) = (image_ref.width(), image_ref.height());

            canvas.set_width(w);
            canvas.set_height(h);
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context.draw_image_with_html_image_element_and_dw_and_dh(&image_ref, 0.0, 0.0, w as f64, h as f64).expect("TODO: panic message");

            let image_data = context.get_image_data(0.0, 0.0, w as f64, h as f64)
                .unwrap()
                .data()
                .0;

            texels.replace(Some((image_data, w as f32, h as f32)));
        });
        image.set_onload(Some(handler.as_ref().unchecked_ref()));

        handler.forget();

        texture
    }

    pub fn get_texel(&self, u: f32, v: f32) -> Rgb<f32> {
        let texels = self.texels.borrow();
        match texels.as_ref() {
            None => {
                Rgb::new(255.0, 255.0, 255.0)
            }
            Some((data, w, h)) => {
                let iu = (u * w).floor() - 1.0;
                let iv = (v * h).floor() - 1.0;

                let offset = (4.0 * (iu + iv * w)) as usize;
                // info!("{:.5}:{:.5} - {}:{} - {}", u, v, iu, iv, offset);

                Rgb::new(
                    data[offset + 0] as f32,
                    data[offset + 1] as f32,
                    data[offset + 2] as f32,
                )
            }
        }
    }
}

struct Renderer {
    camera: Camera,
    lights: Vec<Light>,
    data: Vec<u8>,
    depth_buf: Vec<f32>,
    canvas_size: Rectangle,
    viewport_size: Rectangle,
}

impl Renderer {
    fn render_scene(&mut self, instances: Vec<Instance>) {
        let camera_matrix = self.camera.get_matrix();
        for instance in instances {
            let transform = camera_matrix * instance.transform();
            if let Some(clipped_instance) = self.transform_and_clip(instance, transform) {
                self.render_model(&clipped_instance)
            }
        }

        // info!("{:?}", self.depth_buf)
    }

    fn render_model(&mut self, instance: &Instance) {
        let mut projected = vec![];

        for v in instance.model.vertices.iter() {
            projected.push(self.project_vertex(v));
        }

        for t in instance.model.triangles.iter() {
            self.render_triangle(instance, t, &projected);
        }
    }

    fn render_triangle(&mut self, instance: &Instance, triangle: &Triangle, projected: &Vec<Vec2>) {
        // self.draw_shaded_triangle(triangle, projected);
        self.draw_filled_triangle(instance, triangle, projected);
        // self.draw_wireframe_triangle(triangle, projected);
    }
}

impl Renderer {
    fn draw_line(&mut self, mut p0: Vec2, mut p1: Vec2, color: Rgb<f32>) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            if p0.x > p1.x {
                self.swap(&mut p0, &mut p1)
            }

            let x0 = p0.x as i32;
            let x1 = p1.x as i32;
            let ys = self.interpolate(p0.x, p0.y, p1.x, p1.y);

            for x in x0..=x1 {
                let y = (x - x0) as usize;
                self.put_pixels(x, ys[y] as i32, color);
            }
        } else {
            if p0.y > p1.y {
                self.swap(&mut p0, &mut p1)
            }

            let y0 = p0.y as i32;
            let y1 = p1.y as i32;
            let xs = self.interpolate(p0.y, p0.x, p1.y, p1.x);
            for y in y0..=y1 {
                let x = (y - y0) as usize;
                self.put_pixels(xs[x] as i32, y, color);
            }
        }
    }

    fn draw_wireframe_triangle(&mut self, triangle: &Triangle, projected: &Vec<Vec2>) {
        let (p0, p1, p2) = triangle.points(projected);
        let color = triangle.color.clone();

        self.draw_line(p0.coord.clone(), p1.coord.clone(), color);
        self.draw_line(p0.coord.clone(), p2.coord.clone(), color);
        self.draw_line(p2.coord.clone(), p1.coord.clone(), color);
    }

    fn draw_filled_triangle(&mut self, instance: &Instance, triangle: &Triangle, projected: &Vec<Vec2>) {
        let vertices = &instance.model.vertices;
        let indexes = triangle.indexes;
        let [a, b, c] = indexes;

        let normal = triangle_normal(&vertices[a], &vertices[b], &vertices[c]);
        let vertex_to_camera = -vertices[a];
        if vertex_to_camera.dot(&normal) <= 0.0 {
            return;
        }

        let [i0, i1, i2] = self.sort_vertex_indexes(triangle, projected);
        let (a, b, c) = (indexes[i0], indexes[i1], indexes[i2]);


        let v0 = &vertices[a];
        let v1 = &vertices[b];
        let v2 = &vertices[c];

        let p0 = &projected[a];
        let p1 = &projected[b];
        let p2 = &projected[c];

        let (x02, x012) = self.edge_interpolate(p0.y, p0.x, p1.y, p1.x, p2.y, p2.x);
        let (z02, z012) = self.edge_interpolate(p0.y, 1.0 / v0.z, p1.y, 1.0 / v1.z, p2.y, 1.0 / v2.z);

        let transform = self.camera.get_orientation().transpose() * instance.get_orientation();

        let (n0, n1, n2) = (triangle.normals[i0], triangle.normals[i1], triangle.normals[i2]);
        let n0 = transform * vec4(n0.x, n0.y, n0.z, 1.0);
        let n1 = transform * vec4(n1.x, n1.y, n1.z, 1.0);
        let n2 = transform * vec4(n2.x, n2.y, n2.z, 1.0);

        let (nx02, nx012) = self.edge_interpolate(p0.y, n0.x, p1.y, n1.x, p2.y, n2.x);
        let (ny02, ny012) = self.edge_interpolate(p0.y, n0.y, p1.y, n1.y, p2.y, n2.y);
        let (nz02, nz012) = self.edge_interpolate(p0.y, n0.z, p1.y, n1.z, p2.y, n2.z);

        let uvs = triangle.uvs.clone();
        let (uz02, uz012) = self.edge_interpolate(p0.y, uvs[i0].x / v0.z, p1.y, uvs[i1].x / v1.z, p2.y, uvs[i2].x / v2.z);
        let (vz02, vz012) = self.edge_interpolate(p0.y, uvs[i0].y / v0.z, p1.y, uvs[i1].y / v1.z, p2.y, uvs[i2].y / v2.z);

        let m = x02.len() / 2;
        let (
            (x_left, x_right),
            (z_left, z_right),
            (nx_left, nx_right),
            (ny_left, ny_right),
            (nz_left, nz_right),
            (uz_left, uz_right),
            (vz_left, vz_right),
        ) = if x02[m] < x012[m] {
            (
                (x02, x012), (z02, z012),
                (nx02, nx012), (ny02, ny012), (nz02, nz012),
                (uz02, uz012), (vz02, vz012)
            )
        } else {
            (
                (x012, x02), (z012, z02),
                (nx012, nx02), (ny012, ny02), (nz012, nz02),
                (uz012, uz02), (vz012, vz02)
            )
        };

        let y0 = p0.y as i32;
        let y2 = p2.y as i32;

        for y in y0..=y2 {
            let yi = (y - y0) as usize;

            let (xl, xr) = (x_left[yi], x_right[yi]);
            let (zl, zr) = (z_left[yi], z_right[yi]);

            let (nxl, nxr) = (nx_left[yi], nx_right[yi]);
            let (nyl, nyr) = (ny_left[yi], ny_right[yi]);
            let (nzl, nzr) = (nz_left[yi], nz_right[yi]);

            let zscan = self.interpolate(xl, zl, xr, zr);
            let nxscan = self.interpolate(xl, nxl, xr, nxr);
            let nyscan = self.interpolate(xl, nyl, xr, nyr);
            let nzscan = self.interpolate(xl, nzl, xr, nzr);

            let uzscan = self.interpolate(xl, uz_left[yi], xr, uz_right[yi]);
            let vzscan = self.interpolate(xl, vz_left[yi], xr, vz_right[yi]);


            for x in xl as i32..=xr as i32 {
                let xi = (x - xl as i32) as usize;
                let z = zscan[xi];
                if self.update_depth_buf_if_closer(x, y, z) {
                    let vertex = self.un_project_vertex(x as f32, y as f32, z);
                    let normal = vec3(nxscan[xi], nyscan[xi], nzscan[xi]);
                    let intensity = self.compute_illumination(&vertex, &normal);

                    let color = if let Some(texture) = instance.model.texture.as_ref() {
                        let u = uzscan[xi] / zscan[xi];
                        let v = vzscan[xi] / zscan[xi];

                        texture.get_texel(u, v)
                    } else {
                        triangle.color.clone()
                    };

                    // info!("{},{} - {:?}", x, y, color);

                    self.put_pixels(x, y, color * intensity);
                }
            }
        }
    }

    fn draw_shaded_triangle(&mut self, triangle: &Triangle, projected: &Vec<Vec2>) {
        let (mut p0, mut p1, mut p2) = triangle.points(projected);
        let color = triangle.color.clone();

        if p1.coord.y < p0.coord.y {
            self.swap(&mut p1.coord, &mut p0.coord)
        }

        if p2.coord.y < p0.coord.y {
            self.swap(&mut p2.coord, &mut p0.coord)
        }

        if p2.coord.y < p1.coord.y {
            self.swap(&mut p2.coord, &mut p1.coord)
        }

        let mut x01 = self.interpolate(p0.coord.y, p0.coord.x, p1.coord.y, p1.coord.x);
        let mut h01 = self.interpolate(p0.coord.y, p0.h, p1.coord.y, p1.h);

        let x12 = self.interpolate(p1.coord.y, p1.coord.x, p2.coord.y, p2.coord.x);
        let h12 = self.interpolate(p1.coord.y, p1.h, p2.coord.y, p2.h);

        let x02 = self.interpolate(p0.coord.y, p0.coord.x, p2.coord.y, p2.coord.x);
        let h02 = self.interpolate(p0.coord.y, p0.h, p2.coord.y, p2.h);

        x01.remove(x01.len() - 1);
        let x012 = [x01, x12].concat();

        h01.remove(h01.len() - 1);
        let h012 = [h01, h12].concat();

        let x_left;
        let h_left;
        let x_right;
        let h_right;

        let m = x012.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            h_left = h02;

            x_right = x012;
            h_right = h012;
        } else {
            x_left = x012;
            h_left = h012;

            x_right = x02;
            h_right = h02;
        }

        for y in p0.coord.y as i32..=p2.coord.y as i32 {
            let i = (y - p0.coord.y as i32) as usize;
            let x_l = x_left[i];
            let x_r = x_right[i];

            let h_segment = self.interpolate(x_l, h_left[i], x_r, h_right[i]);
            for x in x_l as i32..=x_r as i32 {
                let shaded_color = color * h_segment[(x - x_l as i32) as usize];
                self.put_pixels(x, y, shaded_color);
            }
        }
    }
}

impl Renderer {
    fn transform_and_clip(&self, instance: Instance, transform: Mat4x4) -> Option<Instance> {
        let bc = &instance.model.bounds_center;
        let center = transform * vec4(bc.x, bc.y, bc.z, 1.0);
        let radius = instance.model.bounds_radius * instance.scale;

        for plane in self.camera.planes.iter() {
            let distance = self.signed_distance(plane, &center.xyz());
            if distance < -radius {
                return None;
            }
        }

        /*
         * transform
         */
        let mut vertices: Vec<Vec3> = vec![];
        for v in instance.model.vertices.iter() {
            let h = Vec4::new(v.x, v.y, v.z, 1.0);
            vertices.push((transform * h).xyz())
        }

        let mut triangles = vec![];
        for plane in self.camera.planes.iter() {
            let mut new_triangles = vec![];
            for triangle in instance.model.triangles.iter() {
                let clipped_triangles = self.clip_triangle(triangle, &vertices, plane);
                new_triangles.extend(clipped_triangles)
            }
            triangles = new_triangles
        }

        Some(Instance {
            model: Model {
                texture: instance.model.texture,
                vertices,
                triangles,
                bounds_center: center.xyz(),
                bounds_radius: radius,
            },
            scale: 1.0,
            rotation: 0.0,
            position: Default::default(),
        })
    }

    fn clip_triangle(&self, triangle: &Triangle, vertices: &Vec<Vec3>, plane: &Plane) -> Vec<Triangle> {
        let indexes = triangle.indexes.clone();
        let [a, b, c] = indexes;
        let a = &vertices[a];
        let b = &vertices[b];
        let c = &vertices[c];

        let d0 = self.signed_distance(plane, a);
        let d1 = self.signed_distance(plane, b);
        let d2 = self.signed_distance(plane, c);

        let in_count = [d0, d1, d2].iter().filter(|&&x| x > 0.0).count();

        if in_count == 3 {
            vec![(*triangle).clone()]
        } else if in_count == 0 {
            vec![]
        } else if in_count == 1 {
            vec![]
        } else {
            vec![]
        }
    }

    fn signed_distance(&self, plane: &Plane, center: &Vec3) -> f32 {
        plane.normal.dot(center) + plane.distance
    }

    fn viewport_to_canvas(&self, x: f32, y: f32) -> Vec2 {
        Vec2::new(
            x * self.canvas_size.w as f32 / self.viewport_size.w as f32,
            y * self.canvas_size.h as f32 / self.viewport_size.h as f32,
        )
    }

    fn canvas_to_viewport(&self, x: f32, y: f32) -> Vec2 {
        Vec2::new(
            x * 1.0 / self.canvas_size.w as f32,
            y * 1.0 / self.canvas_size.h as f32,
        )
    }

    fn project_vertex(&self, v: &Vec3) -> Vec2 {
        self.viewport_to_canvas(v.x * 1.0 / v.z, v.y * 1.0 / v.z)
    }

    fn un_project_vertex(&self, x: f32, y: f32, z: f32) -> Vec3 {
        let oz = 1.0 / z;
        let ux = x * oz / 1.0;
        let uy = y * oz / 1.0;
        let p2d = self.canvas_to_viewport(ux, uy);
        vec3(p2d.x, p2d.y, oz)
    }

    fn swap(&mut self, p0: &mut Vec2, p1: &mut Vec2) {
        let x = p0.x;
        let y = p0.y;
        p0.x = p1.x;
        p0.y = p1.y;
        p1.x = x;
        p1.y = y;
    }

    fn sort_vertex_indexes(&self, triangle: &Triangle, projected: &Vec<Vec2>) -> [usize; 3] {
        let mut sort: [usize; 3] = [0, 1, 2];
        let indexes = &triangle.indexes;

        if projected[indexes[sort[1]]].y < projected[indexes[sort[0]]].y {
            let swap = sort[0];
            sort[0] = sort[1];
            sort[1] = swap;
        }

        if projected[indexes[sort[2]]].y < projected[indexes[sort[0]]].y {
            let swap = sort[0];
            sort[0] = sort[2];
            sort[2] = swap;
        }

        if projected[indexes[sort[2]]].y < projected[indexes[sort[1]]].y {
            let swap = sort[1];
            sort[1] = sort[2];
            sort[2] = swap;
        }

        sort
    }

    fn edge_interpolate(
        &self,
        y0: f32, v0: f32,
        y1: f32, v1: f32,
        y2: f32, v2: f32,
    ) -> (Vec<f32>, Vec<f32>) {
        let mut x01 = self.interpolate(y0, v0, y1, v1);
        let x12 = self.interpolate(y1, v1, y2, v2);
        let x02 = self.interpolate(y0, v0, y2, v2);

        x01.remove(x01.len() - 1);

        let x012 = [x01, x12].concat();

        (x02, x012)
    }

    fn interpolate(
        &self,
        i0: f32, d0: f32,
        i1: f32, d1: f32,
    ) -> Vec<f32> {
        let mut values: Vec<f32> = vec![];

        if i0 == i1 {
            values.push(d0)
        } else {
            let a = (d1 - d0) / (i1 - i0);
            let mut d = d0;
            for _ in i0 as i32..=i1 as i32 {
                values.push(d);
                d = d + a;
            }
        }

        values
    }

    fn compute_illumination(&self, vertex: &Vec3, normal: &Vec3) -> f32 {
        let mut illumination = 0.0;
        for light in self.lights.iter() {
            let vl = match light {
                Light::Ambient { intensity } => {
                    illumination += intensity;
                    continue;
                }
                Light::Directional { direction, .. } => {
                    self.camera.get_orientation().transpose() * vec4(direction.x, direction.y, direction.z, 1.0)
                }
                Light::Point { position, .. } => {
                    -vec4(vertex.x, vertex.y, vertex.z, 1.0) + self.camera.get_matrix() * vec4(position.x, position.y, position.z, 1.0)
                }
            };

            let vl = vl.xyz();

            let cos_alpha = vl.dot(&normal) / (length(&vl) * length(&normal));
            if cos_alpha > 0.0 {
                illumination += cos_alpha * light.intensity();
            }

            let reflected = normal * (2.0 * normal.dot(&vl)) - vl;
            let view = self.camera.position - vertex;
            let cos_beta = reflected.dot(&view) / (length(&reflected) * length(&view));
            if cos_beta > 0.0 {
                illumination += cos_beta.powi(50) * light.intensity();
            }
        }

        illumination
    }
}

impl Renderer {
    fn update_depth_buf_if_closer(&mut self, cx: i32, cy: i32, z: f32) -> bool {
        let w = self.canvas_size.w as f32;
        let h = self.canvas_size.h as f32;


        let x = (w / 2.0 + cx as f32).ceil();
        let y = (h / 2.0 - cy as f32 - 1.0).ceil();

        if x < 0.0 || x >= w || y < 0.0 || y >= h {
            return false;
        }

        let offset = (x + w * y) as usize;
        if self.depth_buf[offset] == 0.0 || self.depth_buf[offset] < z {
            self.depth_buf[offset] = z;
            return true;
        }

        false
    }

    fn put_pixels(&mut self, cx: i32, cy: i32, color: Rgb<f32>) {
        let w = self.canvas_size.w as f32;
        let h = self.canvas_size.h as f32;


        let x = (w / 2.0 + cx as f32).ceil();
        let y = (h / 2.0 - cy as f32 - 1.0).ceil();

        if x < 0.0 || x >= w || y < 0.0 || y >= h {
            return;
        }

        // info!("c[{:}:{:}] / s[{:}:{:}]", cx, cy, x, y);

        let offset = (4.0 * (x + w * y)) as usize;
        self.data[offset + 0] = color.r as u8;
        self.data[offset + 1] = color.g as u8;
        self.data[offset + 2] = color.b as u8;
        self.data[offset + 3] = 255;
    }
}
