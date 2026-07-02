run:
	cargo build --release && rm -f cookedhcp && mv target/release/cookedhcp . && sudo ./cookedhcp
debug:
	cargo build && rm -f cookedhcp && mv target/debug/cookedhcp . && sudo ./cookedhcp