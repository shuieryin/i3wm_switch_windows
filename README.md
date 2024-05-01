# i3wm switch windows

Switching windows across workspaces and monitors

## Installation

```bash
  cargo install i3wm_switch_windows
```

Add the following lines to ~/.i3/config
```bash
bindsym $mod+Tab exec --no-startup-id i3wm_switch_windows
bindsym $mod+Shift+Tab exec --no-startup-id i3wm_switch_windows backward
```