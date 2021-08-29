use bevy::prelude::*;

pub struct GameOverlayPlugin;

impl Plugin for GameOverlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_score_ui.system());
    }
}

pub struct Score(i32);

fn setup(mut c: Commands, asset_server: Res<AssetServer>) {
    c.spawn_bundle(UiCameraBundle::default());

    c.spawn_bundle(TextBundle {
        style: Default::default(),
        text: Text::with_section(
            "Score: 0",
            TextStyle {
                font: (asset_server.load("Roboto-Regular.ttf")),
                font_size: 50.0,
                color: Default::default(),
            },
            Default::default(),
        ),
        // transform: Default::default(),
        ..Default::default()
    })
    .insert(Score(0));
}

fn update_score_ui(mut q: Query<(&Score, &mut TextSection)>) {
    for (score, mut text) in q.iter_mut() {
        text.value = format!("Score: {}", score.0);
    }
}
