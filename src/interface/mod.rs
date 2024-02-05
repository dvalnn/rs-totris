mod delta_time;
mod render_traits;
mod sub_rect;

use std::time::Duration;

use cgmath::{ElementWise, EuclideanSpace, Point2, Vector2};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

use crate::engine::{
    Color as EngineColor, Coordinate, Engine, Matrix, MoveKind, RotateKind,
};

use self::{
    delta_time::DeltaTime,
    render_traits::ScreenColor,
    sub_rect::{Align, SubRect},
};

const WINDOW_INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const PLACEHOLDER: Color = Color::RGB(0x66, 0x77, 0x77);

#[derive(Debug, Clone, Copy)]
struct Tick;

#[derive(Debug, Clone, Copy)]
struct LockTick;

#[derive(Debug, Clone, Copy)]
struct SoftDropTick;

pub fn run(engine: Engine) {
    let sdl = sdl2::init().expect("SDL2 initialization failed");

    let event_subsystem =
        sdl.event().expect("SDL2 event subsystem aquisition failed");

    event_subsystem
        .register_custom_event::<Tick>()
        .expect("Failed to register custom event");

    event_subsystem
        .register_custom_event::<LockTick>()
        .expect("Failed to register custom event");

    event_subsystem
        .register_custom_event::<SoftDropTick>()
        .expect("Failed to register custom event");

    let canvas = {
        let video =
            sdl.video().expect("SDL2 video subsystem aquisition failed");

        let window = video
            .window("rs-totris", WINDOW_INIT_SIZE.x, WINDOW_INIT_SIZE.y)
            .position_centered()
            .resizable()
            .build()
            .expect("Window creation failed");

        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Canvas creation failed")
    };

    let events = sdl.event_pump().expect("Event pump aquisition failed");
    game_loop(events, engine, canvas);
}

fn game_loop(
    mut events: sdl2::EventPump,
    mut engine: Engine,
    mut canvas: Canvas<Window>,
) {
    let mut soft_drop = false;
    let mut lock_down = false;

    let mut delta = DeltaTime::new();

    let mut tick_timer = Duration::default();
    let mut fast_timer = Duration::default();
    let mut _lock_timer = Duration::default();

    loop {
        delta.update();

        if engine.cursor_info().is_none() {
            engine.add_cursor();
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    let Ok(input) = Input::try_from(key) else {
                        continue;
                    };
                    match input {
                        Input::SoftDrop => {
                            soft_drop = true;
                        }
                        Input::HardDrop => {
                            lock_down = true;
                            engine.hard_drop();
                        }
                        Input::Move(kind) => {
                            let _ = engine.move_cursor(kind);
                        }
                        Input::Rotate(kind) => {
                            let _ = engine.rotate_cursor(kind);
                        }
                    }
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    soft_drop = false;
                }

                _ => {}
            }
        }

        //TODO: clean up this logic into a dedicated function
        {
            const SOFT_DROP_SPEED_UP: u32 = 20;

            tick_timer += delta.get();
            fast_timer += delta.get();

            let tick_time = engine.drop_time();
            let fast_tick_time = tick_time / SOFT_DROP_SPEED_UP;

            let tick = tick_timer >= tick_time;
            let fast_tick = fast_timer >= fast_tick_time;

            if (soft_drop && fast_tick) || tick {
                if engine.cursor_has_hit_bottom() {
                    lock_down = true;
                    engine.place_cursor();
                } else {
                    engine.tick_down();
                }
            }

            if fast_tick {
                fast_timer -= fast_tick_time;
            }

            if tick {
                tick_timer -= tick_time;
            }
        }

        if lock_down {
            engine.line_clear(|_| (/*canvas animation*/));
            lock_down = false;
        }

        draw(&mut canvas, &engine);
    }
}

enum Input {
    Rotate(RotateKind),
    Move(MoveKind),
    HardDrop,
    SoftDrop,
}

impl TryFrom<Keycode> for Input {
    type Error = ();
    fn try_from(key: Keycode) -> Result<Self, Self::Error> {
        Ok(match key {
            Keycode::Right => Input::Move(MoveKind::Right),
            Keycode::Left => Input::Move(MoveKind::Left),
            Keycode::Down => Input::SoftDrop,
            Keycode::Space => Input::HardDrop,
            Keycode::Z => Input::Rotate(RotateKind::CounterClockwise),
            Keycode::X => Input::Rotate(RotateKind::Clockwise),
            Keycode::Escape => todo!("Pause"),

            _ => return Err(()),
        })
    }
}

fn draw(canvas: &mut Canvas<Window>, engine: &Engine) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    //NOTE: draw graphics from here on out

    let viewport = canvas.viewport();
    let ui_square = SubRect::absolute(viewport, (1.0, 1.0), None);

    let matrix = ui_square
        .sub_rect((0.5, 1.0), None)
        .sub_rect((7.0 / 8.0, 7.0 / 8.0), None);

    let up_next = ui_square
        .sub_rect((0.25, 0.25), Some((Align::Far, Align::Near)))
        .sub_rect((0.75, 0.75), None);

    let hold = ui_square
        .sub_rect((0.25, 0.25), Some((Align::Near, Align::Near)))
        .sub_rect((0.75, 0.75), None);

    let queue = ui_square
        .sub_rect((0.25, 0.75), Some((Align::Far, Align::Far)))
        .sub_rect((5.0 / 8.0, 23.0 / 24.0), Some((Align::Center, Align::Near)));

    let score = ui_square
        .sub_rect((0.25, 11.0 / 16.0), Some((Align::Near, Align::Far)))
        .sub_rect((7.0 / 8.0, 8.0 / 11.0), Some((Align::Center, Align::Near)));

    canvas.set_draw_color(PLACEHOLDER);
    for sub_rect in &[matrix, up_next, hold, queue, score] {
        canvas
            .fill_rect(Rect::from(sub_rect))
            .expect("Fatal redering error");
    }

    let mut cell_ctx = CellDrawContext {
        origin: matrix.bottom_left(),
        dims: matrix.size(),
        canvas,
    };

    for (coord, cell_color) in engine.cells() {
        cell_ctx.try_draw_cell(coord, cell_color);
    }

    if let Some((cursor_cells, color)) = engine.cursor_info() {
        for coord in cursor_cells {
            cell_ctx.draw_cell(coord, color);
        }
    }

    canvas.present();
}

pub struct CellDrawContext<'canvas> {
    origin: Point2<i32>,
    dims: Vector2<u32>,
    canvas: &'canvas mut Canvas<Window>,
}

impl CellDrawContext<'_> {
    const CELL_COUNT: Vector2<u32> =
        Vector2::new(Matrix::WIDTH as u32, Matrix::HEIGHT as u32);

    fn try_draw_cell(
        &mut self,
        coord: Coordinate,
        cell_color: Option<EngineColor>,
    ) {
        let Some(cell_color) = cell_color else {
            return;
        };

        self.draw_cell(coord, cell_color)
    }

    /// NOTE: We are using a coordinate system where Y increases upwards
    ///       So we need to flip the Y coordinates to match SLD2's
    ///       internal coordinate system.
    ///       In addition, we need to scale the coordinates to fit the
    ///       size (in pixels) of the ui matrix. This is important
    fn draw_cell(&mut self, coord: Coordinate, color: EngineColor) {
        let coord = coord.to_vec().cast::<u32>().expect("Should be safe");
        let this = (coord + Vector2::new(0, 1))
            .mul_element_wise(self.dims)
            .div_element_wise(Self::CELL_COUNT);

        let next = (coord + Vector2::new(1, 0))
            .mul_element_wise(self.dims)
            .div_element_wise(Self::CELL_COUNT);

        let cell_rect = Rect::new(
            self.origin.x + this.x as i32,
            self.origin.y - this.y as i32,
            next.x - this.x,
            this.y - next.y,
        );

        self.canvas.set_draw_color(color.screen_color());
        self.canvas
            .fill_rect(cell_rect)
            .expect("Fatal redering error");
        self.canvas.set_draw_color(PLACEHOLDER);
        self.canvas
            .draw_rect(cell_rect)
            .expect("Fatal redering error");
    }
}
