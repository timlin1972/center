# center

## Port usage

| Port  | Usage             |
| ----  | ----------------- |
| 9759  | main http         |
| 9760  | main https        |
| 9761  | plugin wol        |
| 9762  | plugin sys stat   |
| 9763  | plugin shutdown   |
| 9764  | plugin screen     |
| 9765  | plugin log        |

## To add a plugin

### run.sh
Add `progs`

### server.sh
Add `progs`

### Cargo.toml
Add `members`

### cargo init
`cargo init --bin plugin_xxx`

## Run
`./run.sh`

# systemd
sudo vim /etc/systemd/system/rust-server.service
sudo systemctl start rust-server.service
sudo systemctl enable rust-server.service