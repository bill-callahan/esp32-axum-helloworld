# [Rust] Tokio/Axum running on esp32!
After going down various and several rabbitholes, trying to get my little ESP32 to serve a webpage, I stumbled across [Jasta's esp32-tokio-demo](http://github.com/jasta/esp32-tokio-demo) which did exactly what I was trying to do:
- [x] - Get my ESP32 to startup wifi, and connect to it
- [x] - [Bonus] do it asynchronously
- [x] use Tokio's TcpListener to bind to the wifi address
- [x] - startup a webserver (various out there like Axum, Actix, Tide, etc but all heavily use Tokio, which until now wasn't ESP32 friendly)

This repo demonstrates a working hello world utilizing upstreamed [tokio support for esp32](https://github.com/tokio-rs/tokio/issues/5867).  
The tokio executor and I/O reactor are both working fully with no known gotchas.  Third party libraries utilizing tokio can also be used freely, such as [coap-server-rs](https://github.com/jasta/coap-server-rs).  I elected to do an [axum](https://docs.rs/axum) server, in this repo.

## Quickstart

Recommended that you use an ESP32C3 as upstream Rust support is currently better, though if you do choose any other in the ESP32 family be sure to check out the guides in the [Rust on ESP Book](https://esp-rs.github.io/book/installation/index.html).
- [x] This repo works on an ESP32
```
git clone https://github.com/bill-callahan/esp32-axum-helloworld your-project-name
cd your-project-name
code .
#  src/main.rs # <-- edit the WIFI_SSID/WIFI_PASS variables!
cargo run --release   <-- greatly reduces size of binary, and takes awhile to compile first-time
```

After the board connects to Wi-Fi, you can test that things are working with:

- A web browser - point to http://{ip-obtainded-from-wifi}:3000
- ThunderClient (vscode extension) - 'New Request" --> http://{ip-addr}:3000/ --> "Send" -- you should see an HTML Response and a Status: 200 OK

Try: / - simple Hello World response
     /clock - a static page that displays the time (in browser)

You may optionally use `cargo run --target xtensa-esp32-espidf` or any of the other supported targets which should work provided that you followed the [esp-rs](https://github.com/esp-rs) installation instructions above.
