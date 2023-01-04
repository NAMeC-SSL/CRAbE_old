use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::protobuf::simulation_packet::robot_move_command::Command;
use crate::libs::protobuf::simulation_packet::{MoveLocalVelocity, RobotCommand, RobotMoveCommand};
use crate::libs::tasks::task::Task;
use gilrs::{Axis, Button, Event, Gamepad, GamepadId, Gilrs};

pub struct GamepadInputTask {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl Task for GamepadInputTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        let gilrs = Gilrs::new().unwrap();

        // Iterate over all connected gamepads
        for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }

        Self {
            gilrs,
            active_gamepad: None,
        }
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        // Examine new events
        while let Some(Event { id, event, time }) = self.gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            self.active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = self.active_gamepad.map(|id| self.gilrs.gamepad(id)) {
            // Create robot command
            let mut r = RobotCommand::default();
            r.id = 0;

            // Move Local Velocity
            let mut move_robot: MoveLocalVelocity = MoveLocalVelocity::default();
            if gamepad.value(Axis::RightStickY) > 0.1 || gamepad.value(Axis::RightStickY) < -0.1 {
                move_robot.forward = gamepad.value(Axis::RightStickY);
            }

            if gamepad.value(Axis::RightStickX) > 0.1 || gamepad.value(Axis::RightStickX) < -0.1 {
                move_robot.left = -gamepad.value(Axis::RightStickX);
            }

            if gamepad.is_pressed(Button::LeftTrigger) {
                move_robot.angular = 2.0;
            }

            if gamepad.is_pressed(Button::RightTrigger) {
                move_robot.angular = -2.0;
            }

            let command = Command::LocalVelocity(move_robot);

            r.move_command = Some(RobotMoveCommand {
                command: Some(command),
            });

            if let Some(robot) = data_store.allies.get_mut(0) {
                robot.command = Some(r);
            }
        }

        Ok(())
    }
}
