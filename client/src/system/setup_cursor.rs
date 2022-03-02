use bevy::prelude::{Color, Commands, Transform};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
    shapes,
};

use crate::component::MyCursor;

pub fn setup_cursor(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::SideLength(0.),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn()
        .insert(MyCursor {})
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::from([0., 0., 0., 0.])),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform {
                translation: [0., 0., 0.].into(),
                ..Default::default()
            },
        ));
}
