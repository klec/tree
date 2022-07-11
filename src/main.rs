extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::thread_rng as random;
use rand::Rng;

struct Branch {
    a: f64,
    b: f64,
    l: f64,
    deep: f64,
    weight: f64,
    color: [f32; 4],
    left: Option<Box<Branch>>,
    right: Option<Box<Branch>>,
}

impl Branch {
    fn new(a: f64, b: f64, deep: f64) -> Self {
        Branch {
            deep: deep + 1.0,
            left: None,
            right: None,
            weight: 0.3,
            color: [0.0, 0.0, 0.0, 1.0],
            a: a / 3.0 + random().gen_range(0.0..1.0),
            b: b + random().gen_range(-2.0..2.0),
            l: random().gen_range(40.0..60.0) - deep * 2.0,
        }
    }

    fn rotate(&mut self, angle: f64) {
        self.b += angle;

        match &mut self.left {
            None => {}
            Some(x) => {
                &x.rotate(angle);
            }
        }
        match &mut self.right {
            None => {}
            Some(x) => {
                &x.rotate(angle);
            }
        }
    }

    fn grove(&mut self) {
        self.weight = self.weight + 0.3;

        match &mut self.left {
            None => self.left = Some(Box::new(Self::new(self.a, self.b, self.deep))),
            Some(x) => x.grove(),
        }

        match &mut self.right {
            None => {
                if random().gen::<f64>() < 0.12 {
                    self.right = Some(Box::new(Self::new(self.a, self.b, self.deep)))
                }
            }
            Some(x) => x.grove(),
        }
    }

    fn display(&self, x: f64, y: f64, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
        use graphics::*;
        let mut dx: f64 = self.a.sin() * self.l;
        let mut dy: f64 = self.a.cos() * self.l;
        line(
            self.color,
            self.weight,
            [x, y, x - dx * self.b.cos(), y - dy],
            transform,
            gl,
        );

        dx -= 2.0;
        dy -= 2.0;

        match &self.left {
            None => {}
            Some(branch) => branch.display(x - dx * self.b.cos(), y - dy, transform, gl),
        }

        match &self.right {
            None => {}
            Some(branch) => branch.display(x - dx * self.b.cos(), y - dy, transform, gl),
        }
    }
}

pub struct App {
    gl: GlGraphics,
    tree: Branch,
    mousex: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let tree = &self.tree;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let transform = c.transform.trans(400.0, 700.0);
            &tree.display(0.0, 0.0, transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        &self.tree.rotate(0.001);
    }

    fn init(&mut self) {
        self.tree = Branch::new(0.0, 0.0, 1.0);
        for _n in 1..20 {
            self.tree.grove();
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Tree", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        tree: Branch::new(0.0, 0.0, 1.0),
        mousex: 600.0,
    };

    app.init();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            app.init();
        }

        e.mouse_cursor(|pos| {
            let _dif = app.mousex - pos[0];
            app.mousex = pos[0];
            app.tree.rotate(_dif / 100.0);
        });
    }
}
