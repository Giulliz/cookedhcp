# cookedhcp
## Setup
You **need** to create a .env file in the root folder with the following variables:
- SIADDR: the router's address
- YIADDR: the client's assigned address (you choose it!)
- IP_LIMITED_BROADCAST: the network's limited broadcast ip

If you wish, you _could_ also set a custom DNS by creating a variable named IP_DNS. 
If you don't, Cloudflare DNS will be set instead.

A valid sample config:
```
IP_DNS="8.8.8.8"
SIADDR="192.168.1.1"
YIADDR="192.168.1.100"
IP_LIMITED_BROADCAST="192.168.1.255"
```

## Start cookedhcp!
After the setup is done you can build and run with just `make`

Otherwise, manually: `cargo build --release && rm -f cookedhcp && mv target/release/cookedhcp . && sudo ./cookedhcp`