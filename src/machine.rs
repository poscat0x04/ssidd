#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum OpState {
    Auto,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ServiceState{
    Up,
    Down,
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State {
    op_state: OpState,
    current_state: ServiceState,
    target_state: ServiceState,
}

impl State {
    fn transition(&mut self, input: Input) {
        match input {
            Input::OpStateUpdate { op_state } => { self.op_state = op_state; }
            Input::CurrentStateUpdate { current_state } => { self.current_state = current_state; }
            Input::TargetStateUpdate { target_state } => { self.target_state = target_state; }
        }
    }

    fn output(&self) -> Output {
        match self {
            State{
                op_state: OpState::Auto,
                current_state,
                target_state,
            } => {
                if (current_state != target_state) {
                    Output::state(target_state)
                } else {
                    Output::noop()
                }
            },
            State{
                op_state: OpState::Up,
                current_state,
                target_state: _
            } => {
                if (*current_state == ServiceState::Down) {
                    Output::up()
                } else {
                    Output::noop()
                }
            },
            State{
                op_state: OpState::Down,
                current_state,
                target_state: _
            } => {
                if (*current_state == ServiceState::Up) {
                    Output::down()
                } else {
                    Output::noop()
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Input {
    OpStateUpdate{
        op_state: OpState
    },
    CurrentStateUpdate {
        current_state: ServiceState
    },
    TargetStateUpdate {
        target_state: ServiceState
    },
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Output {
    new_service_state: Option<ServiceState>,
}

impl Output {
    fn noop() -> Self { return Output { new_service_state: None} }

    fn up() -> Self { return Output { new_service_state: Some(ServiceState::Up) } }

    fn down() -> Self { return Output { new_service_state: Some(ServiceState::Down) } }

    fn state(service_state: &ServiceState) -> Self { return Output { new_service_state: Some(*service_state) } }
}
