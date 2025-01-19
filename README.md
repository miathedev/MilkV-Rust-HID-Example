# Rust Hello World for the MilkV-Duo

## Sources
Based on: https://barretts.club/posts/i-got-a-milkv-duo/

## Build
Just run `./compile.sh` once for the initial setup and first build. The script will also install the toolchain required.

## Add a display to the MilkV
Use the following source:

https://milkv.io/docs/duo/resources/spilvgl#bring-up-the-spi-display
However, tho this example is made for the ST7789V - you can easilly add any by linux suported graphic display.

Heres another example:
https://gist.github.com/miathedev/f5a502eec78e4bc113cdb82935824a4e

Its basically always the same. Most important part on the MilkV is patching the fbtft_request_one_gpio function.

## ToDo
* Remote Debugging
* Audio
* Sprites
* MilkV Mailbox
* MilkV GPIO

# Known errors
## Colors are inverted

The display in this code uses 2 bytes per pixel. (BRG565)
Yours might use different color coding.
Use `fb-set` to get the bits-per pixel, fetch your displays datasheet and change the code in 
`frame_buffer_display.rs` accordingly.

## I dont see anything

Check your display connections and you might want to install `BR2_PACKAGE_FB_TEST_APP` into your buildroot to test the framebuffer device.

## not found

```
[root@milkv-duo]~# ./hello_riscv 
-sh: ./hello_riscv: not found
```

Create an symlink to correct the interpreter path used in the binary
```
ln -sf /lib/ld-musl-riscv64v0p7_xthead.so.1 /lib/ld-musl-riscv64.so.1
```

## sftp-server not found
```
scp target/riscv64gc-unknown-linux-musl/release/hello_riscv root@192.168.XXX.YYY:~
root@192.168.XXX.YYY's password: 
sh: /usr/libexec/sftp-server: not found
scp: Connection closed
```

Your ssh client is newer than on the host. Use the legacy options:
```
scp -O target/riscv64gc-unknown-linux-musl/release/hello_riscv root@192.168.XXX.YYY:~
```

## Why does my MilkV get a new MAC address and IP everytime

Yet unsure, i guess they didnt pay MACs.
Which could be why the MAC addr. are super random everytime.