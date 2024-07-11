use bevy::prelude::*;
mod plugins;
use plugins::asset_loader::AssetLoaderPlugin;
use plugins::asteroid::AesteroidPlugin;
use plugins::spaceship::SpaceShipPlugin;
use plugins::movement::MovementPlugin;
use plugins::debug::DebugPlugin;
use plugins::camera::CameraPlugin;


fn main() {
    App::new()
    //#NOTE: Adding a pure color background
    .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
    /*#NOTE: 
    Adding white light
    If you are running 0.13 and noticing that your model does not load properly (aka its completely black), this is due to the extent of brightness of AmbientLight being lowered in 0.12->0.13
    To fix this: just increase brightness to something really high (300. for example).
    */
    .insert_resource(AmbientLight{
        color: Color::default(),
        brightness: 300.0
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(SpaceShipPlugin)
    .add_plugins(AesteroidPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .run();
}