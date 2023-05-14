
use bevy::{
    prelude::*, utils::HashMap, math::Vec3Swizzles, input::mouse::MouseWheel, core_pipeline::clear_color::ClearColorConfig
};
use bevy_asset_loader::prelude::*;
use rand::seq::SliceRandom;
use rand::prelude::*;
use bevy_tileset::prelude::{Tileset, TilesetPlugin, Tilesets};
use bevy_ecs_tilemap::{prelude::{TilemapRenderSettings, TilemapId, TilemapTexture}, TilemapPlugin, tiles::{TileStorage, TilePos, TileTextureIndex, TileBundle, TileColor, TileFlip}, TilemapBundle};
use noise::{Fbm, Perlin, NoiseFn};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    WorldGeneration,
    InGame
}

#[derive(Resource, Debug, Clone, Default)]
pub struct RenderedChunks {
    loaded: HashMap<IVec2, Entity>,
}

#[derive(Resource)]
pub struct WorldStorage {
    tiles: Vec<u32>,
    walls: Vec<u32>,
    width: usize, 
    height: usize,
    spawn_point: usize
}

impl WorldStorage {
    pub fn from_dimensions(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![0; width * height],
            walls: vec![0; width * height],
            width,
            height,
            spawn_point: 0,
        }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_spawn_point(&self) -> UVec2 {
        self.delinearize(self.spawn_point)
    }

    pub fn set_spawn_point(&mut self, x: u32, y: u32) {
        self.spawn_point = self.linearize(x as usize, y as usize);
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    #[inline]
    pub fn linearize(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    #[inline]
    pub fn delinearize(&self, idx: usize) -> UVec2 {
        let x = idx % self.width;
        let y = idx / self.width;
        UVec2::new(x as u32, y as u32)
    }

    #[inline]
    pub fn get_tile(&self, x: i32, y: i32) -> u32 {
        assert!(x >= 0 && y >= 0);

        self.get_tile_idx(self.linearize(x as usize, y as usize))
    }

    #[inline]
    pub fn get_tile_idx(&self, idx: usize) -> u32 {
        self.tiles[idx]
    }

    #[inline]
    pub fn set_tile(&mut self, x: i32, y: i32, tile: u32) {
        assert!(x >= 0 && y >= 0);

        self.set_tile_idx(self.linearize(x as usize, y as usize), tile);
    }

    #[inline]
    pub fn set_tile_idx(&mut self, idx: usize, tile: u32) {
        self.tiles[idx] = tile;
    }

    #[inline]
    pub fn get_wall(&self, x: i32, y: i32) -> u32 {
        assert!(x >= 0 && y >= 0);

        self.get_wall_idx(self.linearize(x as usize, y as usize))
    }

    #[inline]
    pub fn get_wall_idx(&self, idx: usize) -> u32 {
        self.walls[idx]
    }

    #[inline]
    pub fn set_wall(&mut self, x: i32, y: i32, tile: u32) {
        assert!(x >= 0 && y >= 0);

        self.set_wall_idx(self.linearize(x as usize, y as usize), tile);
    }

    #[inline]
    pub fn set_wall_idx(&mut self, idx: usize, wall: u32) {
        self.walls[idx] = wall;
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct LoadPoint {
    radius: u32,
}

impl LoadPoint {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct TileTextures {
    #[asset(path = "world_tiles.ron")]
    tileset: Handle<Tileset>,
    #[asset(path = "world_walls.ron")]
    wallset: Handle<Tileset>,
}

fn spawn_camera(mut commands: Commands, tilesets: Tilesets, world: Res<WorldStorage>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let tile_size = tileset.tile_size();
    let spawn_point = world.get_spawn_point();
    camera_bundle.transform.translation.x = spawn_point.x as f32 * tile_size.x;
    camera_bundle.transform.translation.y = spawn_point.y as f32 * tile_size.y;

    commands.spawn((
        camera_bundle,
        LoadPoint::new(4),
    ));
}

fn announce_updated_chunks(rendered_chunks: Res<RenderedChunks>) {
    if !rendered_chunks.is_changed() { return; }
    info!("{} chunks", rendered_chunks.loaded.values().count());
}

fn despawn_chunks(
    mut commands: Commands,
    tilesets: Tilesets,
    camera_query: Query<(&Transform, &LoadPoint), With<Camera>>,
    chunks_query: Query<(Entity, &Transform), With<TileStorage>>,
    mut rendered_chunks: ResMut<RenderedChunks>
) {
    const CHUNK_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let (camera_transform, load_point) = camera_query.single();
    for (chunk_entity, chunk_transform) in chunks_query.iter() {
        if camera_transform.translation.xy().distance(chunk_transform.translation.xy()) > (load_point.radius * CHUNK_SIZE.x) as f32 * tileset.tile_size().x * 2.0 {
            info!("despawning chunk!");
            let chunk_pos = camera_pos_to_chunk_pos(chunk_transform.translation.xy(), tileset.tile_size());
            rendered_chunks.loaded.remove(&chunk_pos);
            commands.entity(chunk_entity).despawn_recursive();
        }
    }
}

pub fn spawn_chunks(
    mut commands: Commands,
    tilesets: Tilesets,
    world_storage: Res<WorldStorage>,
    camera_query: Query<(&Transform, &LoadPoint), With<Camera>>,
    mut rendered_chunks: ResMut<RenderedChunks>
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let (transform, load_point) = camera_query.single();
    let camera_chunk_pos = camera_pos_to_chunk_pos(transform.translation.xy(), tileset.tile_size());

    // loop through camera visible chunks (?)
    for y in (camera_chunk_pos.y - load_point.radius as i32)..(camera_chunk_pos.y + load_point.radius as i32) {
        for x in (camera_chunk_pos.x - load_point.radius as i32)..(camera_chunk_pos.x + load_point.radius as i32) {
            let chunk_pos = IVec2::new(x, y);
            if rendered_chunks.loaded.contains_key(&chunk_pos) { continue; }
            info!("spawning chunk!");
            let chunk = spawn_chunk(
                &mut commands, // revert this if any issues!
                |x, y| world_storage.in_bounds(x, y),
                |x, y| world_storage.get_tile(x, y),
                tileset,
                chunk_pos,
                11.0,
            );
            rendered_chunks.loaded.insert(chunk_pos, chunk);
        }
    }
}

fn spawn_chunk<F, V>(
    commands: &mut Commands,
    in_bounds: F,
    get_content: V,
    tileset: &Tileset,
    chunk_pos: IVec2,
    chunk_z: f32,
) -> Entity
where
    F: Fn(i32, i32) -> bool,
    V: Fn(i32, i32) -> u32,
{
    const CHUNK_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let tile_size = tileset.tile_size();
    let chunk_transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * tile_size.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * tile_size.y,
        chunk_z,
    ));

    let tileset_handle = tileset.texture();

    commands
        .entity(tilemap_entity)
        .with_children(|builder| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let tile_pos = TilePos { x, y };

                    let tile_pos_x = chunk_pos.x * CHUNK_SIZE.x as i32 + tile_pos.x as i32;
                    let tile_pos_y = chunk_pos.y * CHUNK_SIZE.y as i32 + tile_pos.y as i32;

                    let tile_index = if !in_bounds(tile_pos_x, tile_pos_y) {
                        0
                    } else {
                        get_content(tile_pos_x, tile_pos_y)
                    };

                    let mut rng = thread_rng();
                    let tile_entity = builder
                        .spawn(TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(tile_index),
                            color: TileColor(Color::hsl(0.0, 0.0, rng.gen_range(0.85..1.0))),
                            tilemap_id: TilemapId(builder.parent_entity()),
                            flip: TileFlip { x: rng.gen_bool(0.5), y: rng.gen_bool(0.5), d: false },
                            ..default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(TilemapBundle {
            grid_size: tile_size.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(tileset_handle.clone()),
            tile_size: tile_size.into(),
            transform: chunk_transform,
            ..default()
        })
        .id()
}

fn camera_pos_to_chunk_pos(camera_pos: Vec2, tile_size: Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let tile_size = tile_size.as_ivec2();
    const CHUNK_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
    const I_CHUNK_SIZE: IVec2 = IVec2 {
        x: CHUNK_SIZE.x as i32,
        y: CHUNK_SIZE.y as i32,
    };
    camera_pos / (I_CHUNK_SIZE * tile_size)
}

fn generate(mut commands: Commands, tilesets: Tilesets) {
    let _tileset = tilesets.get_by_name("world_tiles").unwrap();
    let mut world = WorldStorage::from_dimensions(1024, 256);
    let mut rng = thread_rng();
    let fbm = Fbm::<Perlin>::new(0);

    // dirt
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let idx = world.linearize(x, y);
            world.set_tile_idx(idx, 2);
        }
    }

    // surface n grass
    for x in 0..world.get_width() {
        let val = fbm.get([x as f64 / 24.0, 0.0, 0.0]) * 32.0 + world.get_height() as f64 - 30.0;
        world.set_tile(x as i32, val as i32, 3);

        for y in (val as usize + 1)..world.get_height() {
            world.set_tile(x as i32, y as i32, 0);
        }
    }

    // stone
    for x in 0..world.get_width() {
        let val = (x as f32 * 0.4).sin() * 1.6 + world.get_height() as f32 - 50.0;

        for y in (0..(val as i32)).rev() {
            if y < val as i32 - 5 {
                world.set_tile(x as i32, y as i32, 4);
                continue;
            }

            let block = if rng.gen_bool(0.5) { 2 } else { 4 };
            world.set_tile(x as i32, y as i32, block);
        }
    }

    // caves
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            if world.get_tile(x as i32, y as i32) != 4 { continue; }
            let val = fbm.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
            if val < -0.1 {
                world.set_tile(x as i32, y as i32, 0);
            }
        }
    }

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}

fn camera_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    // translate camera
    let (mut transform, mut projection) = q.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 5.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += 5.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::E) {
        projection.scale = 1.0;
    }

    // scroll zoom camera
    for ev in scroll_evr.iter() {
        projection.scale -= ev.y * 0.1;
    }
}

pub fn app() -> App {
    let mut app = App::new();
    let splashes = ["with a new coat of rust", "somewhat safe code", "will probably perform better"];
    let splash = splashes.choose(&mut rand::thread_rng()).unwrap();
    
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: format!("astatine.rs, pre-pre-pre-alpha, {}", splash.to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_state::<GameState>();
    app.add_loading_state(LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::WorldGeneration));

    // world
    const CHUNK_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
    app.insert_resource(TilemapRenderSettings {
        render_chunk_size: UVec2 {
            x: CHUNK_SIZE.x * 2,
            y: CHUNK_SIZE.y * 2,
        },
        ..default()
    });
    app.add_plugin(TilemapPlugin);
    app.add_plugin(TilesetPlugin::default());
    app.add_collection_to_loading_state::<_, TileTextures>(GameState::AssetLoading);
    app.init_resource::<RenderedChunks>();
    app.add_system(generate.in_schedule(OnEnter(GameState::WorldGeneration)));
    app.add_system(spawn_camera.in_schedule(OnEnter(GameState::InGame)));
    app.add_systems(
        (
            despawn_chunks,
            spawn_chunks,
            camera_input,
            announce_updated_chunks
        )
            .in_set(OnUpdate(GameState::InGame)),
    );

    app
}