use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            ;
    }
}

fn start_audio(mut commands: Commands, audio: Res<Audio>) {

}

fn control_flying_sound(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {

}
