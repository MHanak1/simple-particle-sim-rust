use half::f16;
//use matrix::prelude::*;


#[derive(Debug, Copy, Clone)]
struct ParticleType {
    id: u32,
    color: [u8; 4], // red green blue and alpha each 1 byte. i'd love to spell it colour but well for some reason i am making this code internationally readable so color it is
    state: u8, // 0 for solid, 1 for powdery, 2 for fluid, 3 for gas
    density: f16, // let's assume grams/cm^3
    melting_temperature: u16, // in Kelvin
    boiling_temperature: u16, // also Kelvin
//    ignition_temperature: u16, // you know the drill, but also no way to turn this off for now
//    ignition_energy: u16, // how much energy will the particle emit over it burning
//    burn_damage_per_second: u16, //this dictates how fast the particle will burn
//    max_durability: u16, // how strong the particle is, this includes burning
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    particle_type: ParticleType,
    temperature: u16,
//    durability: u16
}


fn main() {
    const SIMULATION_SIZE_X: usize = 10;
    const SIMULATION_SIZE_Y: usize = 15;

    let particle_names = [
        "Air", //0
    ];

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

    let mut particles = [[Particle {particle_type: air_particle, temperature: 300}; SIMULATION_SIZE_Y]; SIMULATION_SIZE_X];
    
    for x in 0..SIMULATION_SIZE_X{
        for y in 0..SIMULATION_SIZE_Y{
            println!("{}", particle_names[particles[x][y].particle_type.id as usize])
        }
    }

    //let mut particles = Conventional::new(100);
  println!("Hello, world!");
}
