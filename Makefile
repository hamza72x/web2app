run_template:
	@cd template; cargo tauri dev

run:
	@cargo build && ./target/debug/web2app args -n Notion -u https://www.notion.so/

.PHONY: run_template run
