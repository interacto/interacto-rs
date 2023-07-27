use crate::command::CustomCmd;

pub struct StubCmd {
    can_do_value: bool,
    exec: u32,
    effects: bool,
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

impl CustomCmd for StubCmd {
    fn execution(&mut self) {
        self.exec += 1;
    }

    fn can_execute(&self) -> bool {
        self.can_do_value
    }
}


mod command {
    use crate::{
        command::{CmdStatus, Command, CustomCmd},
        test::command::StubCmd,
    };

    #[test]
    fn cando_default() {
        struct Cmd;
        impl CustomCmd for Cmd {
            fn execution(&mut self) {}
        }
        impl Cmd {
            pub fn new() -> Self {
                Self {}
            }
        }

        assert_eq!(Command::new(Cmd::new()).can_execute(), true);
    }

    #[test]
    fn command_status_after_creation() {
        let cmd = Command::new(StubCmd::default());
        assert_eq!(cmd.get_status(), CmdStatus::Created);
    }

    #[test]
    fn command_status_after_flush() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.flush();
        assert_eq!(cmd.get_status(), CmdStatus::Flushed);
    }

    #[test]
    fn command_cannot_do_it_when_flushed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.flush();
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_cannot_do_it_when_done() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.done();
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_cannot_do_it_when_cancelled() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.cancel();
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_cannot_do_it_when_cannot_do_and_created() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.child.can_do_value = false;
        assert_eq!(cmd.execute(), false);
    }

    #[test]
    fn command_can_do_it_when_can_do() {
        let mut cmd = Command::new(StubCmd::default());
        assert_eq!(cmd.execute(), true);
    }

    #[test]
    fn command_is_executed_when_do_it() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.execute();
        assert_eq!(cmd.get_status(), CmdStatus::Executed);
    }

    #[test]
    fn command_had_effect_when_done() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.done();
        assert_eq!(cmd.had_effect(), true);
    }

    #[test]
    fn command_had_effect_when_not_done_and_created() {
        let cmd = Command::new(StubCmd::default());
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_had_effect_when_not_done_and_cancelled() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.cancel();
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_had_effect_when_not_done_and_flushed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.flush();
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_had_effect_when_not_done_and_executed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.child.can_do_value = true;
        cmd.execute();
        assert_eq!(cmd.had_effect(), false);
    }

    #[test]
    fn command_not_done_when_flushed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.flush();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Flushed);
    }

    #[test]
    fn command_not_done_when_cancelled() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.cancel();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Cancelled);
    }

    #[test]
    fn command_done_when_created() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Done);
    }

    #[test]
    fn command_done_when_executed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.execute();
        cmd.done();
        assert_eq!(cmd.get_status(), CmdStatus::Done);
    }

    #[test]
    fn is_done_when_created() {
        let cmd = Command::new(StubCmd::default());
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn is_done_when_cancelled() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.cancel();
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn is_done_when_flushed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.flush();
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn is_done_when_done() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.done();
        assert_eq!(cmd.is_done(), true);
    }

    #[test]
       fn is_done_when_executed() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.execute();
        assert_eq!(cmd.is_done(), false);
    }

    #[test]
       fn cancel() {
        let mut cmd = Command::new(StubCmd::default());
        cmd.cancel();
        assert_eq!(cmd.get_status(), CmdStatus::Cancelled);
    }

    #[test]
       fn executed_two_times() {
        let mut cmd = Command::new(StubCmd::default());
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
