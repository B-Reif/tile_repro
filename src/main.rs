use bevy::{prelude::*, window::WindowMode};
use bevy_ecs_tilemap::prelude::*;
mod camera;

fn layer_settings() -> LayerSettings {
    let map_size = UVec2::new(2, 2);
    let chunk_size = UVec2::new(16, 9);
    let tile_size = Vec2::new(16., 16.);
    let texture_size = Vec2::new(192.0, 16.0);
    let mut ls = LayerSettings::new(map_size, chunk_size, tile_size, texture_size);
    ls.cull = false;
    ls
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let entity = commands.spawn().id();

    let texture_handle = asset_server.load("textures/tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component.
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Create a new layer builder with a layer entity.
    let settings = layer_settings();
    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, settings, 0u16, 0u16, None);
    layer_builder.set_all(TileBundle::default());
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);
    commands
        .entity(entity)
        .insert(map)
        .insert(Transform::default())
        .insert(GlobalTransform::default());
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Tile repro".to_string(),
            width: (32. * 16.),
            height: (18. * 16.),
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(camera::setup.system())
        .add_startup_system(startup.system())
        .add_system(camera::movement.system())
        .run()
}
