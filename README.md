niri-workspace-column-indicator
===============================

A Niri workspace-column-indicator module for Waybar. Displays which column in a workspace has focus.

For example, here a tiled window on column 4 has focus:

<div><img src="screenshot.png" alt="screenshot"/></div>


Installation
------------

The Waybar module installation requires [Rust](https://www.rust-lang.org/).

To build and install the custom Waybar module, for example, in folder ~/.local/bin (in $PATH), run:

    $ cargo install --git=https://github.com/willemw12/niri-workspace-column-indicator.git --no-track --root=$HOME/.local

Or the same, but download separately:

    $ git clone https://github.com/willemw12/niri-workspace-column-indicator.git
    $ cargo install --no-track --path=./niri-workspace-column-indicator --root=$HOME/.local

Or build and then copy the program to the standard Waybar scripts folder (outside $PATH):

    $ cargo build --release
    $ mkdir -p ~/.config/waybar/scripts
    $ cp -a target/release/niri-workspace-column-indicator ~/.config/waybar/scripts/

It also requires "Fontawesome icons" to display the characters, for example the icons in the "Fira Code Nerd" font.


Configuration
-------------

Here is a Waybar module configuration example in file ~/.config/waybar/config:

```
"modules-left": ["niri/workspaces", "custom/niri-columns"],
"custom/niri-columns": {
    "exec": "niri-workspace-column-indicator",
    // "exec": "~/.config/waybar/scripts/niri-workspace-column-indicator",
    "return-type": "text",
    "format": "            {}"
},
```


Alternative
-----------

File [`./extra/niri-workspace-column-indicator.sh`](./extra/niri-workspace-column-indicator.sh) does the same as the Rust program but is written in Bash.


License
-------

GPL-3.0-or-later


Link
----

[GitHub](https://github.com/willemw12/niri-workspace-column-indicator)
