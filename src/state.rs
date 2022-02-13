use crate::BulletML;
use indextree::NodeId;
use std::sync::Arc;

use crate::parameters::Parameters;
use crate::tree::BulletMLType;

/// State information that can be used to call
/// [Runner::new_from_state](struct.Runner.html#method.new_from_state) or
/// [Runner::init_from_state](struct.Runner.html#method.init_from_state) when creating new bullets.
///
/// See also [AppRunner::create_bullet](trait.AppRunner.html#tymethod.create_bullet).
pub struct State {
    pub bml: Arc<BulletML>,
    pub bml_type: Option<BulletMLType>,
    pub nodes: Box<[NodeId]>,
    pub parameters: Parameters,
}
