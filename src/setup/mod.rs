// pub mod asset;
mod animate;
mod font;
mod image;
pub mod sound;

use bevy::prelude::*;

use crate::component::ChineseBroadCamera;

#[derive(Resource)]
pub struct AssetLoading;

impl Plugin for AssetLoading {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (loading, font::loading, sound::loading, image::loading, animate::loading),
        )
            .add_systems(Update, image::on_window_resize);
    }
}

pub fn loading(mut commands: Commands) {
    info!("loading...");
    // 创建默认镜头
    commands.spawn((Camera2dBundle::default(), ChineseBroadCamera));
}
