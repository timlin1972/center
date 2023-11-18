[Unit]
Description=Rust Server
After=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/bin/zsh /home/timlin/rust/center/server.sh
WorkingDirectory=/home/timlin/rust/center
User=timlin
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target