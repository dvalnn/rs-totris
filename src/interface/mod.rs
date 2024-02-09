mod render_traits;
mod sub_rect;

use std::isize;

use cgmath::{ElementWise, EuclideanSpace, Point2, Vector2};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

use crate::{
    engine::{Color as EngineColor, Coordinate, Engine, Matrix, Offset},
    game::{DeltaTime, Game, Input, InputAction, KeyAction},
};

use self::{
    render_traits::ScreenColor,
    sub_rect::{Align, SubRect},
};

pub use crate::engine::{MoveKind, RotateKind};

const WINDOW_INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const GRID_COLOR: Color = Color::WHITE;
const PLACEHOLDER_COLOR: Color = Color::RGB(0x66, 0x77, 0x77);

impl TryFrom<Keycode> for Input {
    type Error = ();
    fn try_from(key: Keycode) -> Result<Self, Self::Error> {
        use Input::*;
        Ok(match key {
            Keycode::Right => Move(MoveKind::Right),
            Keycode::Left => Move(MoveKind::Left),
            Keycode::Down => SoftDrop,
            Keycode::Space => HardDrop,
            Keycode::Z => Rotate(RotateKind::CounterClockwise),
            Keycode::X => Rotate(RotateKind::Clockwise),
            Keycode::C => Hold,
            Keycode::Escape => todo!("Pause"),

            _ => return Err(()),
        })
    }
}

pub fn run(game: Game) {
    let sdl = sdl2::init().expect("SDL2 initialization failed");

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

    game_loop(events, game, canvas);
}

fn game_loop(
    mut events: sdl2::EventPump,
    mut game: Game,
    mut canvas: Canvas<Window>,
) {
    let mut delta_time = DeltaTime::new();

    // delta_time -> inputs -> game_logic -> rendering
    loop {
        delta_time.update();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,

                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    if let Ok(input) = Input::try_from(key) {
                        game.handle_input(dbg!(InputAction::new(
                            input,
                            KeyAction::Press,
                        )));
                    };
                }

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Ok(input) = Input::try_from(key) {
                        game.handle_input(dbg!(InputAction::new(
                            input,
                            KeyAction::Release,
                        )));
                    };
                }

                _ => {}
            }
        }

        game.update(delta_time);
        draw(&mut canvas, &game.engine);
        // println!("FPS: {}", delta_time.fps());
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

    // NOTE: UI drawing
    canvas.set_draw_color(PLACEHOLDER_COLOR);
    for sub_rect in &[matrix, up_next, hold, queue, score] {
        canvas
            .fill_rect(Rect::from(sub_rect))
            .expect("Fatal redering error");
    }

    //NOTE: Matrix content rendering
    {
        let mut cell_ctx = CellDrawContext {
            origin: matrix.bottom_left(),
            dims: matrix.size(),
            canvas,
        };

        for (coord, cell_color) in engine.cells() {
            cell_ctx.try_draw_cell(coord, cell_color, true)
        }

        if let Some((cursor_cells, color, _, _)) = engine.cursor_info() {
            for coord in cursor_cells {
                cell_ctx.draw_cell(coord, color.screen_color(), false);
                cell_ctx.draw_cell(coord, GRID_COLOR, true);
            }
        }
    }

    //NOTE: Hold piece rendering
    {
        if let Some((coords, color)) = engine.held_cursor_info() {
            todo!("draw held piece")
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
        draw_wire_frame: bool,
    ) {
        if let Some(cell_color) = cell_color {
            self.draw_cell(coord, cell_color.screen_color(), false);
        };

        if draw_wire_frame {
            self.draw_cell(coord, GRID_COLOR, true);
        }
    }

    /// NOTE: We are using a coordinate system where Y increases upwards
    ///       So we need to flip the Y coordinates to match SLD2's
    ///       internal coordinate system.
    ///       In addition, we need to scale the coordinates to fit the
    ///       size (in pixels) of the ui matrix. This is important
    fn draw_cell(&mut self, coord: Coordinate, color: Color, wire_frame: bool) {
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

        self.canvas.set_draw_color(color);

        if wire_frame {
            self.canvas
                .draw_rect(cell_rect)
                .expect("Fatal redering error");
        } else {
            self.canvas
                .fill_rect(cell_rect)
                .expect("Fatal redering error");
        }
    }
}
