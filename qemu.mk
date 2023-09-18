# config tables
MACH_x86_64        ?= -machine q35
MACH_riscv64       ?= -machine virt
CPU_x86_64         ?= -cpu qemu64,+rdrand,+rdseed
CPU_riscv64        ?= -cpu rv64
KVM_x86_64_true    ?= -enable-kvm
GDB_true           ?= -s -S
DEBUG_1            ?= -d guest_errors
DEBUG_2            ?= -d guest_errors,int
DEBUG_3            ?= -d guest_errors,int,cpu_reset
UEFI_x86_64_true   ?= -bios /usr/share/ovmf/x64/OVMF.fd
#VGA                ?= -device virtio-vga
#-vga virtio
#VGA                ?= -device VGA
#-vga std

# qemu config
MEMORY             ?= 256m
CPUS               ?= 4
MACH               ?= ${MACH_${ARCH}}
UEFI               ?= false

QEMU_FLAGS         ?= ${KVM_${ARCH}_${KVM}} \
					  ${GDB_${GDB}} \
					  ${DEBUG_${DEBUG}} \
                      ${MACH} \
                      ${CPU_${ARCH}} \
					  -smp ${CPUS} \
                      -m ${MEMORY} \
					  -nographic \
					  ${UEFI_${ARCH}_${UEFI}} \
					  ${VGA} \
					  -serial mon:stdio
					  #-no-reboot
					  #-rtc base=localtime

# -no-shutdown

QEMU_RUN_FLAGS     ?=
QEMU_TEST_FLAGS    ?= -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
					  -display none

QEMU_DRIVE         := -drive format=raw,file
QEMU_RUN_x86_64    := ${QEMU_DRIVE}="${HYPERION}"
QEMU_TEST_x86_64   := ${QEMU_DRIVE}="${HYPERION_TESTING}"
QEMU_RUN_riscv64   := -kernel "${KERNEL}"
QEMU_TEST_riscv64  := -kernel "${KERNEL_TESTING}"
QEMU_RUN           := ${QEMU_RUN_${ARCH}}
QEMU_TEST          := ${QEMU_TEST_${ARCH}}

# TODO: multiboot1 direct kernel boot

# qemu normal run
run: ${HYPERION}
	@echo -e "\n\033[32m--[[ running Hyperion in QEMU ]]--\033[0m"
	${QEMU} ${QEMU_FLAGS} ${QEMU_RUN_FLAGS} ${QEMU_RUN}

# run tests in qemu
test: ${HYPERION_TESTING}
	@echo -e "\n\033[32m--[[ running Hyperion-Testing in QEMU ]]--\033[0m"
	${QEMU} ${QEMU_FLAGS} ${QEMU_TEST_FLAGS} ${QEMU_TEST};\
	[ $$? -ne 33 ] && exit 1;\
	exit 0
