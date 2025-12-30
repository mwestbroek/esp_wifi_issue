## ESP32 WiFi issue
How to reproduce:

- Build and flash with `cargo run --release --features esp32` (target is ESP32-WROOM-32E)
  - Ensure environment variables `WIFI_SSID` and `WIFI_PASSWORD` are set for an available WiFi network
- Wait for it to connect and get an IP address
- Ping it like `ping <IP>` and/or curl it, something like
```
while true        
do
    curl http://<IP>:3502
    sleep .5
done
```

After some time, ping times out (e.g. `Request timeout for icmp_seq 34`) and curl too (e.g. `curl: (7) Failed to connect to <IP> port 3502 after 11 ms: Couldn't connect to server`). Presumably packets are being dropped.
