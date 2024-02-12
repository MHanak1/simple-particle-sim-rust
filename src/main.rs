use half::f16;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::{Duration, Instant};
use simple_particle_sim::particle_sim::*;

fn main() {
    // init constaints
    const SIMULATION_SIZE_X: u32 = 100;
    const SIMULATION_SIZE_Y: u32 = 150;
    const SIMULATION_RATE: u32 = 20; //in Hz, iterations per second
    const BACKGROUND_COLOR: [u8; 3] = [255, 255, 255];
    const PIXEL_SIZE: u32 = 6;

    // list of all particle names
    let particle_names = [
        "Air",      //0
        "Sand",     //1
    ];
    
    // define particle types (there is probably a better way to do it but i don't care)
    let air_particle = ParticleType {
        id: 0,
        color: [0, 0, 0, 0],
        state: 3,
        density: f16::from_f32(0.001225),
        melting_temperature: 60,
        boiling_temperature: 79,

    };

    let sand_particle = ParticleType {
        id: 1,
        color: [194, 178, 128, 255],
        state: 1,
        density: f16::from_f32(1.52),
        melting_temperature: 2000,
        boiling_temperature: 2500,

    };
    
    // initialise the sandbox
    let mut particles = ParticleSim::new(
        SIMULATION_SIZE_X as usize,
        SIMULATION_SIZE_Y as usize,
        Particle {
            particle_type: air_particle,
            temperature: 300,
            color_noise: rand::random::<u8>() / 8 + 128,
        }
    );
    
    for x in 0..SIMULATION_SIZE_X{
        for y in 0..SIMULATION_SIZE_Y{
            //println!("{}", particle_names[particles[x][y].particle_type.id as usize])
            if (x > 30 && x < 70) && (y > 30 && y < 70){
                particles.set_particle(x as usize, y as usize, Particle {
                    particle_type: sand_particle,
                    temperature: 2000,
                    color_noise: rand::random::<u8>() / 8 + 128,
                })
            }
        }
    }

    // initialise window
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    event_loop.set_control_flow(ControlFlow::WaitUntil(
        Instant::now().checked_add(Duration::from_millis((1000/SIMULATION_RATE).into())).unwrap(),
    ));


    // main loop
    let _ = event_loop.run(move |event, elwt| {
        match event {
        Event::WindowEvent{
            event,
            window_id,
        } => match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            WindowEvent::RedrawRequested if window_id == window.id() =>{
                  
                elwt.set_control_flow(ControlFlow::WaitUntil(
                    Instant::now().checked_add(Duration::from_millis((1000/SIMULATION_RATE).into())).unwrap(),
                ));

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) {
                    let y = index / width / PIXEL_SIZE;
                    let x = index % width / PIXEL_SIZE;
                   
                    if x < SIMULATION_SIZE_X && y < SIMULATION_SIZE_Y{
                        let rgb = particles.get_particle_color(x as usize, y as usize);
                        buffer[index as usize] = (rgb[2] as u32) | ((rgb[1] as u32) << 8) | ((rgb[0] as u32) << 16);
                    } else {
                        buffer[index as usize] = (BACKGROUND_COLOR[2] as u32) | ((BACKGROUND_COLOR[1] as u32) << 8) | ((BACKGROUND_COLOR[0] as u32) << 16);
                    }
                }

                buffer.present().unwrap();
            },
            _ => {}
            
        }
        _ => {}

        }
    });
}
