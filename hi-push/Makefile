
features := fcm apns email wecom huawei xiaomi
all_features := fcm,apns,email,wecom,huawei,xiaomi


build:
	cargo build --lib

client-wasm:
	cargo build --lib --features client --target wasm32-unknown-unknown
client:
	cargo build --lib --features client


%-feature:
	cargo build --lib --features $*
	@echo [feature] $@

feature:$(addsuffix -feature,$(features))
	@echo [feature] $^


%-http-api:
	cargo build --lib --features http-api,mongo,$*
	@echo [http-api] $@

http-api:$(addsuffix -http-api,$(features))
	@echo [http-api] $^


http-api-all:
	cargo build --lib --features http-api,mongo,$(all_features)

%-test:
	cargo test --features $*
	@echo [test] $@

test:$(addsuffix -test,$(features))
	@echo [test] $^