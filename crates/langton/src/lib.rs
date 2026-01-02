use bevy::prelude::*;
use bevy_canvas_2d::prelude::*;

const BOARD_SIZE: UVec2 = UVec2::new(1024 * 4, 1024 * 4);

pub struct LangtonPlugin;

impl Plugin for LangtonPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.init_resource::<Memory>();
        app.insert_resource(Time::<Fixed>::from_hz(64.0));

        // Systems
        app.add_systems(Startup, spawn_turmites)
            .add_systems(FixedUpdate, move_turmites);
    }
}

#[derive(Resource)]
pub struct Memory {
    pub data: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: vec![0; BOARD_SIZE.element_product() as usize],
        }
    }
}

impl Memory {
    pub fn read(&self, coord: UVec2) -> u8 {
        let index = coord.y as usize * BOARD_SIZE.x as usize + coord.x as usize;
        self.data[index]
    }

    pub fn write(&mut self, coord: UVec2, value: u8) {
        let index = coord.y as usize * BOARD_SIZE.x as usize + coord.x as usize;
        self.data[index] = value;
    }
}

#[derive(Component)]
pub struct Turmite {
    pos: UVec2,
    state: u8,
}

fn spawn_turmites(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let coord = UVec2::new(BOARD_SIZE.x / 2, BOARD_SIZE.y / 2);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(0.5))),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 0.7, 0.5))),
        Turmite { pos: coord, state: 0 }, // start facing North
        Transform::from_translation(coord_to_world_pos(coord)),
    ));
}

fn move_turmites(
    mut draw_pixel_msg: MessageWriter<DrawPixel>,
    mut memory: ResMut<Memory>,
    mut query: Query<(&mut Turmite, &mut Transform)>,
) {
    for (mut turmite, mut transform) in query.iter_mut() {
        for _ in 0..10000 {
            let coord = turmite.pos;

            let input = memory.read(coord);
            let (delta, new_state, output) = transition(turmite.state, input);

            // Move turmite
            turmite.pos = (coord.as_ivec2() + delta).rem_euclid(BOARD_SIZE.as_ivec2()).as_uvec2();
            transform.translation = coord_to_world_pos(turmite.pos);

            // Update state
            turmite.state = new_state;

            // Update memory
            memory.write(coord, output);
            draw_pixel_msg.write(DrawPixel {
                pos: coord,
                rgba_u32: state_to_colour(output),
            });
        }
    }
}

// -- Helpers --

fn coord_to_world_pos(coord: UVec2) -> Vec3 {
    (Vec2::new(coord.x as f32, coord.y as f32) + Vec2::splat(0.5)
        - Vec2::new(BOARD_SIZE.x as f32 * 0.5, BOARD_SIZE.y as f32 * 0.5))
    .extend(1.0)
}

fn transition(state: u8, input: u8) -> (IVec2, u8, u8) {
    // Example transition function for a 2-state turmite
    match (state, input) {
        (0, 0) => (IVec2::new(1, 0), 1, 1),
        (1, 0) => (IVec2::new(0, -1), 2, 1),
        (2, 0) => (IVec2::new(-1, 0), 3, 1),
        (3, 0) => (IVec2::new(0, 1), 0, 1),
        (0, 1) => (IVec2::new(-1, 0), 3, 0),
        (3, 1) => (IVec2::new(0, -1), 2, 0),
        (2, 1) => (IVec2::new(1, 0), 1, 0),
        (1, 1) => (IVec2::new(0, 1), 0, 0),
        _ => unreachable!(),
    }
}

fn state_to_colour(state: u8) -> u32 {
    match state {
        0 => 0xffffffff, // white
        1 => 0xff000000, // black
        _ => {
            let colour = Color::hsl((state as f32 * 137.508) % 360.0, 0.7, 0.5).to_srgba();
            pack_rgba8([
                (colour.red * 255.0) as u8,
                (colour.green * 255.0) as u8,
                (colour.blue * 255.0) as u8,
                255,
            ])
        }
    }
}
