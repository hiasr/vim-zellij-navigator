use zellij_tile::prelude::*;

use std::collections::{BTreeMap, VecDeque};

const DEFAULT_DISABLED_APPS: &[&str; 3] = &["vim", "nvim", "fzf"];

struct State {
    permissions_granted: bool,
    current_term_command: Option<String>,
    command_queue: VecDeque<Command>,

    // Configuration
    enabled: bool,
    move_mod: Mod,
    resize_mod: Mod,
    disable_for_apps: Vec<String>,
}

enum Command {
    MoveFocus(Direction),
    MoveFocusOrTab(Direction),
    Resize(Direction),
}

#[derive(Debug)]
enum Mod {
    Ctrl,
    Alt,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.parse_configuration(configuration);

        request_permission(&[
            PermissionType::WriteToStdin,
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
        subscribe(&[EventType::PermissionRequestResult, EventType::ListClients]);
        if self.permissions_granted {
            hide_self();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ListClients(list) => {
                self.current_term_command = term_command_from_client_list(list);

                if !self.command_queue.is_empty() {
                    let command = self.command_queue.pop_front().unwrap();
                    self.execute_command(command);
                }
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

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let Some(command) = self.handle_message(pipe_message) {
            self.handle_command(command);
        }
        true
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            permissions_granted: false,
            current_term_command: None,
            command_queue: VecDeque::new(),

            enabled: true,
            move_mod: Mod::Ctrl,
            resize_mod: Mod::Alt,
            disable_for_apps: DEFAULT_DISABLED_APPS.map(ToString::to_string).to_vec(),
        }
    }
}

impl State {
    fn handle_message(&mut self, pipe_message: PipeMessage) -> Option<Command> {
        let payload = pipe_message.payload?;
        match payload.as_str() {
            "enable" => self.enabled = true,
            "disable" => self.enabled = false,
            "toggle" => self.enabled = !self.enabled,
            value => {
                return parse_command(&pipe_message.name, value);
            }
        }
        None
    }

    fn handle_command(&mut self, command: Command) {
        self.command_queue.push_back(command);
        list_clients();
    }

    fn execute_command(&mut self, command: Command) {
        if !self.enabled || self.current_pane_is_disabled_app() {
            write_chars(&self.command_to_keybind(&command));
            return;
        }

        match command {
            Command::MoveFocus(direction) => move_focus(direction),
            Command::MoveFocusOrTab(direction) => move_focus_or_tab(direction),
            Command::Resize(direction) => {
                resize_focused_pane_with_direction(Resize::Increase, direction)
            }
        }
    }

    fn current_pane_is_disabled_app(&self) -> bool {
        if let Some(current_command) = &self.current_term_command {
            return self.disable_for_apps.contains(current_command);
        }
        false
    }

    fn parse_configuration(&mut self, configuration: BTreeMap<String, String>) {
        self.move_mod = configuration.get("move_mod").map_or(Mod::Ctrl, |f| {
            string_to_mod(f).expect("Illegal modifier for move_mod")
        });
        self.resize_mod = configuration.get("resize_mod").map_or(Mod::Alt, |f| {
            string_to_mod(f).expect("Illegal modifier for resize_mod")
        });
        self.disable_for_apps = configuration.get("disable_for_apps").map_or(
            DEFAULT_DISABLED_APPS.map(ToString::to_string).to_vec(),
            |f| f.split(",").map(|s| s.trim().to_string()).collect(),
        );
    }

    fn command_to_keybind(&mut self, command: &Command) -> String {
        let mod_key = match command {
            Command::MoveFocus(_) | Command::MoveFocusOrTab(_) => &self.move_mod,
            Command::Resize(_) => &self.resize_mod,
        };

        let direction = match command {
            Command::MoveFocus(direction)
            | Command::MoveFocusOrTab(direction)
            | Command::Resize(direction) => direction,
        };

        match mod_key {
            Mod::Ctrl => ctrl_keybinding(direction),
            Mod::Alt => alt_keybinding(direction),
        }
    }
}

fn term_command_from_client_list(clients: Vec<ClientInfo>) -> Option<String> {
    for c in clients {
        if c.is_current_client {
            let command = c.running_command.split(' ').next()?;
            let command = command.split('/').next_back()?;
            return Some(command.to_string());
        }
    }
    None
}

fn ctrl_keybinding(direction: &Direction) -> String {
    let direction = match direction {
        Direction::Left => "\u{0008}",
        Direction::Right => "\u{000C}",
        Direction::Up => "\u{000B}",
        Direction::Down => "\u{000A}",
    };
    direction.to_string()
}

fn alt_keybinding(direction: &Direction) -> String {
    let mut char_vec: Vec<char> = vec![0x1b as char];
    char_vec.push(match direction {
        Direction::Left => 'h',
        Direction::Right => 'l',
        Direction::Up => 'k',
        Direction::Down => 'j',
    });
    char_vec.iter().collect()
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

fn string_to_mod(s: &str) -> Option<Mod> {
    match s.to_lowercase().as_str() {
        "ctrl" => Some(Mod::Ctrl),
        "alt" => Some(Mod::Alt),
        _ => None,
    }
}

fn parse_command(command: &str, payload: &str) -> Option<Command> {
    let direction = string_to_direction(payload)?;

    match command {
        "move_focus" => Some(Command::MoveFocus(direction)),
        "move_focus_or_tab" => Some(Command::MoveFocusOrTab(direction)),
        "resize" => Some(Command::Resize(direction)),
        _ => None,
    }
}
