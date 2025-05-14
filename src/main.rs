use std::collections::BTreeMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use zellij_tile::prelude::*;
use zellij_tile::{register_plugin, ZellijPlugin};

use zellij_tile::shim::cli_pipe_output;

#[derive(Default)]
struct ZellijGetModePlugin { }

lazy_static! {
    static ref CURRENT_MODE: Mutex<InputMode> = Mutex::new(InputMode::Normal);
}

impl ZellijPlugin for ZellijGetModePlugin {

    fn update(&mut self, event: Event) -> bool {

        if let Event::ModeUpdate(mode_info) = &event {
            let mut current_mode = CURRENT_MODE.lock().unwrap();
            *current_mode = mode_info.mode;
        }

        false
    }

    fn pipe(&mut self, pipe_message: zellij_tile::prelude::PipeMessage) -> bool {

        if let (PipeSource::Cli(input_pipe_id), Some("getmode")) = (pipe_message.source, pipe_message.payload.as_deref()) {
            cli_pipe_output(&input_pipe_id, &format!("{:?}\n", *CURRENT_MODE.lock().unwrap()));
        }
       
        false
    }

     fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadCliPipes, PermissionType::ReadApplicationState]);
        subscribe(&[EventType::ModeUpdate]);
    }
}

register_plugin!(ZellijGetModePlugin);
