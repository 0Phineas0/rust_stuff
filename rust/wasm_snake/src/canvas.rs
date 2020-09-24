use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    x_scaled: u32,
    y_scaled: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Self {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let x_scaled = canvas.width() / width;
        let y_scaled = canvas.height() / height;

        Self {
            canvas,
            ctx,
            x_scaled,
            y_scaled,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.ctx.set_fill_style_color(color);

        let x = x * self.x_scaled;
        let y = y * self.y_scaled;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.x_scaled),
            f64::from(self.y_scaled),
        );
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.canvas.width()),
            f64::from(self.height * self.y_scaled),
        )
    }
}
