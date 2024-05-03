use zellij_tile::prelude::*;

use std::collections::{BTreeMap, VecDeque};

#[derive(Default)]
struct State {
    userspace_configuration: BTreeMap<String, String>,
    permissions_granted: bool,
    current_command: Option<String>,
    direction_queue: VecDeque<Direction>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.userspace_configuration = configuration;
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::WriteToStdin,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::PermissionRequestResult,
            EventType::RunCommandResult,
        ]);
        if self.permissions_granted {
            hide_self();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::RunCommandResult(_, stdout, _, _) => {
                let stdout = String::from_utf8(stdout).unwrap();

                self.current_command = command_from_client_list(stdout);

                if !self.direction_queue.is_empty() {
                    let direction = self.direction_queue.pop_front().unwrap();
                    self.move_focus(direction);
                }
            },

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
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        match pipe_message.name.as_str() {
            "move_focus" => self.handle_move_focus(pipe_message.payload),
            _ => {}
        }
        true
    }
}

impl State {
    fn handle_move_focus(&mut self, payload: Option<String>) {
        if payload.is_none() {
            return;
        }

        let direction = string_to_direction(payload.unwrap().as_str());
        if direction.is_none() {
            return;
        }

        self.direction_queue.push_back(direction.unwrap());
        run_command(&["zellij", "action", "list-clients"], BTreeMap::new());
    }

    fn move_focus(&mut self, direction: Direction) {
        if self.current_pane_is_vim() {
            write_chars(direction_to_keybinding(direction));
        } else {
            match direction {
                Direction::Left | Direction::Right => move_focus_or_tab(direction),
                _ => move_focus(direction),
            }
        }
    }

    fn current_pane_is_vim(&self) -> bool {
        if let Some(current_command) = &self.current_command {
            if current_command == "nvim" || current_command == "vim" {
                return true;
            }
        }
        false
    }

}

fn command_from_client_list(cl: String) -> Option<String> {
    let clients = cl.split('\n').skip(1).collect::<Vec<&str>>();
    if clients.is_empty() {
        return None;
    }

    let columns = clients[0].split_whitespace().collect::<Vec<&str>>();
    if columns.len() < 3 {
        return None;
    }

    let is_terminal = columns[1].starts_with("terminal");
    let no_command = columns[2] == "N/A";
    if !is_terminal || no_command {
        return None;
    }

    let command = columns[2].split('/').last()?;
    Some(command.to_string())
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
