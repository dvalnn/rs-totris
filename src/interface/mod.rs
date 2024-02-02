mod render_traits;
mod sub_rect;

use cgmath::{ElementWise, EuclideanSpace, Vector2};
use sdl2::{
    event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::engine::{Engine, LockTick, Matrix, Tick};

use self::{
    render_traits::ScreenColor,
    sub_rect::{Align, SubRect},
};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const PLACEHOLDER: Color = Color::RGB(0x66, 0x77, 0x77);

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

    let mut canvas = {
        let video =
            sdl.video().expect("SDL2 video subsystem aquisition failed");

        let window = video
            .window("rs-totris", INIT_SIZE.x, INIT_SIZE.y)
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

    let mut events = sdl.event_pump().expect("Event pump aquisition failed");

    loop {
        for event in events.poll_iter() {
            #[allow(clippy::single_match)]
            match event {
                Event::Quit { .. } => return,
                Event::User { .. }
                    if event.as_user_event_type::<Tick>().is_some() =>
                {
                    todo!();
                }
                Event::User { .. }
                    if event.as_user_event_type::<LockTick>().is_some() =>
                {
                    todo!();
                }

                _ => {}
            }
        }

        draw(&mut canvas, &engine)
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

    // canvas.draw_rect(ui_square).expect("Fatal redering error");
    canvas.set_draw_color(PLACEHOLDER);

    for sub_rect in &[matrix, up_next, hold, queue, score] {
        canvas
            .fill_rect(Rect::from(sub_rect))
            .expect("Fatal redering error");
    }

    let origin = matrix.bottom_left();
    let matrix_dims = matrix.size();
    let matrix_cells =
        Vector2::new(Matrix::WIDTH as u32, Matrix::HEIGHT as u32);

    // NOTE: We are using a coordinate system where Y increases upwards
    //       So we need to flip the Y coordinates to match SLD2's
    //       internal coordinate system.
    //       In addition, we need to scale the coordinates to fit the
    //       size (in pixels) of the ui matrix. This is important
    for (coord, cell_color) in engine.cells() {
        let Some(cell_color) = cell_color else {
            continue;
        };

        let coord = coord.to_vec().cast::<u32>().expect("Should be safe");
        let this = (coord + Vector2::new(0, 1))
            .mul_element_wise(matrix_dims)
            .div_element_wise(matrix_cells);

        let next = (coord + Vector2::new(1, 0))
            .mul_element_wise(matrix_dims)
            .div_element_wise(matrix_cells);

        let cell_rect = Rect::new(
            origin.x + this.x as i32,
            origin.y - this.y as i32,
            next.x - this.x,
            this.y - next.y,
        );

        canvas.set_draw_color(cell_color.screen_color());
        canvas.fill_rect(cell_rect).expect("Fatal redering error");
        canvas.set_draw_color(PLACEHOLDER);
        canvas.draw_rect(cell_rect).expect("Fatal redering error");
    }

    canvas.present();
}
