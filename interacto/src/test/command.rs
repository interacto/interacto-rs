/*
 * This file is part of Interacto.
 * Interacto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * Interacto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with Interacto.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::command::CustomCmd;

pub struct StubCmd {
    can_do_value: bool,
    exec: u32,
    effects: bool
}

impl Default for StubCmd {
    fn default() -> StubCmd {
        StubCmd::new(true)
    }
}

impl StubCmd {
    pub fn new(can_do_value: bool) -> Self {
        Self {
            can_do_value,
            exec: 0,
            effects: true,
        }
    }
}

impl CustomCmd<'_> for StubCmd {
    fn execution(&mut self) {
        self.exec += 1;
    }

    fn can_execute(&self) -> bool {
        self.can_do_value
    }
}


mod command {
    use crate::command::{CmdStatus, CustomCmd};
    use super::StubCmd;

    // fn create_stub<'a>() -> &'a Command<'a, StubCmd> {
    //     let mut spec = StubCmd::default();
    //     spec.as_command()
    // }

    #[test]
    fn cando_default() {
        struct Cmd;
        impl CustomCmd<'_> for Cmd {
            fn execution(&mut self) {}
        }
        impl Cmd {
            pub fn new() -> Self {
                Self {}
            }
        }

        assert_eq!(Cmd::new().as_command().can_execute(), true);
    }

    #[test]
    fn command_status_after_creation() {
        assert_eq!(StubCmd::default().as_command().get_status(), CmdStatus::Created);
    }

    #[test]
    fn command_status_after_flush() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.flush();
        assert_eq!(cmd.get_status(), CmdStatus::Flushed);
    }

    #[test]
    fn command_cannot_do_it_when_flushed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.flush();
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_cannot_do_it_when_done() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.done();
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_cannot_do_it_when_cancelled() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.cancel();
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_cannot_do_it_when_cannot_do_and_created() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.child.can_do_value = false;
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_can_do_it_when_can_do() {
        assert_eq!(StubCmd::default().as_command().execute(), true);
    }

    #[test]
    fn command_is_executed_when_do_it() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.execute();
        assert_eq!(cmd.get_status(), CmdStatus::Executed);
    }

    #[test]
    fn command_had_effect_when_done() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.done();
        assert_eq!(cmd.had_effect(), true);
    }

    #[test]
    fn command_had_effect_when_not_done_and_created() {
        assert_eq!(StubCmd::default().as_command().had_effect(), false);
    }

    #[test]
    fn command_had_effect_when_not_done_and_cancelled() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.cancel();
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_had_effect_when_not_done_and_flushed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.flush();
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_had_effect_when_not_done_and_executed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.child.can_do_value = true;
        cmd.execute();
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_not_done_when_flushed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.flush();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Flushed);
    }

    #[test]
    fn command_not_done_when_cancelled() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.cancel();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Cancelled);
    }

    #[test]
    fn command_done_when_created() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Done);
    }

    #[test]
    fn command_done_when_executed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.execute();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Done);
    }

    #[test]
    fn is_done_when_created() {
        assert_eq!(StubCmd::default().as_command().is_done(), false);
    }

    #[test]
       fn is_done_when_cancelled() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.cancel();
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn is_done_when_flushed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.flush();
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn is_done_when_done() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.done();
        assert_eq!(cmd.is_done(), true);
    }

    #[test]
       fn is_done_when_executed() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.execute();
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn cancel() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.cancel();
        assert_eq!(cmd.get_status(), CmdStatus::Cancelled);
    }

    #[test]
       fn executed_two_times() {
        let mut spec = StubCmd::default();
        let mut cmd = spec.as_command();
        cmd.execute();
        cmd.execute();
        assert_eq!(cmd.child.exec, 2);
    }

    // #[test]
    //    fn crash_in_execution_command_executed() {
    //     struct Cmd;
    //     impl CustomCmd for Cmd {
    //         fn execution(&mut self) {
    //             panic!()
    //         }
    //     }
    //     impl Cmd {
    //         pub fn new() -> Self {
    //             Self {}
    //         }
    //     }

    //     let mut cmd: Command<Cmd> = Command::new(Cmd::new());
    //     cmd.execute();

    //     // expect(() => command.execute() as boolean).toThrow(new Error("Cmd err"));
    //     // expect(command.getStatus()).toBe("executed");
    // }
}
