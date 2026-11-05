# Burnout

A Rust CLI prompt that supports Bash, Zsh, Git Bash, PowerShell, and Cmd shells.

## Installation

This needs to be addressed.

## Setup

Once Burnout is installed, the call to Burnout for a shell must be setup.

### Bash

For Bash, the following must be added to .bashrc:

```bash
  eval "$(burnout bash initialise)"
```

For transient prompts, Ble.sh must be installed and have the following flag added to .bashrc:

```bash
  bleopt prompt_ps1_transient=always
```

### Zsh

For Zsh, the following must be added to .zshrc:

```zsh
  eval "$(burnout bash initialise)"
```

For transient prompts, Ble.sh must be installed and have the following flag added to .zshrc:

```zsh
  bleopt prompt_ps1_transient=always
```

### Git Bash

For Git Bash, the following must be added to .bashrc:

```bash
  eval "$(burnout bash initialise)"
```

For transient prompts, Ble.sh must be installed and have the following flag added to .bashrc:

```bash
  bleopt prompt_ps1_transient=always
```

### PowerShell

For PowerShell, the following must be added to Microsoft.PowerShell_profile.ps1:

```powershell
  Invoke-Expression (&burnout powershell initialise)
```

### Cmd

For Cmd, clink must be installed and following must be added to a burnout.lua, added in the clink script folder:

```cmd
  load(io.popen('burnout cmd initialise'):read('*a'))()
```

## Configuration

This needs to be addressed.

## Plugins

This needs to be addressed.
