use bevy::{
    math::Rect,
    prelude::{AssetServer, Color, Commands, Res, TextBundle},
    text::{Text, TextSection, TextStyle},
    ui::{AlignSelf, Display, PositionType, Style, Val},
};

use crate::component::DebugWindow;

pub fn setup_debug_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                display: Display::None,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Debug: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "\nFPS: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(DebugWindow::default());
}
