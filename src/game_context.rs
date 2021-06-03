use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use ggez::audio::{Source};
use ggez::audio::SoundSource;
use crate::systems::rendering_system::RenderingSystem;
use crate::resources::input_queue::InputQueue;
use crate::systems::input_system::InputSystem;
use crate::systems::gameplay_state_system::GameplayStateSystem;
use crate::constant::{MAX_LEVEL, RESOURCE_PREFIX_PATH};
use std::cmp::min;
use std::fs;
use crate::components::{Position, Direction, Renderable, Wall, Box, Player, Spot, Movable, Blocking, Directional, FloorType, FloorMaterial, WallColor, WallShape, BoxSpotColor, BoxType};
use crate::resources::game_state::GameState;
use crate::entity_builder::EntityBuilder;
use crate::resources::sound_library::SoundLibrary;
use crate::resources::game_vars::GameVars;


pub struct GameContext {
    pub world: World,
}

impl GameContext {
    pub fn from(world: World) -> Self {
        GameContext { world }
    }

    pub fn initialize_level(&mut self, level: u8, context: &mut Context) {
        let level = min(level, MAX_LEVEL);
        let map_string= fs::read_to_string(format!("{}/maps/map_{}.txt", RESOURCE_PREFIX_PATH, level))
            .expect(&format!("Unable to read file. Check if level {} exists!", level));
        self.generate_map(map_string);

        let mut sound_lib = self.world.write_resource::<SoundLibrary>();
        sound_lib.music_sound.ingame_music = Some(Source::new(context, format!("/sounds/musics/ingame_music_{}.wav", level)).unwrap());
        sound_lib.music_sound.victory_music = Some(Source::new(context, format!("/sounds/musics/victory_music_{}.wav", level)).unwrap());

        if let Some(ref mut ingame_music) = sound_lib.music_sound.ingame_music { ingame_music.play().unwrap(); }
    }

    pub fn register_components(&mut self) {
        self.world.register::<Renderable>();
        self.world.register::<Wall>();
        self.world.register::<Player>();
        self.world.register::<Box>();
        self.world.register::<Spot>();
        self.world.register::<Movable>();
        self.world.register::<Blocking>();
        self.world.register::<Directional>();
    }

    pub fn register_resources(&mut self) {
        self.world.insert(InputQueue::default());
        self.world.insert(GameState::default());
        self.world.insert(SoundLibrary::default());
        self.world.insert(GameVars::default());
    }

    pub fn generate_map(&mut self, map_string: String) {
        let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

        for (y, &row) in rows.iter().enumerate() {
            let columns = row.split(' ').collect::<Vec<_>>();

            for (x, &column) in columns.iter().enumerate() {
                let position = Position { x: x as u8, y: y as u8, z: 0 };

                match column {
                    "." => EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand),
                    "W" => {
                        EntityBuilder::create_wall(&mut self.world, position, WallColor::Gray, WallShape::Square);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "P" => {
                        EntityBuilder::create_player(&mut self.world, position, Direction::Down);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "B" => {
                        EntityBuilder::create_box(&mut self.world, position, BoxType::Bright, BoxSpotColor::Red);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "S" => {
                        EntityBuilder::create_spot(&mut self.world, position, BoxSpotColor::Red);
                        EntityBuilder::create_floor(&mut self.world, position, FloorType::Gravel, FloorMaterial::Sand);
                    },
                    "N" => (),
                    c => panic!("Unrecognized map item {}", c)
                }
            }
        }
    }
}

impl event::EventHandler for GameContext {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut is = InputSystem::new();
        is.run_now(&self.world);
        drop(is);

        let mut gss = GameplayStateSystem::new();
        gss.run_now(&self.world);
        drop(gss);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rs = RenderingSystem::from(ctx, self);
        rs.run_now(&self.world);
        drop(rs);

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.push(keycode);
    }
}