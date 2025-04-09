use log::info;
use nalgebra_glm::{length, normalize, Vec3};
use rgb::Rgb;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, ImageData, Performance};
use yew::{html, Component, Context, Html, NodeRef};
use crate::webapp::components::graphics::Light;

pub struct RaytracingCanvas {
    canvas_ref: NodeRef,
}

#[derive(Debug)]
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
struct Range {
    pub min: f32,
    pub max: f32,
}
impl Range {
    pub fn isin(&self, t: f32) -> bool {
        self.min < t && t < self.max
    }
}

struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Rgb<f32>,
    pub specular: f32,
    pub reflective: f32,
}

impl Component for RaytracingCanvas {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} width="300" height="300"/>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas: HtmlCanvasElement = self.canvas_ref.cast().unwrap();
            ctx.link().send_future(async move {
                let raytracer = RayTracer {
                    performance: window().unwrap().performance().unwrap(),
                    spheres: vec![
                        Sphere {
                            center: Vec3::new(0.0, -1.0, 3.0),
                            radius: 1.0,
                            color: Rgb::new(255.0, 0.0, 0.0),
                            specular: 500.0,
                            reflective: 0.2,
                        },
                        Sphere {
                            center: Vec3::new(2.0, 0.0, 4.0),
                            radius: 1.0,
                            color: Rgb::new(0.0, 0.0, 255.0),
                            specular: 500.0,
                            reflective: 0.3,
                        },
                        Sphere {
                            center: Vec3::new(-2.0, 0.0, 4.0),
                            radius: 1.0,
                            color: Rgb::new(0.0, 255.0, 0.0),
                            specular: 10.0,
                            reflective: 0.4,
                        },
                        Sphere {
                            center: Vec3::new(0.0, -5001.0, 0.0),
                            radius: 5000.0,
                            color: Rgb::new(255.0, 255.0, 0.0),
                            specular: 1000.0,
                            reflective: 0.5,
                        },
                    ],
                    lights: vec![
                        Light::Ambient { intensity: 0.2 },
                        Light::Point { intensity: 0.6, position: Vec3::new(2.0, 1.0, 0.0) },
                        Light::Directional { intensity: 0.2, direction: Vec3::new(1.0, 4.0, 4.0) },
                    ],
                };

                raytracer.render(canvas.clone());
            })
        }
    }
}

struct RayTracer {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
    performance: Performance,
}

impl RayTracer {
    fn render(&self, canvas: HtmlCanvasElement) {
        let start = self.performance.now();

        let context = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let w = canvas.width() as i32;
        let h = canvas.height() as i32;

        let mut image_data = context
            .get_image_data(0.0, 0.0, w as f64, h as f64)
            .unwrap()
            .data()
            .0;

        for x in (-w / 2)..=(w / 2) {
            for y in (-h / 2)..=(h / 2) {
                let ray = Ray {
                    origin: Vec3::new(0.0, 0.0, 0.0),
                    direction: self.canvas_to_viewport(x as f32, y as f32, w as f32, h as f32),
                };

                let range = Range { min: 1.0, max: f32::INFINITY };

                let color = self.tracy_ray(&ray, &range, 3);
                self.put_pixels(x as f32, y as f32, w as f32, h as f32, color, &mut image_data);
            }
        }

        let end = self.performance.now();
        info!("execution: {:?}", end - start);

        let data = ImageData::new_with_u8_clamped_array(Clamped(image_data.as_slice()), w as u32).unwrap();

        context.put_image_data(&data, 0.0, 0.0).expect("TODO: panic message");
    }

    fn put_pixels(&self, x: f32, y: f32, w: f32, h: f32, color: Rgb<f32>, data: &mut Vec<u8>) {
        let x = w / 2.0 + x;
        let y = h / 2.0 - y - 1.0;

        if x < 0.0 || x >= w || y < 0.0 || y >= h {
            return;
        }

        let offset: usize = (4.0 * (x + w * y)) as usize;
        data[offset + 0] = color.r as u8;
        data[offset + 1] = color.g as u8;
        data[offset + 2] = color.b as u8;
        data[offset + 3] = 255;
    }

    fn canvas_to_viewport(&self, x: f32, y: f32, w: f32, h: f32) -> Vec3 {
        Vec3::new(
            x * 1.0 / w,
            y * 1.0 / h,
            1.0,
        )
    }
}

impl RayTracer {
    fn tracy_ray(&self, ray: &Ray, range: &Range, depth: u32) -> Rgb<f32> {
        let intersection = self.closest_intersection(&ray, &range);

        if let Some((closest_t, closest_sphere)) = intersection {
            let point = ray.origin + closest_t * ray.direction;
            let normal = point - closest_sphere.center;
            let normal = normalize(&normal);


            let view = -ray.direction;
            let lighting = self.compute_lighting(
                &Ray {
                    origin: point,
                    direction: view,
                },
                &normal,
                closest_sphere.specular,
            );
            let local_color = closest_sphere.color * lighting;

            let r = closest_sphere.reflective;
            if r <= 0.0 || depth <= 0 {
                local_color
            } else {
                let reflected_ray = Ray {
                    origin: point,
                    direction: self.reflect_ray(&view, &normal),
                };

                let reflected_color = self.tracy_ray(&reflected_ray, &Range {
                    min: 0.1,
                    max: f32::INFINITY,
                }, depth - 1);

                local_color * (1.0 - r) + reflected_color * r
            }
        } else {
            Rgb::new(0.0, 0.0, 0.0)
        }
    }
    fn closest_intersection(&self, ray: &Ray, range: &Range) -> Option<(f32, &Sphere)> {
        let mut closest_sphere: Option<&Sphere> = None;
        let mut closest_t = f32::INFINITY;

        for sphere in self.spheres.iter() {
            let (t1, t2) = self.intersect_ray_sphere(&ray, &sphere);
            if t1 < closest_t && range.isin(t1) {
                closest_t = t1;
                closest_sphere = Some(sphere)
            }

            if t2 < closest_t && range.isin(t2) {
                closest_t = t2;
                closest_sphere = Some(sphere)
            }
        }

        if let None = closest_sphere {
            None
        } else {
            Some((closest_t, closest_sphere.unwrap()))
        }
    }
    fn intersect_ray_sphere(&self, ray: &Ray, sphere: &Sphere) -> (f32, f32) {
        let oc = ray.origin - sphere.center;
        let r = sphere.radius;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            (f32::INFINITY, f32::INFINITY)
        } else {
            (
                (-b + discriminant.sqrt()) / (2.0 * a),
                (-b - discriminant.sqrt()) / (2.0 * a)
            )
        }
    }

    fn compute_lighting(&self, ray: &Ray, normal: &Vec3, specular: f32) -> f32 {
        let mut i: f32 = 0.0;
        let length_n = length(&normal);
        let length_v = length(&ray.direction);

        for light in self.lights.iter() {
            match light {
                Light::Ambient { intensity } => {
                    i += intensity;
                }
                other => {
                    let vec_l;
                    let t_max: f32;

                    match other {
                        Light::Point { position, .. } => {
                            vec_l = position - ray.origin;
                            t_max = 1.0;
                        }
                        Light::Directional { direction, .. } => {
                            vec_l = direction.clone();
                            t_max = f32::INFINITY;
                        }
                        _ => { continue }
                    }

                    let intersection = self.closest_intersection(&Ray {
                        origin: ray.origin,
                        direction: vec_l,
                    }, &Range { min: 0.001, max: t_max });

                    if intersection.is_some() {
                        continue;
                    }

                    let n_dot_l = normal.dot(&vec_l);
                    if n_dot_l > 0.0 {
                        i += light.intensity() * n_dot_l / (length_n * length(&vec_l));
                    }

                    if specular != -1.0 {
                        let vec_r = normal * (2.0 * n_dot_l) * normal.dot(&vec_l) - vec_l;
                        let r_dot_v = vec_r.dot(&ray.direction);
                        if r_dot_v > 0.0 {
                            let b = r_dot_v / (length(&vec_r) * length_v);
                            i += light.intensity() * b.powf(specular);
                        }
                    }
                }
            }
        }
        i
    }

    fn reflect_ray(&self, direction: &Vec3, normal: &Vec3) -> Vec3 {
        2.0 * normal * normal.dot(direction) - direction
    }
}