PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/man
INSTALL = install -m 755
MANINSTALL = install -m 644
CONFIG_DIR = $(HOME)/.config/stoac
ZSH_FILE = stoac_zsh.zsh

install: release
	sudo $(INSTALL) target/release/stoac $(BINDIR)/stoac
#	$(MANINSTALL) mytool.1 $(MANDIR)/man1/mytool.1 -> for man pages later

# Create the config directory if it doesn't exist
$(CONFIG_DIR):
	@echo "Creating config directory $(CONFIG_DIR)..."
	mkdir -p $(CONFIG_DIR)

zsh_plugin: install $(CONFIG_DIR)
	@echo "Copying the zsh script"
	cp $(ZSH_FILE) $(CONFIG_DIR)
	@echo "To make the zsh_plugin work add the following to your .zshrc:"
	@echo "source '$(CONFIG_DIR)/stoac_zsh.zsh'"

release: src/main.rs
	@echo "Building the Rust binary..."
	cargo build --release

clean:
	rm -f target/

uninstall:
	sudo rm -i $(BINDIR)/stoac
