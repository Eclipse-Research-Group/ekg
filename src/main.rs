use std::sync::{Arc, Mutex};

use clap::Parser;
use nalgebra::ComplexField;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use nalgebra::Vector2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    has_gps_fix: bool,
    is_clipping: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrameResponse {
    frame: Option<Frame>,
    node_id: String,
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
    draw_rectangle(origin.x, origin.y, size.x, size.y, Color::from_hex(0xeeeeee));

    draw_rectangle(0.0, origin.y + size.y / 2.0, size.x, 2.0, BLACK);
    draw_rectangle(0.0, origin.y, size.x, 2.0, BLACK);
    draw_rectangle(0.0, origin.y + size.y, size.x, 2.0, BLACK);

    let parts_per_pixel = 1800 as f32 / size.x;
    
    let mut last_pos = Vector2::new(0.0, 0.0);

    let line_color = match frame.metadata.is_clipping {
        true => RED,
        false => BLUE,
    };

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
            line_color,
        );

        last_pos = Vector2::new(x + origin.x, val * size.y + origin.y);

    }

    draw_text("+1", origin.x + size.x - 30.0, origin.y - 10.0, 25.0, BLACK);
    draw_text("0", origin.x + size.x - 30.0, origin.y + size.y / 2.0 - 10.0, 25.0, BLACK);
    draw_text("-1", origin.x + size.x - 30.0, origin.y + size.y - 10.0, 25.0, BLACK);

    if frame.metadata.is_clipping {
        draw_text("Clipping", origin.x + 10.0, origin.y + 10.0, 25.0, RED);
    }

}

#[derive(Parser)]
struct Cli {
    pub host: Option<String>
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let args = Cli::parse();
    let endpoint = args.host.unwrap_or("http://localhost:8767/frame".to_string());
    let window_size = Vector2::<f32>::new(800.0, 600.0);

    request_new_screen_size(window_size.x, window_size.y);

    let frame_arc: Arc<Mutex<Option<FrameResponse>>> = Arc::new(Mutex::new(None));


    let frame_arc2 = frame_arc.clone();
    std::thread::spawn(move || {
        loop {
            let endpoint = endpoint.clone();
            let frame = || -> anyhow::Result<FrameResponse> { 
                let response = reqwest::blocking::get(endpoint)?;
                let frame = response.json::<FrameResponse>()?;
                Ok(frame)
            }();

            match frame {
                Ok(frame) => {
                    frame_arc2.lock().unwrap().replace(frame);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    frame_arc2.lock().unwrap().take();
                }
            };

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    loop {


        clear_background(WHITE);

        draw_text("EKG for heartbeat-acquisition v2", 10.0, 20.0, 30.0, BLACK);

        match frame_arc.lock().unwrap().as_ref() {
            Some(frame_response) => {

                draw_text(format!("Node ID: {}", frame_response.node_id).as_str(), 10.0, 40.0, 30.0, BLACK);

                if let Some(frame) = &frame_response.frame {

                    draw_frame(frame.clone(), Vector2::new(0.0, macroquad::window::screen_height() * 0.22), Vector2::new(macroquad::window::screen_width(), macroquad::window::screen_height() * 0.75));

                    match frame.timestamp {
                        Some(timestamp) => {
                            draw_text(format!("Timestamp: {}", timestamp).as_str(), 10.0, 60.0, 30.0, BLACK);
                        }
                        None => {
                            draw_text("No timestamp", 10.0, 40.0, 30.0, BLACK);
                        }
                    }
                    draw_text(format!("Satellites: {}", frame.fix).as_str(), 10.0, 80.0, 30.0, BLACK);

                    match frame.metadata.has_gps_fix {
                        true => {
                            draw_text("GPS Lock", 10.0, 100.0, 30.0, GREEN);
                        }
                        false => {
                            draw_text("No GPS", 10.0, 100.0, 30.0, RED);
                        }
                    }
                } else {
                    draw_text("No frame", 10.0, 100.0, 100.0, RED);
                }

            }
            None => {
                draw_text("No data", 10.0, 100.0, 100.0, RED);
            }
        }

        next_frame().await;

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
}