//! Applying the fork's local offline preflight to GUI Play.
use super::*;
use std::process::Command;

impl Launcher {
    pub(super) fn handle_offline_game(&mut self, command: std::result::Result<Command, String>) {
        if let Some(tag) = self.starting_client.take() {
            let instance_id = self.starting_instance.take();
            let cmd = match command {
                Ok(cmd) => cmd,
                Err(error) => {
                    self.client_error = Some(error);
                    self.client_started = None;
                    self.play_pressed = None;
                    self.starting_pack = None;
                    self.joining = None;
                    self.quick_play = None;
                    return;
                }
            };
            // A pack's Play button plays that pack; Play plays the selected instance.
            let pack = self.starting_pack.take();
            let instance = match &pack {
                Some(work) => instances::list().into_iter().find(|i| &i.work == work),
                None => {
                    let Some(instance) = instances::list()
                        .into_iter()
                        .find(|i| Some(&i.id) == instance_id.as_ref())
                    else {
                        self.client_error = Some(
                            tr("This instance no longer exists. Choose another instance to play.")
                                .into(),
                        );
                        self.joining = None;
                        self.quick_play = None;
                        return;
                    };
                    Some(instance)
                }
            };
            let run = instance.as_ref().map(|i| i.id.clone());
            // What follows the profile's own arguments, in the same order: the player's extra
            // arguments come last, to add to the launcher's or override them.
            let mut after = std::process::Command::new("");
            if let Some(code) = self.joining.take() {
                if let Err(e) = backend::join_friend(&mut after, &code) {
                    self.client_error = Some(e);
                }
            }
            if let Some((flag, value)) = self.quick_play.take() {
                after.arg(flag).arg(value);
            }
            if let Err(error) = backend::append_offline_game_args(&mut after) {
                self.client_error = Some(error);
                return;
            }
            let after: Vec<_> = after.get_args().map(ToOwned::to_owned).collect();
            // Last, around every argument: the instance's pre-launch command,
            // wrapper and variables.
            self.client_vars = instance
                .as_ref()
                .map(|i| i.vars(cmd.get_program()))
                .unwrap_or_default();
            let vars = self.client_vars.clone();
            let launch = instance
                .as_ref()
                .map(|i| i.launch.clone())
                .unwrap_or_default();
            // The profile on the launch's thread: a pack's carried mods are read whole for it.
            let profile = move |cmd: &mut std::process::Command| {
                match (&pack, &instance) {
                    (Some(work), _) => horizon_launcher::modpacks::apply_profile(cmd, work)?,
                    (None, Some(instance)) => instances::apply(cmd, instance)?,
                    (None, None) => {}
                }
                cmd.args(after);
                cmd.envs(vars.iter().map(|(k, v)| (k, v)));
                backend::strip_session_env(cmd);
                Ok(())
            };
            self.starting_client = Some(tag);
            // The jar phase keeps the row of the started instance on Starting.
            self.starting_instance = run.clone();
            let stop = self.pre_launch_stop_for(&launch);
            jar_launch::client(cmd, profile, launch, stop, run, self.tx.clone());
        }
    }
}
