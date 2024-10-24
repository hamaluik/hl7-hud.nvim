# hl7-hud.nvim

## Description

This plugin is a simple plugin to help you to work with HL7 messages. It plugs
into lualine to show where in an HL7 message you are, and it provides some
commands to help you navigate the message.

![Demo](./demo/demo.gif)

## Installation

### hl7-hud Binary

In order for this to work, you must compile the `hl7-hud` binary, which is a Rust
binary that does the grunt work for this plugin. You can find the source code
for the binary in the `hl7-hud` directory of this repository. You can compile
the binary by running `cargo build --release` in that directory.

Either install it in your path (1), or set the `path` option in the setup function
to the path of the binary (2).

1. Install the binary in your path:
   ```sh
   cd hl7-hud
   cargo install --path .
   ```
2. Set the `path` option in the setup function to the path of the binary:
   ```lua
   require('hl7-hud').setup({
       path = "/path/to/hl7-hud",
   })
   ```

### lazy.nvim

```lua
return {
    {
        dir = "~/Documents/projects/hl7-hud.nvim",
        name = "hl7-hud",
        config = function()
            require('hl7-hud').setup({
                path = "/path/to/hl7-hud",
            })

            vim.api.nvim_set_keymap("n", "<leader>hq", ":lua require('hl7-hud').query_input()<CR>", { silent = true })
            vim.api.nvim_set_keymap("n", "<leader>hp", ":lua print(require('hl7-hud').cursor_pos())<CR>", { silent = true })
            vim.api.nvim_set_keymap("n", "<leader>ht", ":lua print(require('hl7-hud').cursor_timestamp())<CR>", { silent = true })
        end
    },
    {
        "nvim-lualine/lualine.nvim",
        dependencies = {
            "nvim-tree/nvim-web-devicons",
            "hl7-hud",
        },
        config = function()
            local hl7_hud = require('hl7-hud')

            require('lualine').setup({
                extensions = { hl7_hud.lualine_ext },
            })
        end,
    },
}

```

## Usage

### Commands

- `lua require('hl7-hud').query('<your HL7 query>')`: Move to the part of the
message that matches the query.
- `lua require('hl7-hud').query_input()`: Same as `hl7_query`, but prompts
for the query.
- `lua require('hl7-hud').cursor_pos()`: Get the current position of the cursor
in the message.
- `lua require('hl7-hud').parse_timestamp('<HL7 timestamp>')`: Parse the timestamp
  that is passed as an argument.
- `lua require('hl7-hud').cursor_timestamp()`: Parse the timestamp that is
  under the cursor

## TODO

* [ ] Add more commands to navigate the message (e.g. move to the next segment, field, etc).
* [ ] Add more information to the lualine extension.
* [ ] Switch to [nvim-oxi](https://crates.io/crates/nvim-oxi) ?
