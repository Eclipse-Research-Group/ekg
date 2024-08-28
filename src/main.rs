use nalgebra::ComplexField;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    has_gps_fix: bool,
    is_clipping: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Frame {
    timestamp: Option<i64>,
    sample_rate: f32,
    metadata: FrameMetadata,
    latitude: f32,
    longitude: f32,
    elevation: f32,
    speed: f32,
    angle: f32,
    fix: u16,
    data: Vec<i16>,
}

fn draw_frame(mut d: RaylibDrawHandle, frame: Frame, origin: nalgebra::Vector2<i32>, size: nalgebra::Vector2<i32>) {
    // d.draw_rectangle(origin.x, origin.y, size.x, size.y, Color::BLACK);


    let parts_per_pixel = frame.data.len() as f32 / size.x as f32;

    let mut i: i32 = 0;

    let mut last_position = nalgebra::Vector2::new(0, 0);

    while i < size.x {


        let frame_index = (i as f32 * parts_per_pixel).ceil() as usize;
        let value = frame.data[frame_index] as f32 / 1024.0;

        let x = origin.x + i as i32;
        let y = (value * (size.y as f32)) as i32 + origin.y;

        if i == 0 {
            last_position.x = x;
            last_position.y = y;
        }

        // d.draw_pixel(x, y, Color::RED);
        d.draw_line(last_position.x, last_position.y, x, y, Color::RED);

        last_position.x = x;
        last_position.y = y;

        i += 1;
    }

    d.draw_text("Heartbeat", 12, 12, 20, Color::BLACK);

    d.draw_text("+1", origin.x + size.x - 20, origin.y + size.y, 20, Color::BLACK);
    d.draw_text("0", origin.x + size.x - 20, origin.y + size.y / 2 + 5, 20, Color::BLACK);
    d.draw_text("-1", origin.x + size.x - 20, origin.y, 20, Color::BLACK);
    // d.draw_line(0, (maxY + minY) / 2, size.0, (maxY + minY) / 2, Color::GRAY);
    d.draw_rectangle(0, origin.y + size.y / 2, size.x, 2, Color::GRAY);
    d.draw_rectangle(0, origin.y , size.x, 2, Color::GRAY);
    d.draw_rectangle(0, origin.y + size.y, size.x, 2, Color::GRAY);
    

}

fn main() {
    let size = (640, 480);

    let (mut rl, thread) = raylib::init()
        .size(size.0, size.1)
        .title("Heartbeat: EKG")
        .build();

    while !rl.window_should_close() {
        let result: anyhow::Result<Frame> = || -> anyhow::Result<Frame> {
            let response = reqwest::blocking::get("http://localhost:8080/frame")?;
            let frame = response.json::<Frame>()?;
            Ok(frame)
        }();

        match result {
            Ok(frame) => {
                println!("data count: {}", frame.data.len());

                let mut d = rl.begin_drawing(&thread);
                
                let minY = 400;
                let maxY = 100;

                d.clear_background(Color::WHITE);
                // d.draw_text("Heartbeat", 12, 12, 20, Color::BLACK);

                // d.draw_text("+1", size.0 - 20, maxY, 20, Color::GRAY);
                // d.draw_text("0", size.0 - 20, (maxY + minY) / 2 + 5, 20, Color::GRAY);
                // d.draw_text("-1", size.0 - 20, minY, 20, Color::GRAY);
                // // d.draw_line(0, (maxY + minY) / 2, size.0, (maxY + minY) / 2, Color::GRAY);
                // d.draw_rectangle(0, (maxY + minY) / 2, size.0, 2, Color::GRAY);
                
                draw_frame(d, frame, nalgebra::Vector2::new(0, maxY), nalgebra::Vector2::new(640, 300));

            },
            Err(e) => {

                println!("Error: {:?}", e);
                let mut d = rl.begin_drawing(&thread);

                d.clear_background(Color::WHITE);
                d.draw_text("NO DATA", 12, 12, 100, Color::RED);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
        
    }
}