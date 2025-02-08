use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "@djowea/sqlite-context-server"; // Update this
const PACKAGE_VERSION: &str = "0.1.2";
const SERVER_PATH: &str = "node_modules/@djowea/sqlite-context-server/index.mjs";

struct SqliteModelContextExtension;

#[derive(Debug, Deserialize)]
struct SqliteContextServerSettings {
    database_path: String,
}

impl zed::Extension for SqliteModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("sqlite-context-server", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `database_path` setting".into());
        };
        let settings: SqliteContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: "node".to_string(),
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![("DATABASE_PATH".into(), settings.database_path)],
        })
    }
}

zed::register_extension!(SqliteModelContextExtension);
