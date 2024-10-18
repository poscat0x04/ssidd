use crate::machine::{Input, Output, State, UnitState};
use zbus::fdo::Error;
use zbus::fdo::Error::InvalidArgs;
use zbus::interface;


#[derive(Debug, Clone)]
struct UnitController {}

impl UnitController {
    pub async fn up(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn down(&self) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub state: State,
    pub unit_ctrl: UnitController,
}

#[interface(name = "moe.poscat.SSIDd")]
impl App {
    #[zbus(property)]
    async fn op_state(&self) -> u8 {
        self.state.op_state as u8
    }

    #[zbus(property)]
    async fn current_state(&self) -> u8 {
        self.state.current_state as u8
    }

    #[zbus(property)]
    async fn target_state(&self) -> u8 {
        self.state.target_state as u8
    }

    #[zbus(property)]
    async fn set_op_state(&mut self, state: u8) -> Result<(), Error> {
        match state.try_into() {
            Ok(s) => {
                self.run_input(Input::OpStateUpdate { op_state: s }).await?;
                Ok(())
            }
            Err(()) => Err(InvalidArgs(format!("unknown OP state {}", state)))
        }
    }
}

impl App {
    pub async fn run_input(&mut self, input: Input) -> Result<(), Error> {
        self.state.transition(input);
        let Output { new_state } = self.state.output();

        match new_state {
            None => { Ok(()) }
            Some(UnitState::Up) => {
                self.unit_ctrl.up().await
            }
            Some(UnitState::Down) => {
                self.unit_ctrl.down().await
            }
        }
    }
}
