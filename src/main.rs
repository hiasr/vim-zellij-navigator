use zellij_tile::prelude::*;

use std::collections::{BTreeMap, VecDeque};
use std::str::FromStr;

struct State {
    permissions_granted: bool,
    current_term_command: Option<String>,
    command_queue: VecDeque<Command>,

    // Configuration
    move_mod: Vec<Mod>,
    resize_mod: Vec<Mod>,
    use_arrow_keys: bool,
}

enum Command {
    MoveFocus(Direction),
    MoveFocusOrTab(Direction),
    Resize(Direction),
}

#[derive(Debug)]
enum Mod {
    Shift,
    Alt,
    Ctrl,
    Super,
    Hyper,
    Meta,
    CapsLock,
    NumLock,
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
        if let Some(command) = parse_command(pipe_message) {
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

            move_mod: vec![Mod::Ctrl],
            resize_mod: vec![Mod::Alt],
            use_arrow_keys: false,
        }
    }
}

impl State {
    fn handle_command(&mut self, command: Command) {
        self.command_queue.push_back(command);
        list_clients();
    }

    fn execute_command(&mut self, command: Command) {
        if self.current_pane_is_vim() {
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

    fn current_pane_is_vim(&self) -> bool {
        if let Some(current_command) = &self.current_term_command {
            if current_command == "nvim" || current_command == "vim" {
                return true;
            }
        }
        false
    }

    fn parse_configuration(&mut self, configuration: BTreeMap<String, String>) {
        self.move_mod = configuration.get("move_mod").map_or(vec![Mod::Ctrl], |f| {
            Self::parse_modifiers(f).expect("Illegal modifier for move_mod")
        });
        self.resize_mod = configuration.get("resize_mod").map_or(vec![Mod::Alt], |f| {
            Self::parse_modifiers(f).expect("Illegal modifier for resize_mod")
        });
        self.use_arrow_keys = configuration
            .get("use_arrow_keys")
            .is_some_and(|v| v.to_lowercase() == "true");
    }

    fn parse_modifiers(input: &str) -> Result<Vec<Mod>, String> {
        input.split('+').map(|s| s.trim().parse::<Mod>()).collect()
    }

    fn command_to_keybind(&mut self, command: &Command) -> String {
        let modifiers = match command {
            Command::MoveFocus(_) | Command::MoveFocusOrTab(_) => &self.move_mod,
            Command::Resize(_) => &self.resize_mod,
        };

        let direction = match command {
            Command::MoveFocus(direction)
            | Command::MoveFocusOrTab(direction)
            | Command::Resize(direction) => direction,
        };

        // Use the ASCII control characters for single modifier keybindings
        if modifiers.len() == 1 && !self.use_arrow_keys {
            match &modifiers[0] {
                Mod::Ctrl => return ctrl_keybinding(direction),
                Mod::Alt => return alt_keybinding(direction),
                _ => {}
            }
        }

        if self.use_arrow_keys {
            return arrow_kitty_keybinding(direction, modifiers);
        }

        kitty_keybinding(direction, modifiers)
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

fn mod_to_kitty_protocol(modifier: &Mod) -> u8 {
    match modifier {
        Mod::Shift => 1,
        Mod::Alt => 2,
        Mod::Ctrl => 4,
        Mod::Super => 8,
        Mod::Hyper => 16,
        Mod::Meta => 32,
        Mod::CapsLock => 64,
        Mod::NumLock => 128,
    }
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

fn mods_to_kitty_protocol(modifiers: &[Mod]) -> String {
    let mut kitty_modifiers = 1;
    for modifier in modifiers {
        kitty_modifiers += mod_to_kitty_protocol(modifier);
    }
    format!("{}", kitty_modifiers)
}

fn arrow_kitty_keybinding(direction: &Direction, modifiers: &[Mod]) -> String {
    let key_code = match direction {
        Direction::Up => "A",
        Direction::Down => "B",
        Direction::Right => "C",
        Direction::Left => "D",
    };
    let mod_code = mods_to_kitty_protocol(modifiers);
    format!("\x1b\x5b1;{}{}", mod_code, key_code)
}

fn kitty_keybinding(direction: &Direction, modifiers: &[Mod]) -> String {
    let key_code = match direction {
        Direction::Left => "104",
        Direction::Right => "108",
        Direction::Up => "107",
        Direction::Down => "106",
    };

    let mod_code = mods_to_kitty_protocol(modifiers);

    format!("\x1b\x5b{};{}u", key_code, mod_code)
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

impl FromStr for Mod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shift" => Ok(Mod::Shift),
            "alt" => Ok(Mod::Alt),
            "ctrl" => Ok(Mod::Ctrl),
            "super" => Ok(Mod::Super),
            "hyper" => Ok(Mod::Hyper),
            "meta" => Ok(Mod::Meta),
            "caps_lock" => Ok(Mod::CapsLock),
            "num_lock" => Ok(Mod::NumLock),
            _ => Err(format!("Invalid modifier: {}", s)),
        }
    }
}

fn parse_command(pipe_message: PipeMessage) -> Option<Command> {
    let payload = pipe_message.payload?;
    let command = pipe_message.name;

    let direction = string_to_direction(payload.as_str())?;

    match command.as_str() {
        "move_focus" => Some(Command::MoveFocus(direction)),
        "move_focus_or_tab" => Some(Command::MoveFocusOrTab(direction)),
        "resize" => Some(Command::Resize(direction)),
        _ => None,
    }
}
