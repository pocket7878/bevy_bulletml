#[cfg(test)]
use crate::tree::BulletMLNode;

use super::state::State;
use bevy::prelude::*;

/// Application specific BulletML runner trait.
pub trait AppRunner<D, B: Component> {
    /// Initializes the runner.
    ///
    /// This function is called when a new [Runner](struct.Runner.html) is created/reused.
    fn init(&mut self) {}
    /// Gets this bullet's direction based on application data.
    fn get_bullet_direction(&self, data: &D, bullet: &B) -> f64;
    /// Gets this bullet's aim direction based on application data.
    ///
    /// The "target" related to the "aim" notion is application specific.
    fn get_aim_direction(
        &self,
        data: &D,
        bullet_transform: &Transform,
        target_transform: &Transform,
    ) -> f64;
    /// Gets this bullet's speed based on application data.
    fn get_bullet_speed(&self, data: &D, bullet: &B) -> f64;
    /// Gets the bullet default speed.
    fn get_default_speed(&self) -> f64;
    /// Gets the BulletML "rank", a value between 0 and 1 indicating the level of difficulty.
    /// The value is used in arithmetic expressions with `$rank`.
    fn get_rank(&self, data: &D) -> f64;
    /// Tells the application to create a bullet with the given `direction` and `speed`.
    ///
    /// The simple use case is to create a bullet whose direction and speed won't change until it
    /// disappears or hits the target.
    ///
    /// Nevertheless there could be more complex use cases which involve creating a new runner with
    /// the same BulletML document or even another one.
    fn create_simple_bullet(
        &mut self,
        data: &mut D,
        direction: f64,
        speed: f64,
        bullet_transform: &Transform,
        commands: &mut Commands,
    );
    /// Tells the application to create a bullet based on the given `state`, initial `direction`
    /// and initial `speed`.
    ///
    /// The typical use case is to create a new runner with the same BulletML document. See
    /// [Runner::new_from_state](struct.Runner.html#method.new_from_state) and
    /// [Runner::init_from_state](struct.Runner.html#method.init_from_state).
    fn create_bullet(
        &mut self,
        data: &mut D,
        state: State,
        direction: f64,
        speed: f64,
        bullet_transform: &Transform,
        commands: &mut Commands,
    );
    /// Gets the current iteration number.
    fn get_turn(&self, data: &D) -> u32;
    /// Tells the application to make this bullet vanish.
    fn do_vanish(&mut self, data: &mut D, bullet: &mut B);
    fn do_change_direction(&mut self, _data: &mut D, _direction: f64, _bullet: &mut B) {}
    /// Tells the application to make this bullet change speed.
    fn do_change_speed(&mut self, _data: &mut D, _speed: f64, _bullet: &mut B) {}
    /// Tells the application to make this bullet accelerate.
    fn do_accel_x(&mut self, _: f64, _bullet: &mut B) {}
    /// Tells the application to make this bullet accelerate.
    fn do_accel_y(&mut self, _: f64, _bullet: &mut B) {}
    /// Gets this bullet's X speed.
    fn get_bullet_speed_x(&self, _bullet: &B) -> f64 {
        0.
    }
    /// Gets this bullet's Y speed.
    fn get_bullet_speed_y(&self, _bullet: &B) -> f64 {
        0.
    }
    /// Gets a new random value. The random number generator is managed by the application.
    fn get_rand(&self, data: &mut D) -> f64;
    #[cfg(test)]
    fn log(&mut self, _data: &mut D, _node: &BulletMLNode) {}
}
