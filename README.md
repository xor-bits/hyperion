# Hyperion

An operating system

## Why the name?

Idk, I asked ChatGPT to give it a name:

> Hyperion is a moon of Saturn that is known for its irregular shape and its rapid rotation.
> It was named after the Titan Hyperion, a figure from Greek mythology who was the father of the sun,
> the moon, and the dawn.
>
> I chose the name Hyperion for your operating system because it sounds modern and powerful, and it
> has a connection to the sun and the moon, which could be seen as representing the dual nature of an
> operating system as both a hardware-level and a software-level entity. Additionally, the irregular
> shape of Hyperion and its rapid rotation could be seen as metaphors for the flexibility and speed
> of a modern operating system.

## How do I run it?

### In QEMU

It is as simple as just:

```bash
# normal
make run
# without kvm
make KVM=false run
# with uefi
make UEFI=true run
```

### On HW?

Please don't

## Dependencies

Packages for Arch:
```bash
pacman -Syu make qemu-system-x86 xorriso jq
```

Rust:
```bash
# rustup:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# bare metal target
rustup target add x86_64-unknown-none
# nightly compiler
rustup toolchain install nightly
```

## Example image(s)

![image](https://github.com/xor-bits/hyperion/assets/42496863/cde71ecf-825f-4e5b-9a32-f204ffbef6e7)

![image](https://github.com/xor-bits/hyperion/assets/42496863/76460288-d6d7-47de-ab1b-399d0a91dc80)

## Font

The font used contains the first 256 bitmap glyphs from [GNU Unifont](http://unifoundry.com/)
