# cookedhcp
## Setup
You **need** to create a .env file in the root folder with the following variables:
- SIADDR
- YIADDR
- IP_LIMITED_BROADCAST

You _could_ also provide a custom DNS in a variable named as follows:
- IP_DNS

For example:
```
IP_DNS="1.1.1.1"
SIADDR="192.168.1.1"
YIADDR="192.168.1.100"
IP_LIMITED_BROADCAST="192.168.1.255"
```

## Start cookedhcp!
After the setup is done you can build and run with just `make`

Otherwise, manually: `cargo build --release && rm -f cookedhcp && mv target/release/cookedhcp . && sudo ./cookedhcp`