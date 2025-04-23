#!/bin/bash
cd \$(dirname \$0)/../initramfs
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../boot/initramfs-secure.img
echo "[+] Initramfs packed."
