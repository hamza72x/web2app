template_mod="src/generated/mod.rs"

build_template:
	@echo '// Auto generated by $$ make build_template' > ${template_mod} && \
	echo "// Don't edit this file by HAND, edit the ../template directory instead." >> ${template_mod} && \
	echo "// Then run $$ make build_template" >> ${template_mod} && \
	echo "\npub const MAIN_RS: &str = \"$(shell cat template/src/main.rs | base64)\";" >> ${template_mod} && \
	echo "\npub const CARGO_TOML: &str = \"$(shell cat template/Cargo.toml | base64)\";" >> ${template_mod} && \
	echo "\npub const CARGO_LOCK: &str = \"$(shell cat template/Cargo.lock | base64)\";" >> ${template_mod} && \
	echo "\npub const APP_CONFIG: &str = \"$(shell cat template/src/app_config.rs | base64)\";" >> ${template_mod} && \
	echo "\npub const APP_DATA: &str = \"$(shell cat template/src/app_data.rs | base64)\";" >> ${template_mod} && \
	echo "\npub const APP_MENU: &str = \"$(shell cat template/src/app_menu.rs | base64)\";" >> ${template_mod} && \
	echo "\npub const JS_SCRIPTS: &str = \"$(shell cat template/src/js_scripts.rs | base64)\";" >> ${template_mod}

run_template:
	@cd template; cargo run

run:
	@cargo build && ./target/debug/web2app args -n Notion -u https://www.notion.so/

.PHONY: build_template run_template run
