# makefile for rsh

BINARY = rsh
INSTALL = /usr/local/bin
SHELL_LIST = /etc/shells
CARGO = cargo
SUDO = sudo # switch to doas if you want

.PHONY: build debug install clean remove

build:
	$(CARGO) build --release

debug:
	$(CARGO) build

install:
	@echo "installing $(BINARY) to $(INSTALL)..."
	$(SUDO) install -m 755 target/release/$(BINARY) $(INSTALL)/$(BINARY)
	@grep -qxF "$(INSTALL)/$(BINARY)" $(SHELL_LIST) \
		|| echo "$(INSTALL)/$(BINARY)" | $(SUDO) tee -a $(SHELL_LIST) > /dev/null
	@echo "done. $(BINARY) installed and registered in $(SHELL_LIST)"

clean:
	$(CARGO) clean

remove:
	@echo "removing $(BINARY)..."
	$(SUDO) rm -f $(INSTALL)/$(BINARY)
	$(SUDO) sed -i "\|$(INSTALL)/$(BINARY)|d" $(SHELL_LIST)
	@echo "done. $(BINARY) removed"

