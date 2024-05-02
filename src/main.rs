use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    userspace_configuration: BTreeMap<String, String>,
    permissions_granted: bool,
    current_pane_is_vim: bool,
    current_tab: usize,
    payload: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.userspace_configuration = configuration;
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::RunCommands,
            PermissionType::WriteToStdin,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::PaneUpdate,
            EventType::TabUpdate,
            EventType::PermissionRequestResult,
        ]);
        if self.permissions_granted {
            hide_self();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PaneUpdate(pane_manifest) => {
                let focused_pane = focused_pane_pos(self.current_tab, &pane_manifest).unwrap();
                self.current_pane_is_vim =
                    focused_pane.title == "vim" || focused_pane.title == "nvim";
            }
            Event::TabUpdate(tab_infos) => {
                self.current_tab = get_focused_tab(&tab_infos).unwrap().position;
            }
            Event::PermissionRequestResult(permission) => {
                self.permissions_granted = match permission {
                    PermissionStatus::Granted => true,
                    PermissionStatus::Denied => false,
                };
                if self.permissions_granted {
                    hide_self();
                }
            }
            _ => {}
        }
        true
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("Is vim: {}", self.current_pane_is_vim);
        println!("Payload: {}", self.payload)
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        match pipe_message.name.as_str() {
            "nvim_hook" => self.handle_nvim_hook(pipe_message.payload),
            "move_focus" => self.handle_move_focus(pipe_message.payload),
            _ => {}
        }
        true
    }
}

impl State {
    fn handle_nvim_hook(&mut self, payload: Option<String>) {
        if payload.is_none() {
            return;
        }
        match payload.unwrap().as_str() {
            "open" => {
                self.current_pane_is_vim = true;
            }
            "close" => {
                self.current_pane_is_vim = false;
            }
            _ => {}
        }
    }

    fn handle_move_focus(&mut self, payload: Option<String>) {
        if payload.is_none() {
            return;
        }
        self.payload = payload.clone().unwrap();

        let direction = string_to_direction(payload.unwrap().as_str());
        if direction.is_none() {
            return;
        }

        if self.current_pane_is_vim {
            write_chars(direction_to_keybinding(direction.unwrap()));
        } else {
            move_focus(direction.unwrap());
        }
    }
}

fn focused_pane_pos(tab_position: usize, pane_manifest: &PaneManifest) -> Option<PaneInfo> {
    let panes = pane_manifest.panes.get(&tab_position);
    if let Some(panes) = panes {
        for pane in panes {
            if pane.is_focused {
                return Some(pane.clone());
            }
        }
    }
    None
}

fn direction_to_keybinding(direction: Direction) -> &'static str {
    match direction {
        Direction::Left => "\u{0008}",
        Direction::Right => "\u{000C}",
        Direction::Up => "\u{000B}",
        Direction::Down => "\u{000A}",
    }
}

fn string_to_direction(s: &str) -> Option<Direction> {
    match s {
        "left" => Some(Direction::Left),
        "right" => Some(Direction::Right),
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        _ => None,
    }
}
