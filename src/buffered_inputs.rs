use bevy::prelude::*;

#[derive(Component)]
pub struct BufferedInput {
    pub timer: Timer,
    pub pressed: bool,
}

pub fn update_buffered_inputs(
    timer: Res<Time>,
    mut buffered_inputs: Query<(Entity, &mut BufferedInput)>,
    mut commands: Commands,
) {
    for (entity, mut buffered_input) in buffered_inputs.iter_mut() {
        buffered_input.timer.tick(timer.delta());

        if buffered_input.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
