use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;
mod board;
use board::*;
mod ui;
use ui::*;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Castling is Cheating".to_string(),
            width: 1200.,
            height: 1000.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<PickingCamera>()
        .add_plugin(PickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup_stuff)
        .run();
}

fn setup_stuff(
    mut commands: Commands,
) {
    commands
        //Camera
        .spawn_bundle( PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 17.5, 4.0),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        // Lights
        .commands()
        .spawn_bundle( PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 6.0, 4.0)),
            ..Default::default()
        });
}