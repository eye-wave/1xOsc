use lazy_static::lazy_static;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use rand::thread_rng;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum Point {
    Line(f32, f32),
}

pub struct Icon {
    points: Vec<Point>,
}

impl View for Icon {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bg_color: vg::Color = cx.background_color().into();
        let stroke_color: vg::Color = cx.font_color().into();

        let margin = 16.0;
        let bounds = cx.bounds();

        let x = bounds.x + margin / 2.0;
        let y = bounds.y + margin / 2.0;
        let w = bounds.w - margin;
        let h = bounds.h - margin;

        let mut path = vg::Path::new();
        path.rect(x, y, w, h);
        path.close();

        let paint = vg::Paint::color(bg_color);
        canvas.fill_path(&path, &paint);

        let mut path = vg::Path::new();
        path.move_to(x, y + h / 2.0);
        for point in self.points.iter() {
            match *point {
                Point::Line(px, py) => path.line_to(x + px * w, y + py * h),
            }
        }

        if let Some(Point::Line(px, py)) = self.points.last() {
            path.move_to(x + px * w, y + py * h);
        }

        path.close();

        let paint = vg::Paint::color(stroke_color);
        canvas.stroke_path(&path, &paint);
    }
}

impl Icon {
    pub fn new(cx: &mut Context, points: Vec<Point>) -> Handle<Self> {
        Self { points }.build(cx, |_| {})
    }
}

lazy_static! {
    pub static ref SINE_ICON_POINTS: Vec<Point> = (0..50)
        .map(|n| {
            Point::Line(
                (n as f32) / 50.0,
                1.0 - (((n as f32) / 25.0 * PI).sin() / 2.0 + 0.5),
            )
        })
        .collect();
    pub static ref TRIANGLE_ICON_POINTS: Vec<Point> = vec![
        Point::Line(0.25, 0.0),
        Point::Line(0.75, 1.0),
        Point::Line(1.0, 0.5),
    ];
    pub static ref SQUARE_ICON_POINTS: Vec<Point> = vec![
        Point::Line(0.0, 0.0),
        Point::Line(0.5, 0.0),
        Point::Line(0.5, 1.0),
        Point::Line(1.0, 1.0),
        Point::Line(1.0, 0.5),
    ];
    pub static ref SAW_ICON_POINTS: Vec<Point> = vec![
        Point::Line(0.5, 0.0),
        Point::Line(0.5, 1.0),
        Point::Line(1.0, 0.5),
    ];
    pub static ref NOISE_ICON_POINTS: Vec<Point> = (0..25)
        .map(|n| Point::Line((n as f32) / 25.0, {
            if n % 2 == 0 {
                thread_rng().gen_range(0.0..0.5)
            } else {
                thread_rng().gen_range(0.5..1.0)
            }
        }))
        .collect();
}
