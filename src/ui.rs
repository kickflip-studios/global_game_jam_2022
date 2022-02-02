use crate::constants::*;
use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::texture::ImageType,
    sprite::collide_aabb::{collide, Collision},
};

pub fn ui_system(
    scoreboard: Res<Scoreboard>,
    mut query: Query<(&mut Text)>
)
{
    let mut text = query.single_mut();


    text.sections[1].value = format!("{}", scoreboard.score);
}

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
    .spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Annhilations: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load(BOLD_FONT),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load(NORMAL_FONT),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                }
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

}


pub fn spawn_game_over_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "GAME OVER ('r' to restart)".to_string(),
                    TextStyle {
                        font: asset_server.load(BOLD_FONT),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(Scoreboard { score: 0 })
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_ui)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver)
                .with_system(spawn_game_over_text)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                .with_system(ui_system)
            );

    }
}
