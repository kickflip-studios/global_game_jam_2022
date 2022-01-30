use crate::constants::*;
use bevy::{
	core::FixedTimestep,
	prelude::*,
	render::texture::ImageType,
	sprite::collide_aabb::{collide, Collision},
};

pub fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
	let mut text = query.single_mut();
	text.sections[1].value = format!("{}", scoreboard.score);
}

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(TextBundle {
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
				},
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

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.insert_resource(Scoreboard { score: 0 })
			.add_startup_system(spawn_scoreboard)
			.add_system(scoreboard_system);
	}
}
