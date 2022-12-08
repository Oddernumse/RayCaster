use ggez::{*, graphics::{Color, Canvas}, glam::Vec2};
use ggez::input::keyboard::KeyCode;

struct MainState {
    dt: std::time::Duration,
    pos_x: f32,
    pos_y: f32,
}

struct Core {
    height: f32,
    width: f32,
}

fn draw_line(ctx: &mut Context, canvas: &mut Canvas, pos_x: f32, pos_y: f32, core_size: &mut Core) {
    // let mb = &mut graphics::MeshBuilder::new();

    let core_midx = pos_x+core_size.width/2.0;
    let core_midy = pos_y+core_size.height/2.0;

    let line = graphics::Mesh::new_line(ctx, &[
        Vec2::new(pos_x+core_size.width/2.0, pos_y+core_size.height/2.0),
        Vec2::new(pos_x+core_size.width+150.0, pos_y+core_size.height+150.0),

        Vec2::new(pos_x+core_size.width/2.0, pos_y+core_size.height/2.0),
        Vec2::new(pos_x-150.0, pos_y-150.0),

        Vec2::new(pos_x+core_size.width/2.0, pos_y+core_size.height/2.0),
        Vec2::new(pos_x-150.0, pos_y+155.0)

        ], 1.0, Color::GREEN).unwrap();

    canvas.draw(
        &line,
        graphics::DrawParam::new()
    );
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();

        let k_ctx = &ctx.keyboard;

        if k_ctx.is_key_pressed(KeyCode::Right) {
            self.pos_x += 1.0;
        } else if k_ctx.is_key_pressed(KeyCode::Left) {
            self.pos_x -= 1.0;
        } else if k_ctx.is_key_pressed(KeyCode::Up) {
            self.pos_y -= 1.0;
        } else if k_ctx.is_key_pressed(KeyCode::Down) {
            self.pos_y += 1.0;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("render time: {}ms", self.dt.as_millis());

        let mut core_size = Core {
            height: 125.0,
            width: 167.0,
        };

        // You probably change window size here but idfk what GraphicsContext is, in the context of &mut self

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.0]));
        draw_line(ctx, &mut canvas, self.pos_x, self.pos_y, &mut core_size);

        let core = graphics::Rect::new(self.pos_x, self.pos_y, core_size.width, core_size.height);
        let test_rect = graphics::Rect::new(250.0, 175.0, 150.0, 150.0);

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(core.point())
                .scale(core.size())
                .color(Color::BLUE),
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(test_rect.point())
                .scale(test_rect.size())
                .color(Color::RED)
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() {
    let state = MainState {
        dt: std::time::Duration::new(0,0),
        pos_x: 100.0,
        pos_y: 100.0,
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("RayCaster", "Odd")
        .default_conf(c)
        .build()
        .unwrap();
    
    event::run(ctx, event_loop, state);
}
