use std::sync::{Arc, Mutex};

use nalgebra::ComplexField;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use nalgebra::Vector2;

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

fn draw_frame(frame: Frame, origin: nalgebra::Vector2<f32>, size: nalgebra::Vector2<f32>) {
    draw_rectangle(0.0, origin.y + size.y / 2.0, size.x, 2.0, BLACK);
    draw_rectangle(0.0, origin.y, size.x, 2.0, BLACK);
    draw_rectangle(0.0, origin.y + size.y, size.x, 2.0, BLACK);

    let parts_per_pixel = 1800 as f32 / size.x;
    
    let mut last_pos = Vector2::new(0.0, 0.0);

    for i in 0..1800 {
        let x = i as f32 / parts_per_pixel;
        let val = frame.data[i] as f32 / 1024.0;

        if i == 0 {
            last_pos = Vector2::new(origin.x + x, val * size.y + origin.y);
        }

        draw_line(
            last_pos.x,
            last_pos.y,
            x + origin.x,
            val * size.y + origin.y,
            2.0,
            BLUE,
        );

        last_pos = Vector2::new(x + origin.x, val * size.y + origin.y);

    }

    draw_text("+1", origin.x + size.x - 30.0, origin.y - 10.0, 25.0, BLACK);
    draw_text("0", origin.x + size.x - 30.0, origin.y + size.y / 2.0 - 10.0, 25.0, BLACK);
    draw_text("-1", origin.x + size.x - 30.0, origin.y + size.y - 10.0, 25.0, BLACK);


}

#[macroquad::main("BasicShapes")]
async fn main() {

    let window_size = Vector2::<f32>::new(800.0, 600.0);

    request_new_screen_size(window_size.x, window_size.y);

    let frame_arc: Arc<Mutex<Option<Frame>>> = Arc::new(Mutex::new(None));


    let frame_arc2 = frame_arc.clone();
    std::thread::spawn(move || {
        loop {
            let frame = || -> anyhow::Result<Frame> { 
                let response = reqwest::blocking::get("http://localhost:8080/frame")?;
                let frame = response.json::<Frame>()?;
                Ok(frame)
            }();

            let frame = match frame {
                Ok(frame) => frame,
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            };

            frame_arc2.lock().unwrap().replace(frame);
        }
    });

    loop {


        clear_background(WHITE);

        draw_text("EKG for heartbeat-acquisition v2", 10.0, 20.0, 30.0, BLACK);

        match frame_arc.lock().unwrap().as_ref() {
            Some(frame) => {
                match frame.timestamp {
                    Some(timestamp) => {
                        draw_text(format!("Timestamp: {}", timestamp).as_str(), 10.0, 40.0, 30.0, BLACK);
                    }
                    None => {
                        draw_text("No timestamp", 10.0, 40.0, 30.0, BLACK);
                    }
                }
                draw_text(format!("Satellites: {}", frame.fix).as_str(), 10.0, 60.0, 30.0, BLACK);
                draw_frame(frame.clone(), Vector2::new(0.0, 100.0), Vector2::new(macroquad::window::screen_width(), macroquad::window::screen_height() * 0.75));
            }
            None => {
                draw_text("No frame received", 10.0, 100.0, 30.0, BLACK);
            }
        }

        next_frame().await;

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
}