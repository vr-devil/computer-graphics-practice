use log::info;
use nalgebra_glm::{identity, Mat4x4, rotate_y, scaling, translation, Vec2, Vec3, vec3, Vec4, vec4, triangle_normal};
use rgb::Rgb;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, Performance, window};
use yew::{Component, Context, Html, html, NodeRef};

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
                let raytracer = Rasterizer {
                    canvas,
                    performance: window().unwrap().performance().unwrap(),
                };

                raytracer.render();
            })
        }
    }
}

struct Rasterizer {
    canvas: HtmlCanvasElement,
    performance: Performance,
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


        let mut drawer = Renderer {
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
        drawer.draw_line(Vec2::new(-150.0, -150.0), Vec2::new(150.0, 150.0), color);
        drawer.draw_line(Vec2::new(-150.0, 150.0), Vec2::new(150.0, -150.0), color);
        drawer.draw_line(Vec2::new(-150.0, 0.0), Vec2::new(150.0, -0.0), color);
        drawer.draw_line(Vec2::new(0.0, 150.0), Vec2::new(0.0, -150.0), color);
        // drawer.draw_filled_triangle(Vec2::new(-120.0, -70.0), Vec2::new(0.0, 85.0), Vec2::new(120.0, -20.0), color);
        // drawer.draw_shaded_triangle(
        //     Point { coord: Vec2::new(-120.0, 70.0), h: 1.0 },
        //     Point { coord: Vec2::new(0.0, 85.0), h: 0.3 },
        //     Point { coord: Vec2::new(120.0, -20.0), h: 0.5 },
        //     Rgb::new(0.0, 255.0, 0.0),
        // );
        // drawer.draw_cube(&Cube {
        //     af: Vec3::new(-2.0, -0.5, 5.0),
        //     bf: Vec3::new(-2.0, 0.5, 5.0),
        //     cf: Vec3::new(-1.0, 0.5, 5.0),
        //     df: Vec3::new(-1.0, -0.5, 5.0),
        //
        //     ab: Vec3::new(-2.0, -0.5, 6.0),
        //     bb: Vec3::new(-2.0, 0.5, 6.0),
        //     cb: Vec3::new(-1.0, 0.5, 6.0),
        //     db: Vec3::new(-1.0, -0.5, 6.0),
        // });

        let red: Rgb<f32> = Rgb::new(255.0, 0.0, 0.0);
        let green: Rgb<f32> = Rgb::new(0.0, 255.0, 0.0);
        let blue: Rgb<f32> = Rgb::new(0.0, 0.0, 255.0);
        let yellow: Rgb<f32> = Rgb::new(255.0, 255.0, 0.0);
        let purple: Rgb<f32> = Rgb::new(255.0, 0.0, 255.0);
        let cyan: Rgb<f32> = Rgb::new(0.0, 255.0, 255.0);

        let cube = Model {
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
                Triangle::new(0, 1, 2, red.clone()),
                Triangle::new(0, 2, 3, red.clone()),
                Triangle::new(1, 5, 6, yellow.clone()),
                Triangle::new(1, 6, 2, yellow.clone()),
                Triangle::new(2, 6, 7, cyan.clone()),
                Triangle::new(2, 7, 3, cyan.clone()),
                Triangle::new(4, 0, 3, green.clone()),
                Triangle::new(4, 3, 7, green.clone()),
                Triangle::new(4, 5, 1, purple.clone()),
                Triangle::new(4, 1, 0, purple.clone()),
                Triangle::new(5, 4, 7, blue.clone()),
                Triangle::new(5, 7, 6, blue.clone()),
            ],
            bounds_center: Default::default(),
            bounds_radius: 3.0f32.sqrt(),
        };


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
        let instances = vec![
            Instance { model: cube.clone(), position: vec3(-1.5, 0.0, 7.0), rotation: 0.0, scale: 0.75 },
            Instance { model: cube.clone(), position: vec3(1.25, 2.5, 7.5), rotation: 195.0, scale: 1.0 },
            Instance { model: cube.clone(), position: vec3(3.5, -1.0, 7.0), rotation: 30.0, scale: 1.0 },
        ];
        drawer.render_scene(&camera, instances);

        let end = self.performance.now();
        info!("execution: {:?}", end - start);

        let data = ImageData::new_with_u8_clamped_array(Clamped(drawer.data.as_slice()), w).unwrap();

        context.put_image_data(&data, 0.0, 0.0).expect("TODO: panic message");
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
    pub a: usize,
    pub b: usize,
    pub c: usize,

    pub color: Rgb<f32>,
    pub h1: f32,
    pub h2: f32,
    pub h3: f32,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize, color: Rgb<f32>) -> Self {
        Self { a, b, c, color, h1: 1.0, h2: 1.0, h3: 1.0 }
    }

    fn new_with_intensity(indexes: (usize, usize, usize), intensities: (f32, f32, f32), color: Rgb<f32>) {
        let (a, b, c) = indexes;
        let (h1, h2, h3) = intensities;
        Self { a, b, c, h1, h2, h3, color };
    }

    fn points(&self, projected: &Vec<Vec2>) -> (Point, Point, Point) {
        (
            Point { coord: projected[self.a].clone(), h: self.h1 },
            Point { coord: projected[self.b].clone(), h: self.h2 },
            Point { coord: projected[self.c].clone(), h: self.h3 },
        )
    }
}

struct Camera {
    pub position: Vec3,
    pub rotation: f32,
    pub planes: Vec<Plane>,
}

impl Camera {
    fn get_matrix(&self) -> Mat4x4 {
        rotate_y(&identity(), self.rotation.to_radians()) * translation(&-self.position)
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
        let r = rotate_y(&identity(), self.rotation.to_radians());
        let t = translation(&self.position);

        t * r * s
    }
}

struct Renderer {
    data: Vec<u8>,
    depth_buf: Vec<f32>,
    canvas_size: Rectangle,
    viewport_size: Rectangle,
}

impl Renderer {
    fn render_scene(&mut self, camera: &Camera, instances: Vec<Instance>) {
        let camera_matrix = camera.get_matrix();
        for instance in instances {
            let transform = camera_matrix * instance.transform();
            if let Some(clipped_instance) = self.transform_and_clip(&camera, instance, transform) {
                self.render_model(&clipped_instance.model)
            }
        }

        info!("{:?}", self.depth_buf)
    }

    fn render_model(&mut self, model: &Model) {
        let mut projected = vec![];

        for v in model.vertices.iter() {
            projected.push(self.project_vertex(v));
        }

        for t in model.triangles.iter() {
            self.render_triangle(t, &model.vertices, &projected);
        }
    }

    fn render_triangle(&mut self, triangle: &Triangle, vertices: &Vec<Vec3>, projected: &Vec<Vec2>) {
        // self.draw_shaded_triangle(triangle, projected);
        self.draw_filled_triangle(triangle, vertices, projected);
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

    fn draw_filled_triangle(&mut self, triangle: &Triangle, vertices: &Vec<Vec3>, projected: &Vec<Vec2>) {
        let (i0, i1, i2) = self.sort_vertex_indexes(triangle, projected);

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];

        let normal = triangle_normal(&vertices[triangle.a], &vertices[triangle.b], &vertices[triangle.c]);
        let vertex_to_camera = -v0;
        if vertex_to_camera.dot(&normal) <= 0.0 {
            return;
        }

        let mut p0 = projected[i0].clone();
        let mut p1 = projected[i1].clone();
        let mut p2 = projected[i2].clone();
        let color = triangle.color.clone();

        let (x02, x012) = self.edge_interpolate(p0.y, p0.x, p1.y, p1.x, p2.y, p2.x);
        let (z02, z012) = self.edge_interpolate(p0.y, 1.0 / v0.z, p1.y, 1.0 / v1.z, p2.y, 1.0 / v2.z);

        let m = x02.len() / 2;
        let (
            (x_left, x_right),
            (z_left, z_right)
        ) = if x02[m] < x012[m] {
            ((x02, x012), (z02, z012))
        } else {
            ((x012, x02), (z012, z02))
        };

        let y0 = p0.y as i32;
        let y2 = p2.y as i32;

        for y in y0..=y2 {
            let (xl, xr) = (x_left[(y - y0) as usize], x_right[(y - y0) as usize]);
            let (zl, zr) = (z_left[(y - y0) as usize], z_right[(y - y0) as usize]);
            let zscan = self.interpolate(xl, zl, xr, zr);

            for x in xl as i32..=xr as i32 {
                let z = zscan[(x - xl as i32) as usize];
                if self.update_depth_buf_if_closer(x, y, z) {
                    self.put_pixels(x, y, color);
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
    fn transform_and_clip(&self, camera: &Camera, mut instance: Instance, transform: Mat4x4) -> Option<Instance> {
        let bc = &instance.model.bounds_center;
        let center = transform * vec4(bc.x, bc.y, bc.z, 1.0);
        let radius = instance.model.bounds_radius * instance.scale;

        for plane in camera.planes.iter() {
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
        for plane in camera.planes.iter() {
            let mut new_triangles = vec![];
            for triangle in instance.model.triangles.iter() {
                let clipped_triangles = self.clip_triangle(triangle, &vertices, plane);
                new_triangles.extend(clipped_triangles)
            }
            triangles = new_triangles
        }

        Some(Instance {
            model: Model {
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
        let a = &vertices[triangle.a];
        let b = &vertices[triangle.b];
        let c = &vertices[triangle.c];

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

    fn project_vertex(&self, v: &Vec3) -> Vec2 {
        self.viewport_to_canvas(v.x * 1.0 / v.z, v.y * 1.0 / v.z)
    }

    fn swap(&mut self, p0: &mut Vec2, p1: &mut Vec2) {
        let x = p0.x;
        let y = p0.y;
        p0.x = p1.x;
        p0.y = p1.y;
        p1.x = x;
        p1.y = y;
    }

    fn sort_vertex_indexes(&self, triangle: &Triangle, projected: &Vec<Vec2>) -> (usize, usize, usize) {
        let mut indexes = (triangle.a, triangle.b, triangle.c);

        if projected[indexes.1].y < projected[indexes.0].y {
            let swap = indexes.0;
            indexes.0 = indexes.1;
            indexes.1 = swap;
        }

        if projected[indexes.2].y < projected[indexes.0].y {
            let swap = indexes.0;
            indexes.0 = indexes.2;
            indexes.2 = swap;
        }

        if projected[indexes.2].y < projected[indexes.1].y {
            let swap = indexes.1;
            indexes.1 = indexes.2;
            indexes.2 = swap;
        }

        indexes
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
