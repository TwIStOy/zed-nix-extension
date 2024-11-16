use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct NixExtension;

const LSP_SERVERS: &[&'static str] = &["nixd", "nil"];

impl zed::Extension for NixExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // returns the first available language server
        let path = LSP_SERVERS
            .iter()
            .find_map(|binary_name| worktree.which(binary_name))
            .ok_or_else(|| {
                "No Nix language server (nixd or nil) is available in your environment (PATH)."
                    .to_string()
            })?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree);
        match settings {
            Ok(settings) => {
                let init_opts = settings.initialization_options;
                match init_opts {
                    Some(init_opts) => {
                        return Ok(Some(zed::serde_json::to_value(init_opts).map_err(
                            |_| {
                                "Failed to serialize LSP initialization options to JSON."
                                    .to_string()
                            },
                        )?));
                    }
                    None => Ok(None),
                }
            }
            Err(_err) => Ok(None),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree);
        match settings {
            Ok(settings) => {
                let settings = settings.settings;
                match settings {
                    Some(init_opts) => {
                        return Ok(Some(zed::serde_json::to_value(init_opts).map_err(
                            |_| {
                                "Failed to serialize LSP initialization options to JSON."
                                    .to_string()
                            },
                        )?));
                    }
                    None => Ok(None),
                }
            }
            Err(_err) => Ok(None),
        }
    }
}

zed::register_extension!(NixExtension);
