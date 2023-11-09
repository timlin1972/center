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

## To add a plugin

### run.sh
Add `progs`

### Cargo.toml
Add `members`

### cargo init
`cargo init --bin plugin_xxx`

## Run
`./run.sh`