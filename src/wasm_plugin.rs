use anyhow::Result;
use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncPluginHandler;

use crate::configuration::Configuration; // import the Configuration from above

#[derive(Default)]
pub struct MyPluginHandler;

impl SyncPluginHandler<Configuration> for MyPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "keyGoesHere".to_string(),
            help_url: "".to_string(),          // fill this in
            config_schema_url: "".to_string(), // leave this empty for now
            update_url: None,                  // leave this empty for now
        }
    }

    fn license_text(&mut self) -> String {
        "License text goes here.".to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        // implement this... for example
        let mut config = config;
        let mut diagnostics = Vec::new();
        let line_width = get_value(
            &mut config,
            "line_width",
            global_config.line_width.unwrap_or(120),
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration { line_width },
            diagnostics,
            file_matching: FileMatchingInfo {
                // these can be derived from the config
                file_extensions: vec!["txt".to_string()],
                file_names: vec![],
            },
        }
    }

    fn check_config_updates(
        &self,
        message: dprint_core::plugins::CheckConfigUpdatesMessage,
    ) -> Result<Vec<dprint_core::plugins::ConfigChange>> {
        todo!()
    }

    fn format(
        &mut self,
        request: dprint_core::plugins::SyncFormatRequest<Configuration>,
        format_with_host: impl FnMut(
            dprint_core::plugins::SyncHostFormatRequest,
        ) -> dprint_core::plugins::FormatResult,
    ) -> dprint_core::plugins::FormatResult {
        todo!()
    }
}

generate_plugin_code!(MyPluginHandler, MyPluginHandler);
