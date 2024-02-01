use cgmath::Vector2;
use sdl2::{
    event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::engine::{Engine, Matrix};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const PLACEHOLDER_1: Color = Color::RGB(0x66, 0x77, 0x77);
const PLACEHOLDER_2: Color = Color::RGB(0x77, 0x88, 0x88);

pub fn run(engine: Engine) {
    let sdl = sdl2::init().expect("SDL2 initialization failed");

    let mut canvas = {
        let video = sdl
            .video()
            .expect("SDL2 video subsystem initialization failed");

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

    let mut events = sdl.event_pump().expect("Event pump creation failed");

    loop {
        for event in events.poll_iter() {
            #[allow(clippy::single_match)]
            match event {
                Event::Quit { .. } => return,
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

    let ui_square = {
        let (x, y) = canvas.viewport().size();
        let small_side = std::cmp::min(x, y);
        let margin = (x.saturating_sub(y) / 2, y.saturating_sub(x) / 2);
        Rect::new(margin.0 as i32, margin.1 as i32, small_side, small_side)
    };

    let matrix = {
        let mut matrix = ui_square;
        matrix.resize(
            ((ui_square.width() / 2) as f32 * 7.0 / 8.0) as _,
            (ui_square.height() as f32 * 7.0 / 8.0) as _,
        );
        matrix.center_on(ui_square.center());
        matrix
    };

    let up_next = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        bounding_box.resize(quarter, quarter);
        bounding_box.offset(3 * quarter as i32, 0);

        let mut rect = bounding_box;
        let inner_dim = bounding_box.width() * 3 / 4;
        rect.resize(inner_dim, inner_dim);
        rect.center_on(bounding_box.center());

        rect
    };

    let hold = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        bounding_box.resize(quarter, quarter);

        let mut rect = bounding_box;
        let inner_dim = bounding_box.width() * 3 / 4;
        rect.resize(inner_dim, inner_dim);
        rect.center_on(bounding_box.center());

        rect
    };

    let queue = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        bounding_box.resize(quarter, quarter * 3);
        bounding_box.offset(3 * quarter as i32, quarter as i32);

        let mut rect = bounding_box;
        let inner_width = bounding_box.width() * 5 / 8;
        let inner_height = bounding_box.height() * 23 / 24;
        rect.resize(inner_width, inner_height);
        rect.center_on(bounding_box.center());
        rect.set_y(bounding_box.top());

        rect
    };

    let score = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        let sixteenth = quarter / 4;
        bounding_box.resize(quarter, 2 * quarter);
        bounding_box.offset(0, 5 * sixteenth as i32);

        let mut rect = bounding_box;
        let inner_width = bounding_box.width() * 7 / 8;
        rect.set_width(inner_width);
        rect.center_on(bounding_box.center());
        rect.set_y(bounding_box.top());

        rect
    };

    // canvas.draw_rect(ui_square).expect("Fatal redering error");
    canvas.set_draw_color(PLACEHOLDER_1);
    canvas.fill_rect(matrix).expect("Fatal redering error");
    canvas.fill_rect(up_next).expect("Fatal redering error");
    canvas.fill_rect(hold).expect("Fatal redering error");
    canvas.fill_rect(queue).expect("Fatal redering error");
    canvas.fill_rect(score).expect("Fatal redering error");

    let origin = matrix.bottom_left();
    let (m_width, m_heigth) = matrix.size();

    for (coord, _cell_color) in engine.cells() {
        let coord = coord.cast::<i32>().expect("Should never fail");

        let this_x = coord.x as u32 * m_width / Matrix::WIDTH as u32;
        let this_y = (coord.y as u32 + 1) * m_heigth / Matrix::HEIGHT as u32;
        let next_x = (coord.x as u32 + 1) * m_width / Matrix::WIDTH as u32;
        let next_y = coord.y as u32 * m_heigth / Matrix::HEIGHT as u32;

        // NOTE: We are using a coordinate system where Y increases upwards
        //       So we need to flip the Y coordinates to match SLD2's
        //       internal coordinate system
        let cell_rect = Rect::new(
            origin.x + this_x as i32,
            origin.y - this_y as i32,
            next_x - this_x,
            this_y - next_y,
        );

        //TODO: use cell_color and skip drawing if it's None
        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(cell_rect).expect("Fatal redering error");
    }

    canvas.present();
}
