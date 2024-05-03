# vim-zellij-navigator
This plugin is designed to give the same functionality as [vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator) in Zellij.

Note: When a new session is started, a pane flashes the first time each direction is pressed. This will be fixed in a near release of Zellij when headless mode is available.

## Version 0.1.0 vs 0.2.0
Version 0.1.0 makes use of an additional Neovim plugin to know whether Neovim is currently opened. 
Starting from 0.2.0 the plugin makes use of the Zellij `list-clients` command to remove the need for the Neovim plugin, this currently has no plugin binding so a shell command needs to be launched from the plugin, which introduces some delay. If this bothers you, you can continue using 0.1.0 by changing the keybindings to use this version. This problem will be gone in the next release when direct plugin bindings for `list-clients` have released.

## Installation
Minimum Zellij version: v0.40.1

Zellij plugins do not need to be installed, simply add the keybindings to your zellij configuration file.

Next to this plugin, you will also need support from the Neovim side to execute the correct commands if the edge of the Neovim panes have been reached.
Since the plugin works by sending Ctrl+hjkl to Neovim, these must be bound to the corresponding command in one of the following plugins.
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
```javascript
keybinds {
    shared_except "locked" {
        bind "Ctrl h" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.2.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "left";
            };
        }

        bind "Ctrl j" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.2.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "down";
            };
        }

        bind "Ctrl k" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.2.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "up";
            };
        }

        bind "Ctrl l" {
            MessagePlugin "https://github.com/hiasr/vim-zellij-navigator/releases/download/0.2.0/vim-zellij-navigator.wasm" {
                name "move_focus";
                payload "right";
            };
        }
    }
}
```

