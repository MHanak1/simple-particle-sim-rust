use std::num::NonZeroU32;
use std::rc::Rc;
use winit::{
    event::{Event, WindowEvent, StartCause, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::{Duration, Instant};
use simple_particle_sim::particle_sim::*;
use simple_particle_sim::texture;

    
    // define particle types (there is probably a better way to do it but i don't care)
    static air_particle: ParticleType = ParticleType {
        id: 0,
        vapor_color: [0, 0, 0, 0],
        liquid_color: [50, 50, 150, 100],
        solid_color: [200, 200, 255, 255],
        solid: true,
        gas_density: 0.001225,
        liquid_density: 0.85,
        melting_temperature: 60,
        boiling_temperature: 79,
        heat_capacity: 700,
        heat_resistance: 50,
    };


    static sand_particle: ParticleType = ParticleType {
        id: 1,
        vapor_color: [194, 178, 128, 255],
        liquid_color: [194, 178, 128, 255],
        solid_color: [194, 178, 128, 255],
        solid: false,
        liquid_density: 10.52,
        gas_density: 1.0,
        melting_temperature: 2000,
        boiling_temperature: 5000,
        heat_capacity: 835,
        heat_resistance: 5,
    };

    static steel_particle: ParticleType = ParticleType {
        id: 2,
        vapor_color: [113, 121, 126, 255],
        liquid_color: [113, 121, 126, 255],
        solid_color: [113, 121, 126, 255],
        solid: true,
        liquid_density: 7.85,
        gas_density: 1.0,
        melting_temperature: 1640,
        boiling_temperature: 29000,
        heat_capacity: 420,
        heat_resistance: 2,
    };

    static water_particle: ParticleType = ParticleType {
        id: 3,
        vapor_color: [35,137,218, 55],
        liquid_color: [35,137,218, 105],
        solid_color: [35,137,218, 255],
        solid: true,
        gas_density: 0.6,
        liquid_density: 1.0,
        melting_temperature: 273,
        boiling_temperature: 373,
        heat_capacity: 4184,
        heat_resistance: 2,
    };

    const ceramics_particle: ParticleType = ParticleType {
        id: 4,
        vapor_color: [80,80,80, 255],
        liquid_color: [80,80,80, 255],
        solid_color: [80,80,80, 255],
        solid: true,
        gas_density: 0.6,
        liquid_density: 1.0,
        melting_temperature: 4000,
        boiling_temperature: 50000, //simply disabled
        heat_capacity: 850,
        heat_resistance: 3,
    };

    // init constaints
    const SIMULATION_SIZE_X: u32 = 225;
    const SIMULATION_SIZE_Y: u32 = 120;
    const SIMULATION_RATE: u32 = 60; //in Hz, iterations per second
    const BACKGROUND_COLOR: [u8; 3] = [20, 20, 20];
    const PIXEL_SIZE: u32 = 6;


fn main() {
    let mut time = 0;

    // list of all particle names
    let _particle_names = [
        "Air",      //0
        "Sand",     //1
    ];
    
    // initialise the sandbox
    let mut particles = ParticleSim::new(
        SIMULATION_SIZE_X as usize,
        SIMULATION_SIZE_Y as usize,
        Particle::new(air_particle).set_temperature(500),

//        Particle {
//            particle_type: air_particle,
//            energy: air_particle.heat_capacity * 300,
//            color_noise: rand::random::<u8>() / 8 + 128,
//        }
    );
    
    for x in 0..SIMULATION_SIZE_X{
        for y in 0..SIMULATION_SIZE_Y{
            //println!("{}", particle_names[particles[x][y].particle_type.id as usize])
            if (x >= 25 && x < 75) && (y >= 50 && y < 100){
                if x > 100{
                    particles.set_particle(
                        x as usize,
                        y as usize, 
                        Particle::new(water_particle).set_temperature(270)
                   )
                } else {
                    particles.set_particle(
                        x as usize, 
                        y as usize, 
                        //Particle::new(sand_particle).set_temperature(290).set_noise_value(texture::random(8))
                        Particle::new(steel_particle).set_temperature(300).set_noise_value(texture::metal(6, 20, x, y))
                   )
               }
            }
        }
    }
/*
    for x in 0..99{
        particles.set_particle(
            x as usize,
            50, 
            Particle::new(ceramics_particle).s2et_temperature(280).set_noise_value(texture::random(8))
       )
    }

    
    for x in 20..81{
        if x < 100{
            particles.set_particle(
                x as usize,
                80, 
                Particle::new(ceramics_particle).set_temperature(280).set_noise_value(texture::random(8))
           )
        }
    }

    for y in 60..85{
        if y < 100{
            particles.set_particle(
                20,
                y, 
                Particle::new(ceramics_particle).set_temperature(280).set_noise_value(texture::random(8))
           );
            particles.set_particle(
                80,
                y, 
                Particle::new(ceramics_particle).set_temperature(280).set_noise_value(texture::random(8))
           )
        }
    }

    for y in 80..99{
        if y < 100{
            particles.set_particle(
                40,
                y, 
                Particle::new(ceramics_particle).set_temperature(280).set_noise_value(texture::random(8))
           );
            particles.set_particle(
                60,
                y, 
                Particle::new(ceramics_particle).set_temperature(280).set_noise_value(texture::random(8))
           )
        }
    }
*/


    // initialise window
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    
    let mut mouse_clicked = false;
    let mut mouse_pos: [u32; 2] = [0, 0];

    let mut finished_simulating = true;

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
            
            WindowEvent::MouseInput {
                button,
                state,
                ..
            } => {
                match button {
                    MouseButton::Left => {
                        mouse_clicked = state.is_pressed();
                    },
                    _ => {}
                }
            },

            WindowEvent::CursorMoved {
                position,
                ..
            }=> {
                mouse_pos = [position.x as u32, position.y as u32];
                handle_mouse(mouse_pos[0], mouse_pos[1], mouse_clicked, &mut particles);

                /*
                let mut x = position.x as i32 / PIXEL_SIZE as i32;
                let mut y = position.y as i32 / PIXEL_SIZE as i32;

                if particles.particle_exists(x as usize, y as usize) && mouse_clicked{
                    particles.set_particle(x as usize, y as usize, Particle::new(steel_particle).set_temperature(5000).set_noise_value(texture::metal(6, 20, x as u32, y as u32)));
                }
                
                for i in 0..3{
                    for j in 0..3{
                        x = position.x as i32 / PIXEL_SIZE as i32 + i - 1;
                         y = position.y as i32 / PIXEL_SIZE as i32 + j - 1;
                        if x < 0{x = 0}
                        if y < 0{y = 0}
                        if particles.particle_exists(x as usize, y as usize){
                            if true || particles.particle_at(x as usize, y as usize).particle_type.id == 0{
                                particles.set_particle(x as usize, y as usize, Particle::new(water_particle).set_temperature(290));
                                //particles.set_particle(x, y, Particle::new(ceramics_particle).set_temperature(5000).set_noise_value(texture::metal(6, 20, x as u32, y as u32)));
                                //particles.set_particle(x, y, Particle::new(air_particle).set_temperature(900));
                            }
                        }
                    }
                }
*/
            }

            WindowEvent::CloseRequested => {
                elwt.exit();
            },
            WindowEvent::RedrawRequested if window_id == window.id() =>{
                  


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

                let pixels = particles.render_pixels();
                let mut buffer = surface.buffer_mut().unwrap();

                for index in 0..(width * height) {
                    let y = index / width / PIXEL_SIZE;
                    let x = index % width / PIXEL_SIZE;
                   
                    if particles.particle_exists(x as usize, y as usize){
                        let rgb = pixels[x as usize + y as usize * particles.width];
                        buffer[index as usize] = (rgb[2] as u32) | ((rgb[1] as u32) << 8) | ((rgb[0] as u32) << 16);

                    } else {
                        buffer[index as usize] = (BACKGROUND_COLOR[2] as u32) | ((BACKGROUND_COLOR[1] as u32) << 8) | ((BACKGROUND_COLOR[0] as u32) << 16);
                    }
                }
/*
                for index in 0..(width * height) {
                    let y = index / height / PIXEL_SIZE;
                    let x = index % width / PIXEL_SIZE;
                   
                    if particles.particle_exists(x as usize, y as usize){
                        //println!("{}, {}, {}, {}", x, y, x + y * width, index);
                        let rgb = pixels[(x + y * SIMULATION_SIZE_X) as usize];
                        buffer[index as usize] = (rgb[2] as u32) | ((rgb[1] as u32) << 8) | ((rgb[0] as u32) << 16);

                    } else {
                        buffer[index as usize] = (BACKGROUND_COLOR[2] as u32) | ((BACKGROUND_COLOR[1] as u32) << 8) | ((BACKGROUND_COLOR[0] as u32) << 16);
                    }
                }
*/
                buffer.present().unwrap();
            },
            _ => {}
            
        },
        Event::NewEvents(cause) => {
            match cause {
                StartCause::ResumeTimeReached { .. } => {


                    println!("{}", finished_simulating);

                    
                    if finished_simulating{
                        println!("hello");

                        finished_simulating = false;

                        elwt.set_control_flow(ControlFlow::WaitUntil(
                            Instant::now().checked_add(Duration::from_millis((1000/SIMULATION_RATE).into())).unwrap(),
                        ));
 
    /*
                        for x in 0..particles.width {
                            //particles.set_particle(x, particles.height - 2, Particle::new(ceramics_particle).set_temperature(500));
                            particles.set_particle(x, 0, Particle::new(air_particle).set_temperature(300));
                            if x > 40 && x < 60{
                                particles.set_particle(x, 95, Particle::new(air_particle).set_temperature(4000));
                            }
                        }
    */

                        handle_mouse(mouse_pos[0], mouse_pos[1], mouse_clicked, &mut particles);
                        particles.simulate_heat_simplified(time);
                        //particles.simulate_gasses(time);
                        particles.simulate_liquids(time);
                        particles.simulate_sand(time);
                        window.request_redraw();
                        finished_simulating = true;

                        time += 1;
                    }
                },
                _ => {}
            }
        },
        _ => {}

        }
    });
}

fn handle_mouse(xa: u32, ya: u32, mouse_clicked: bool, particles: &mut ParticleSim){
    let x = xa as i32 / PIXEL_SIZE as i32;
    let y = ya as i32 / PIXEL_SIZE as i32;
    
    if particles.particle_exists(x as usize, y as usize) && mouse_clicked{
        particles.set_particle(x as usize, y as usize, Particle::new(steel_particle).set_temperature(5000).set_noise_value(texture::metal(6, 20, x as u32, y as u32)));
    }
}
