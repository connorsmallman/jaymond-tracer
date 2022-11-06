pub mod canvas {
    use crate::tuple::tuple::Color;
    use insta::assert_debug_snapshot;

    pub struct Canvas {
        pub(crate) width: usize,
        pub(crate) height: usize,
        pixels: Vec<Color>,
    }

    impl Canvas {
        pub fn new(width: usize, height: usize) -> Canvas {
            Canvas {
                width,
                height,
                pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
            }
        }

        pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
            self.pixels[y * self.width + x] = color;
        }

        pub fn pixel_at(&self, x: usize, y: usize) -> Color {
            self.pixels[y * self.width + x]
        }

        fn tuple_to_color(&self, color: Color) -> (u8, u8, u8) {
            let r = (color.red.min(1.0).max(0.0) * 255.0).round() as u8;
            let g = (color.green.min(1.0).max(0.0) * 255.0).round() as u8;
            let b = (color.blue.min(1.0).max(0.0) * 255.0).round() as u8;
            (r, g, b)
        }

        pub fn to_ppm(&self) -> String {
            let mut ppm = String::new();
            ppm.push_str(&format!("P3 {} {} 255", self.width, self.height));
            for y in 0..self.height {
                ppm.push_str("\n");
                for x in 0..self.width {
                    let color = self.pixel_at(x, y);
                    ppm.push_str(&format!("{} {} {} ", self.tuple_to_color(color).0, self.tuple_to_color(color).1, self.tuple_to_color(color).2));
                }
            }
            ppm
        }
    }

    #[test]
    fn canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        assert_eq!(canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn canvas_to_ppm() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        let ppm = canvas.to_ppm();
        assert_debug_snapshot!(ppm);
    }
}