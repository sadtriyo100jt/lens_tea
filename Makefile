BINARY_NAME = lens
TARGET = target/release/$(BINARY_NAME)
INSTALL_LOCATION = /usr/local/bin/$(BINARY_NAME)
IMAGES_LOCATION = ~/.$(BINARY_NAME)

.PHONY: all build install uninstall clean

all:
	cargo build --release

install: all
	cp $(TARGET) $(INSTALL_LOCATION)
	mkdir -p $(IMAGES_LOCATION)

uninstall:
	rm $(INSTALL_LOCATION)
	rm -rf $(IMAGES_LOCATION)

clean:
	pkill ruin
	rm $(INSTALL_LOCATION)
