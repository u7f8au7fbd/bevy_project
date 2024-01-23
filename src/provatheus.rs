use bevy::{app::PluginGroupBuilder, core::*, math::vec2, prelude::*};
use bevy_simple_tilemap::prelude::*;

pub fn enable_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}

pub fn gizmos_xy(mut gizmos: Gizmos, mut config: ResMut<GizmoConfig>, window: Query<&Window>) {
    for window in window.iter() {
        config.depth_bias = -1.0;
        config.line_width = 0.2;

        gizmos.line(
            //X軸
            Vec3::new(window.width() / 2., 0.0, 0.0),
            Vec3::new(-(window.width() / 2.), 0.0, 0.0),
            Color::RED,
        );
        gizmos.line(
            //Y軸
            Vec3::new(0.0, window.height() / 2., 0.0),
            Vec3::new(0.0, -(window.height() / 2.), 0.0),
            Color::GREEN,
        );
    }
}

fn comparison(x: f32, y: f32) -> f32 {
    if x < y {
        y
    } else {
        x
    }
}

fn roll_up(x: f32, y: f32) -> i32 {
    if x as i32 % y as i32 == 0 {
        x as i32 / y as i32
    } else {
        (x as i32 / y as i32) + 1
    }
}
fn odd_number_check(x: &mut i32) {
    if *x % 2 != 0 {
        *x += 1;
    }
}

pub fn world_grid_2d(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window: Query<&Window>,
) {
    let grid_size = 100.;
    for window in window.iter() {
        let window_size = comparison(window.width(), window.height());
        let mut field_scale = roll_up(window_size, grid_size);
        odd_number_check(&mut field_scale);
        let fix_scale = ((field_scale as f32 / 2.) * grid_size) - grid_size / 2.;
        let texture_handle = asset_server.load("./provatheus/grid.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            vec2(grid_size, grid_size),
            1,
            1,
            Some(vec2(1.0, 1.0)),
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut tiles = Vec::new();

        for y in 0..field_scale {
            for x in 0..field_scale {
                tiles.push((
                    IVec3::new(x, y, -256),
                    Some(Tile {
                        sprite_index: 0,
                        ..default()
                    }),
                ));
            }
        }
        let mut tilemap = TileMap::default();
        tilemap.set_tiles(tiles);
        let tilemap_bundle = TileMapBundle {
            tilemap,
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-fix_scale, -fix_scale, 0.),
                ..default()
            },
            ..default()
        };
        // Spawn tilemap
        commands.spawn(tilemap_bundle);
    }
}

pub struct ProvatheusPlugin;

impl PluginGroup for ProvatheusPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UpdatePlugin)
            .add(StartupPlugin)
    }
}

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enable_visible, gizmos_xy));
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_grid_2d);
    }
}
