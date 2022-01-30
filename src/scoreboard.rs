use bevy::{
    core::FixedTimestep,
    prelude::*,
	render::texture::ImageType,
    sprite::collide_aabb::{collide, Collision},
};


pub struct Scoreboard {
    score: usize,
}

pub fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", scoreboard.score);
}

pub fn spawn_scoreboard(
	mut commands: Commands,
	asset_server: Res<AssetServer>
)
{
	commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Annhilations: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load(BOLD_FONT),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load(NORMAL_FONT),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
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
		app
			.add_startup_system(player_spawn)
			.add_system(player_movement);
			// .add_system_set(
			// 	SystemSet::new()
			// 		.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
			// 		.with_system(particle_collision_system.system())
			// );
	}
}
