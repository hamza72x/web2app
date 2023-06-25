run_template:
	@cd template; RUST_BACKTRACE=1 cargo tauri dev

run:
	@cargo build && RUST_BACKTRACE=1 ./target/debug/web2app args -n Notion -u https://www.notion.so/

.PHONY: run_template run

