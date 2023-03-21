# Tauri App

This is an application based on Tauri, utilizing Rust and WebUI.


## UDP送受信テスト

受信
```
$ cargo run --bin socket recv 127.0.0.1:5000
```

送信
```
$ cargo run --bin socket send 127.0.0.1:5000
```