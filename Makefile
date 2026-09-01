OVMF_PATH ?= /usr/share/edk2/ovmf/OVMF_CODE.fd
TARGET := x86_64-unknown-uefi

.PHONY: build-ada build-x86_64 run-x86_64 clean

build-ada:
	mkdir -p ada/time/obj ada/memory/obj
	gcc -c ada/time/src/time.adb -O2 -g0 -gnatp -mno-red-zone -fno-PIC -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-strict-aliasing -o ada/time/obj/time.o
	gcc -c ada/memory/src/memory.adb -O2 -g0 -gnatp -mno-red-zone -fno-PIC -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-strict-aliasing -o ada/memory/obj/memory.o

	objcopy -I elf64-x86-64 -O pe-x86-64 ada/time/obj/time.o ada/time/obj/time.obj
	objcopy -I elf64-x86-64 -O pe-x86-64 ada/memory/obj/memory.o ada/memory/obj/memory.obj

build-x86_64: build-ada
	RUSTFLAGS="\
		-C link-arg=$(CURDIR)/ada/time/obj/time.obj \
		-C link-arg=$(CURDIR)/ada/memory/obj/memory.obj" \
		cargo +nightly build --package ToPluto-kernel --target $(TARGET)
		
	rm -rf target/esp
	mkdir -p target/esp/EFI/BOOT
	cp target/$(TARGET)/debug/ToPluto-kernel.efi target/esp/EFI/BOOT/BOOTX64.EFI

run-x86_64: build-x86_64
	qemu-system-x86_64 \
		-m 2G \
		-bios $(OVMF_PATH) \
		-drive format=raw,file=fat:rw:target/esp \
		-serial stdio

clean:
	rm -rf target ada/time/obj ada/memory/obj