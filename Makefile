IMAGE=BOOT.BIN
BOOTLOADER=target/armv7a-none-eabi/release/bootloader

all:
	cargo build --release
	cp $(BOOTLOADER) $(BOOTLOADER).elf
	bootgen -arch zynq -image output.bif -o $(IMAGE) -w on
