# cookedhcp
## Setup
You **need** to create a .env file in the root folder with the following variables:
- SIADDR: the router's address
- YIADDR: the client's assigned address (you choose it!)
- IP_LIMITED_BROADCAST: the network's limited broadcast ip
- IP_DNS: it is what you think it is

If you don't want to manually specify a DNS you could set it to "NO" and it will be ignored.

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