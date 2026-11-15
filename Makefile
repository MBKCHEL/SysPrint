BIN= sysprint
DEST= /usr/local/bin/$(BIN)
SUDO= $(shell command -v doas >/dev/null 2>&1 && echo doas || echo sudo)

.PHONY: build install clean

build:
	cargo build --release

install: build
	$(SUDO) install -m 755 target/release/$(BIN) $(DEST)
	@echo "Installed to $(DEST)"

clean:
	cargo clean
