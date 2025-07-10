# vim-zellij-navigator
This plugin is designed to give the same functionality as [vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator) in Zellij.

Note: When a new session is started, a pane flashes the first time each direction is pressed. This will be fixed in a near release of Zellij when headless mode is available.

## Version 0.1.0 vs 0.2.0
Version 0.1.0 makes use of an additional Neovim plugin to know whether Neovim is currently opened. 
Starting from 0.2.0 the plugin makes use of the Zellij `list-clients` command to remove the need for the Neovim plugin, this currently has no plugin binding so a shell command needs to be launched from the plugin, which introduces some delay. If this bothers you, you can continue using 0.1.0 by changing the keybindings to use this version. This problem will be gone in the next release when direct plugin bindings for `list-clients` have released.

## Installation
Minimum Zellij version: v0.42.2

Zellij plugins do not need to be installed, simply add the keybindings to your zellij configuration file.

Next to this plugin, you will also need support from the Neovim side to execute the correct commands if the edge of the Neovim panes have been reached.
Since the plugin works by sending Ctrl+hjkl to Neovim, these must be bound to the corresponding command in one of the following plugins.

- [smart-splits.nvim](https://github.com/mrjones2014/smart-splits.nvim)
- [zellij-nav.nvim](https://github.com/swaits/zellij-nav.nvim)
- [Navigator.nvim](https://github.com/numToStr/Navigator.nvim): There is currently a PR which adds support for Zellij, until then [this fork](https://github.com/dynamotn/Navigator.nvim) can be used. The advantage of Navigator.nvim is that it will detect if it is running in Zellij and Tmux and will work with both without changing any config.

For version <0.2.0 you also need to install this plugin:
```lua
-- Lazy.nvim
{
    "hiasr/vim-zellij-navigator.nvim",
    config = function()
        require('vim-zellij-navigator').setup()
    end
},
```

## Usage
Available commands:
- `move_focus` with payload `up`, `down`, `left`, `right` to move the focus in the corresponding direction.
- `move_focus_or_tab` with payload `up`, `down`, `left`, `right` to move the focus in the corresponding direction or switch to the next tab if the focus is already at the edge.
- `resize` with payload `up`, `down`, `left`, `right` to resize the pane in the corresponding direction.

If you use configuration for the plugin it must be added to every command in order to function consistently. 
This is because the plugin is loaded with the configuration of the first command executed.

Available configuration options:
- `move_mod`: The modifier keys passed to Neovim with `move_focus` or `move_focus_or_tab`. Default: `ctrl`. Multiple modifier keys should be separated with a `+`.
- `resize_mod`: The modifier keys passed to Neovim with the `resize` command. Default: `alt`. Multiple modifier keys should be separated with a `+`.
- `use_arrow_keys`: When set to `true`, uses arrow key sequences instead of hjkl for keybindings sent to Neovim. This setting can differ per command. Default: `false`.

### Modifier Keys Support

The plugin supports multiple modifier combinations. Available modifiers:
- `ctrl` - Control key (uses ASCII control characters when used alone)
- `alt` - Alt key (uses escape sequences when used alone)
- `shift` - Shift key
- `super` - Super/Windows key
- `hyper` - Hyper key
- `meta` - Meta key
- `caps_lock` - Caps Lock key
- `num_lock` - Num Lock key

Examples:
- Single modifier: `move_mod "ctrl"`
- Multiple modifiers: `move_mod "ctrl+shift"`
- Complex combination: `resize_mod "alt+super+shift"`

**Note:** Single `ctrl` and `alt` modifiers without using arrow keys use optimized ASCII control characters and escape sequences respectively. All other combinations (including multi-modifier with ctrl/alt) use the kitty keyboard protocol. Unexpected behaviour may occur if your terminal emulator doesn't support this.

### Example Configuration

Basic configuration:
```javascript
keybinds {
    shared_except "locked" {
        bind "Ctrl h" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "move_focus_or_tab";
                payload "left";

                // Plugin Configuration
                move_mod "ctrl"; // Optional, should be added on every move command if changed.
                use_arrow_keys "false"; // Optional, uses arrow keys instead of hjkl. Should be added to every command where you want to use it.
            };
        }

        bind "Ctrl j" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "down";

                move_mod "ctrl";
                use_arrow_keys "false";
            };
        }

        bind "Ctrl k" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "up";

                move_mod "ctrl";
                use_arrow_keys "false";
            };
        }

        bind "Ctrl l" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "move_focus_or_tab";
                payload "right";

                move_mod "ctrl"; // Optional, should be added on every command if you want to use it
                use_arrow_keys "false";
            };
        }

        bind "Alt h" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "resize";
                payload "left";

                resize_mod "alt"; 
            };
        }

        bind "Alt j" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "resize";
                payload "down";

                resize_mod "alt";
            };
        }

        bind "Alt k" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "resize";
                payload "up";

                resize_mod "alt";
            };
        }

        bind "Alt l" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "resize";
                payload "right";

                resize_mod "alt";
            };
        }
    }
}
```

Configuration with multiple modifiers:
```javascript
keybinds {
    shared_except "locked" {
        bind "Ctrl Shift h" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "left";
                
                move_mod "ctrl+shift"; // Multiple modifiers
                resize_mod "alt+super"
            };
        }
        
        bind "Alt Super j" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.3.0/vim-zellij-navigator.wasm" {
                name "resize";
                payload "down";
                
                move_mod "ctrl+shift"; 
                resize_mod "alt+super";
            };
        }
    }
}
```

