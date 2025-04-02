PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/man
INSTALL = install -m 755
MANINSTALL = install -m 644
CONFIG_DIR = $(HOME)/.config/stoac
ZSH_FILE = stoac.zsh

install: release
	sudo $(INSTALL) target/release/stoac $(BINDIR)/stoac
#	$(MANINSTALL) mytool.1 $(MANDIR)/man1/mytool.1 -> for man pages later

release: src/main.rs
	@echo "Building the Rust binary..."
	cargo build --release

clean:
	rm -f target/

uninstall:
	sudo rm -i $(BINDIR)/stoac
