use crate::Runner;
use bevy::prelude::*;

pub trait BulletCreater<D> {
	fn create_simple_bullet(&mut self, position: &Vec3, direction: f64, speed: f64);
	fn create_bullet(
		&mut self,
		data: &mut D,
		runner: Runner<D>,
		position: &Vec3,
		direction: f64,
		speed: f64,
	);
}
