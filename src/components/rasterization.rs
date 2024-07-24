use log::info;
use nalgebra_glm::{length, normalize, Vec2, Vec3};
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

        let w = self.canvas.width() as i32;
        let h = self.canvas.height() as i32;

        info!("w{:}/h{:}", w, h);

        let mut image_data = context
            .get_image_data(0.0, 0.0, w as f64, h as f64)
            .unwrap()
            .data()
            .0;

        let color = Rgb::new(255.0, 255.0, 255.0);
        self.draw_line(Vec2::new(-150.0, -150.0), Vec2::new(150.0, 150.0), color, &mut image_data);
        self.draw_line(Vec2::new(-150.0, 150.0), Vec2::new(150.0, -150.0), color, &mut image_data);
        self.draw_line(Vec2::new(-150.0, 0.0), Vec2::new(150.0, -0.0), color, &mut image_data);
        self.draw_line(Vec2::new(0.0, 150.0), Vec2::new(0.0, -150.0), color, &mut image_data);

        let end = self.performance.now();
        info!("execution: {:?}", end - start);

        let data = ImageData::new_with_u8_clamped_array(Clamped(image_data.as_slice()), w as u32).unwrap();

        context.put_image_data(&data, 0.0, 0.0).expect("TODO: panic message");
    }


    fn draw_line (&self, mut p0: Vec2, mut p1: Vec2, color: Rgb<f32>, data: &mut Vec<u8>) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            if p0.x > p1.x {
                self.swap(&mut p0, &mut p1)
            }

            let x0 = p0.x as i32;
            let x1 = p1.x as i32;
            let ys = self.interpolate(x0, p0.y, x1, p1.y);

            for x in x0..=x1 {
                let y = (x-x0) as usize;
                self.put_pixels(x, ys[y] as i32, color, data);
            }
        } else {
            if p0.y > p1.y {
                self.swap(&mut p0, &mut p1)
            }

            let y0 = p0.y as i32;
            let y1 = p1.y as i32;
            let xs = self.interpolate(y0, p0.x, y1, p1.x);
            for y in y0..=y1 {
                let x = (y-y0) as usize;
                self.put_pixels(xs[x] as i32, y, color, data);
            }
        }
    }

    fn swap(&self, p0: &mut Vec2, p1: &mut Vec2) {
        let x = p0.x;
        let y = p0.y;
        p0.x = p1.x;
        p0.y = p1.y;
        p1.x = x;
        p1.y = y;
    }

    fn interpolate(&self, i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
        let mut values: Vec<f32> = vec![];

        if i0 == i1 {
            values.push(d0)
        } else {
            let a = (d1 - d0) / (i1 - i0) as f32;
            let mut d = d0;
            for i in i0 as i32..=i1 as i32 {
                values.push(d);
                d = d + a;
            }
        }

        values
    }

    fn put_pixels(&self, cx: i32, cy: i32, color: Rgb<f32>, data: &mut Vec<u8>) {
        let w = self.canvas.width() as f32;
        let h = self.canvas.height() as f32;


        let x = (w / 2.0 + cx as f32).ceil();
        let y = (h / 2.0 - cy as f32 - 1.0).ceil();

        if x < 0.0 || x >= w || y < 0.0 || y >= h {
            return;
        }

        info!("c[{:}:{:}] / s[{:}:{:}]", cx, cy, x, y);

        let offset: usize = (4.0 * (x + w * y)) as usize;
        data[offset + 0] = color.r as u8;
        data[offset + 1] = color.g as u8;
        data[offset + 2] = color.b as u8;
        data[offset + 3] = 255;
    }
}