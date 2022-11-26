extern crate sdl2;
extern crate rand;

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::prelude::*;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn put_pixel(x: u32, y: u32, color: Color, framedata: &mut Vec<u8>) {
    if x > 0 && y > 0 && x < WIDTH && y < HEIGHT {
        framedata[((x + y * WIDTH)*4 + 0) as usize] = color.b;
        framedata[((x + y * WIDTH)*4 + 1) as usize] = color.g;
        framedata[((x + y * WIDTH)*4 + 2) as usize] = color.r;
        framedata[((x + y * WIDTH)*4 + 3) as usize] = color.a;
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Particle {
    x: u32,
    y: u32,
    moving: bool,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Diffusion Limited Aggregation", WIDTH, HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator.create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT).unwrap();
    let mut framedata: Vec<u8> = vec![0; ((WIDTH*HEIGHT)*4) as usize];

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut particles: Vec<Particle> = vec![];

    // starting particle 
    particles.push(Particle{
        x: WIDTH / 2,
        y: HEIGHT / 2,
        moving: false,
    });

    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        particles.push(Particle{
            x: rng.gen_range(0..WIDTH),
            y: rng.gen_range(0..HEIGHT),
            moving: true,
        });
    }
            

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }


        // update moving
        for i in &mut particles {
            // I would love to use some lovely higher order functions here but I think I just
            // don't quite understand exactly how to do this properly
            if i.moving && i.x > 0 && i.y > 0 && i.x < WIDTH && i.y < HEIGHT  {
                let dir = rng.gen_range(0..4);
                match dir {
                    0 => { i.x -= 1 },
                    1 => { i.x += 1 },
                    2 => { i.y -= 1 },
                    3 => { i.y += 1 },
                    _ => (),
                }
            }
        }

        // draw all static pixels
        particles.iter()
        //    .filter(|p| !p.moving)
            .for_each(|p| put_pixel(p.x, p.y, Color::WHITE, &mut framedata));
        

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update failed");
        canvas.copy(&framebuffer, None, None).expect("Failed to copy framebuffer to canvas");
        canvas.present();
    }
}
