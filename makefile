PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/man
INSTALL = install -m 755
MANINSTALL = install -m 644

install: release
	sudo $(INSTALL) target/release/stoac $(BINDIR)/stoac
#	$(MANINSTALL) mytool.1 $(MANDIR)/man1/mytool.1 -> for man pages later

release: src/main.rs
	cargo build --release

clean:
	rm -f target/
