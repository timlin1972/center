# center

## How to add a new plugin

### Create a new repo on github

https://github.com/timlin1972?tab=repositories

- Add a README file
- Add .gitignore
  - Rust
- Choose a license
  - MIT License

### Clone under ./center

```
git clone git@github.com:timlin1972/tln_command.git
cargo init tln_command
```

Remove the last section of ./.gitignore

### Some modifications

- Copy src/lib.rs from other working plugin and modify
- Remove src/main.rs
- Copy part of Cargo.toml from other working plugin

  ```
  [dependencies]
  # color console
  anstream = "0.6.15"
  owo-colors = "4.1.0"

  crossbeam-channel = "0.5.13"

  common = { path = "../common" }
  ```

### Under release folder

- Modify `packages.sh`
- Modify `release.sh`

### Check-in

## CLI

### exit

### send plugin {plugin} {action} {data}

#### log

- send plugin log report myself
- send plugin log add 'New log'
- send plugin log clear all

#### mqtt

- send plugin mqtt report myself  
  send `onboard` info to broker

- send plugin mqtt report '{"topic": "tln/center_test/send", "payload": "exit"}'

#### sysinfo

- report myself
- report status

## CLI Examples

### Report local device sysinfo to broker

- send plugin sysinfo report myself

### Ask a remote device to exit

- send plugin mqtt report '{"topic": "tln/center_test/send", "payload": "exit"}'
- send plugin mqtt report '{"topic": "tln/pi5/send", "payload": "exit"}'

### Ask a remote device to report sysinfo status to me

- send plugin mqtt report '{"topic": "tln/center_test/send", "payload": "send plugin sysinfo report status"}'

### Ask a remote device to report sysinfo status to mqtt broker

- send plugin mqtt report '{"topic": "tln/pi5/send", "payload": "send plugin sysinfo report myself"}'

## mqtt

- tln/{name}/onboard
  - 0/1
- tln/{name}/uptime
  - u64
- tln/{name}/hostname
  - string
- tln/{name}/os
  - string
- tln/{name}/send
  - string (payload)
