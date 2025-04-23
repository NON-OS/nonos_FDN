<pre> ███╗   ██╗ ██████╗  ███╗   ██╗ ██████╗ ███████╗
 ████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝
 ██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗
 ██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║
 ██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║
 ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
</pre>

<pre lang="markdown">
# NØNOS — A Self-Sovereign Operating System for Encrypted Infrastructure, Pseudonymous Contribution & Trustless Computation

---

## TL;DR  
NØNOS is a zero-trust, stateless, terminal-native OS designed for anonymous operations, encrypted communication, and verifiable computation. It is built for real-world deployment in adversarial environments — from decentralized labs to censorship zones.

---

## LIVE LINKS  
- Website: [https://nonos-tech.xyz](https://nonos-tech.xyz)  
- Twitter/X: [https://twitter.com/nonos_eth](https://twitter.com/nonos_eth)  

---

## Project Snapshot  

| Feature                  | Status                                |
|--------------------------|----------------------------------------|
| Custom Kernel            | ✅ Compiling & Bootable via QEMU        |
| Initramfs System         | ✅ Custom logic, terminal-only, minimal footprint |
| ISO Build                | ⏳ ISO structure and boot process working |
| Root Filesystem          | ⏳ Large .img not in repo (external download) |
| Relay Networking Layer   | ⚙️ Mesh system WIP via Anyone SDK        |
| CLI Interface (`nonosctl`) | ⚙️ Prototype commands ready            |
| Plugin Module System     | ⚙️ CLI-integrated sandbox plugins in dev |
| Tokenomics Layer         | ⏳ Model defined, protocol design ready |
| DAO & Governance         | ⚙️ Scoring + zk-attestation flow in progress |

---

## Folder Structure  nonos-os/
├── boot/             # Compiled kernel (bzImage), initrd, bootloader links  
├── configs/          # GRUB configs, system boot params, kernel flags  
├── docs/             # Documentation, dev notes, whitepaper drafts  
├── initramfs/        # Custom init logic, BusyBox scripts, /init  
├── iso/              # Live ISO output structure and supporting files  
├── kernel/           # Linux kernel source and .config used for NØNOS  
├── modules/          # Optional or experimental kernel modules  
├── packages/         # Custom .deb packages or local app builds  
├── scripts/          # QEMU launchers, ISO builders, helper tools  
├── nonos-rootfs.img  # (excluded) Root filesystem image — see download link  
├── README.md         # You’re here  
└── WHITEPAPER.md     # System architecture, tokenomics, governance  

---

## Getting Started

### Requirements  
- qemu, cpio, gzip, nasm  
- make, gcc, binutils  
- Optional: nix for reproducible builds  

### Build & Boot (Dev Mode)cd kernel
make defconfig
make -j$(nproc)

cd ../initramfs
chmod +x init
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../boot/initrd.img

cd ../scripts
./launch-qemu.sh

---

## Core Philosophy  

NØNOS is not about convenience — it’s about control.  
No cloud callbacks. No browser stack. No fingerprinting.

It is an OS where:  
- You are anonymous by default.  
- You control your keys, compute, and connection.  
- Every interaction is cryptographically signed or sandboxed.

---

## License  

This project uses a dual-license model:

1. Codebase:  
Licensed under AGPL-3.0 — guarantees public sharing of modifications and protects against closed forks.

2. Branding & Docs (ASCII, README, Manifesto):  
Licensed under CC BY-NC-SA 4.0  
(Attribution–NonCommercial–ShareAlike)

> Note: The NØNOS name and logo are reserved under a non-commercial, open-source contribution license.  
> You are free to fork, remix, and deploy — but you may not resell or privatize the brand.

---

## Contribute  

We welcome pseudonymous contributors.  
You don’t need a GitHub profile — you need a GPG key. *(SOON)*  

### Coming Soon:
- nonosctl submit for module pushes  
- ZK-based contributor registry  
- Plugin publishing system  

---

## Conclusion  

The full vision, architecture, economic design, governance model, and threat model are described in the whitepaper and website.

NØNOS isn’t a product.  
It’s a protocol for permissionless sovereignty.
</pre>
