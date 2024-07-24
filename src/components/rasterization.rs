use log::info;
use nalgebra_glm::{Vec2};
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

        info!("w{:}/h{:}", w, h);


        let mut drawer = Drawer {
            data: context
                .get_image_data(0.0, 0.0, w as f64, h as f64)
                .unwrap()
                .data()
                .0,
            weight: w,
            height: h,
        };

        let color = Rgb::new(255.0, 255.0, 255.0);
        drawer.draw_line(Vec2::new(-150.0, -150.0), Vec2::new(150.0, 150.0), color);
        drawer.draw_line(Vec2::new(-150.0, 150.0), Vec2::new(150.0, -150.0), color);
        drawer.draw_line(Vec2::new(-150.0, 0.0), Vec2::new(150.0, -0.0), color);
        drawer.draw_line(Vec2::new(0.0, 150.0), Vec2::new(0.0, -150.0), color);
        drawer.draw_filled_triangle(Vec2::new(0.0, 85.0), Vec2::new(-120.0, -70.0), Vec2::new(120.0, -20.0), color);

        let end = self.performance.now();
        info!("execution: {:?}", end - start);

        let data = ImageData::new_with_u8_clamped_array(Clamped(drawer.data.as_slice()), w as u32).unwrap();

        context.put_image_data(&data, 0.0, 0.0).expect("TODO: panic message");
    }
}

struct Drawer {
    data: Vec<u8>,
    weight: u32,
    height: u32,
}

impl Drawer {
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

    fn draw_filled_triangle(&mut self, mut p0: Vec2, mut p1: Vec2, mut p2: Vec2, color: Rgb<f32>) {
        if p1.y < p0.y {
            self.swap(&mut p1, &mut p0)
        }

        if p2.y < p0.y {
            self.swap(&mut p2, &mut p0)
        }

        if p2.y < p1.y {
            self.swap(&mut p2, &mut p1)
        }

        let mut x01 = self.interpolate(p0.y, p0.x, p1.y, p1.x);
        let x12 = self.interpolate(p1.y, p1.x, p2.y, p2.x);
        let x02 = self.interpolate(p0.y, p0.x, p2.y, p2.x);

        x01.remove(x01.len() - 1);

        let x012 = [x01, x12].concat();

        let x_left;
        let x_right;
        let m = x012.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012;
        } else {
            x_left = x012;
            x_right = x02;
        }


        let y0 = p0.y as i32;
        let y2 = p2.y as i32;

        for y in y0..=y2 {
            for x in x_left[(y - y0) as usize] as i32..=x_right[(y - y0) as usize] as i32 {
                self.put_pixels(x, y, color);
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

    fn interpolate(&self, i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
        let mut values: Vec<f32> = vec![];

        if i0 == i1 {
            values.push(d0)
        } else {
            let a = (d1 - d0) / (i1 - i0) as f32;
            let mut d = d0;
            for _ in i0 as i32..=i1 as i32 {
                values.push(d);
                d = d + a;
            }
        }

        values
    }

    fn put_pixels(&mut self, cx: i32, cy: i32, color: Rgb<f32>) {
        let w = self.weight as f32;
        let h = self.height as f32;


        let x = (w / 2.0 + cx as f32).ceil();
        let y = (h / 2.0 - cy as f32 - 1.0).ceil();

        if x < 0.0 || x >= w || y < 0.0 || y >= h {
            return;
        }

        info!("c[{:}:{:}] / s[{:}:{:}]", cx, cy, x, y);

        let offset: usize = (4.0 * (x + w * y)) as usize;
        self.data[offset + 0] = color.r as u8;
        self.data[offset + 1] = color.g as u8;
        self.data[offset + 2] = color.b as u8;
        self.data[offset + 3] = 255;
    }
}