OVMF_PATH ?= /usr/share/edk2/ovmf/OVMF_CODE.fd
TARGET_UEFI := x86_64-unknown-uefi

.PHONY: build-ada build-x86_64 run-x86_64

build-ada:
	cd ada/time && alr build --release
	objcopy -O pe-x86-64 ada/time/obj/time.o ada/time/obj/time_coff.o
	rm -f ada/time/lib/libada_time.a
	ar rcs ada/time/lib/libada_time.a ada/time/obj/time_coff.o

build-x86_64: build-ada
	RUSTFLAGS="-L native=$(CURDIR)/ada/time/lib -l static=ada_time" \
		cargo +nightly build --package skidbladnir-kernel --target $(TARGET_UEFI)
	rm -rf target/esp
	mkdir -p target/esp/EFI/BOOT
	cp target/$(TARGET_UEFI)/debug/skidbladnir-kernel.efi target/esp/EFI/BOOT/BOOTX64.EFI

run-x86_64: build-x86_64
	qemu-system-x86_64 \
		-m 2G \
		-bios $(OVMF_PATH) \
		-drive format=raw,file=fat:rw:target/esp \
		-serial stdio