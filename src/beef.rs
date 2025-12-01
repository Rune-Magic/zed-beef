use zed_extension_api as zed;

struct BeefExtension;

impl zed::Extension for BeefExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
            &mut self,
            _language_server_id: &zed::LanguageServerId,
            _worktree: &zed::Worktree,
        ) -> zed::Result<zed::Command> {
            Ok(zed::Command {
                command: "BeefLsp".to_string(),
                args: [].to_vec(),
                env: [].to_vec(),
            })
    }
}

zed::register_extension!(BeefExtension);
