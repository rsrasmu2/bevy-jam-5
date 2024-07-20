use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Select,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Move => InputControlKind::DualAxis,
            PlayerAction::Select => InputControlKind::Button,
        }
    }
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);
        input_map.insert(Self::Select, MouseButton::Left);

        input_map
    }
}
