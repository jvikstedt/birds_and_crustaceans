use bevy::{
    input::Input,
    prelude::{Camera, GlobalTransform, MouseButton, Query, Res, ResMut, With},
    render::camera::CameraPlugin,
    window::Windows,
};
use glam::Vec2;

use crate::resource::MouseInfo;

pub fn update_mouse_info(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_info: ResMut<MouseInfo>,
    buttons: Res<Input<MouseButton>>,
) {
    if let Some((camera, camera_transform)) = q_camera.iter().find(|(c, _)| match &c.name {
        Some(name) => name == &CameraPlugin::CAMERA_2D.to_string(),
        None => false,
    }) {
        let wnd = wnds.get(camera.window).unwrap();

        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();

            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            let world_pos: Vec2 = world_pos.truncate();

            let x: i16 = world_pos.x as i16;
            let y: i16 = world_pos.y as i16;

            mouse_info.x = x;
            mouse_info.y = y;
        }
    }

    if buttons.just_released(MouseButton::Left) {
        mouse_info.clicked = true;
    }
}
