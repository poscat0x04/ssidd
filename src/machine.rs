#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OpState {
    Auto = 0,
    Up = 1,
    Down = 2,
}

impl TryFrom<u8> for OpState {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Auto),
            1 => Ok(Self::Up),
            2 => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnitState {
    Up,
    Down,
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct State {
    pub op_state: OpState,
    pub current_state: UnitState,
    pub target_state: UnitState,
}

impl State {
    pub fn transition(&mut self, input: Input) {
        match input {
            Input::OpStateUpdate { op_state } => { self.op_state = op_state; }
            Input::CurrentStateUpdate { current_state } => { self.current_state = current_state; }
            Input::TargetStateUpdate { target_state } => { self.target_state = target_state; }
        }
    }

    pub fn output(&self) -> Output {
        match self {
            State {
                op_state: OpState::Auto,
                current_state,
                target_state,
            } => {
                if current_state != target_state {
                    Output::state(target_state)
                } else {
                    Output::noop()
                }
            }
            State {
                op_state: OpState::Up,
                current_state,
                target_state: _
            } => {
                if *current_state == UnitState::Down {
                    Output::up()
                } else {
                    Output::noop()
                }
            }
            State {
                op_state: OpState::Down,
                current_state,
                target_state: _
            } => {
                if *current_state == UnitState::Up {
                    Output::down()
                } else {
                    Output::noop()
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Input {
    OpStateUpdate {
        op_state: OpState
    },
    CurrentStateUpdate {
        current_state: UnitState
    },
    TargetStateUpdate {
        target_state: UnitState
    },
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Output {
    pub new_state: Option<UnitState>,
}

impl Output {
    fn noop() -> Self { return Output { new_state: None }; }

    fn up() -> Self { return Output { new_state: Some(UnitState::Up) }; }

    fn down() -> Self { return Output { new_state: Some(UnitState::Down) }; }

    fn state(service_state: &UnitState) -> Self { return Output { new_state: Some(*service_state) }; }
}
