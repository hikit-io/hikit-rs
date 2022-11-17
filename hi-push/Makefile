cli-wasm:
	cargo build --lib --features client --target wasm32-unknown-unknown
cli:
	cargo build --lib --features client


build:
	cargo build --lib

fcm:
	cargo build --lib --features fcm
apns:
	cargo build --lib --features apns
wecom:
	cargo build --lib --features wecom
xiaomi:
	cargo build --lib --features xiaomi
huawei:
	cargo build --lib --features huawei
email:
	cargo build --lib --features email

all:fcm apns wecom xiaomi huawei email
	echo 'all'
