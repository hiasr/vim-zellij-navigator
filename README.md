# vim-zellij-navigator
This plugin is designed to give the same functionality as [vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator) in zellij.

Note: When a new session is started, a pane the first time each direction is pressed. This will be fixed in a near release of Zellij when headless mode is available.

## Installation
Zellij plugins do not need to be installed, simply add the keybindings to your zellij configuration file.

Next to this plugin, you will also need support from the Neovim side to execute the correct commands if the edge of the Neovim panes have been reached.
Since the plugin works by sending Ctrl+hjkl to Neovim, these must be bound to the corresponding command in one of the following plugins.
- [zellij-nav.nvim](https://github.com/swaits/zellij-nav.nvim)
- [Navigator.nvim](https://github.com/numToStr/Navigator.nvim): There is currently a PR which adds support for Zellij, until then [this fork](https://github.com/dynamotn/Navigator.nvim) can be used. The advantage of Navigator.nvim is that it will detect if it is running in Zellij and Tmux and will work with both without changing any config.

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

