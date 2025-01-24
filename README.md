# Rust Hello World for the MilkV-Duo

This project aims to implement a HID on the MilkV-Duo.
The MilkV-Duo doesnt have a GPU or any OpenGL/... acceleration.
Cause of the lack, drawing to a screen is running 100% on the CPU not accelerated.
Plus - most graphic librarys wont work without OpenGL/Vulkan/... .
Thats why this project started. I wanted a way to simply directly draw to the framebuffer device without X11 or any X11 related hacks.

It uses the embedded-graphics crate - keep in mind that graphic operations are rather simple compared to software stacks like lvgl (please someone make cross compiling work for the lvgl bindings).

Background:
I tryed using the SDL2 and lvgl-bindings crate - but i failed cross compiling it.
If someone succeeds cross-compiling them, please create an issue in this repo or write me an email.

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

# Known errors/tricks

## Connect to wifi
Source: https://community.milkv.io/t/connect-to-wi-fi-on-duo-s/1540

Start wpa_supplicant with:

```
echo "update_config=1" >> /etc/wpa_supplicant.conf
wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant.conf
```
At this point run:
```
wpa_cli -i wlan0
```
Use the scan and scan_results commands to see the available networks:
```
> scan
OK
<3>CTRL-EVENT-SCAN-STARTED 
<3>CTRL-EVENT-SCAN-RESULTS 
> scan_results
bssid / frequency / signal level / flags / ssid
11:22:33:44:55:66       5745    -78     [WPA2-PSK-CCMP][ESS]    MYSSID-5G
11:22:33:44:55:67       2427    -85     [WPA2-PSK-CCMP][ESS]    MYSSID
To associate with MYSSID-5G, add the network, set the credentials and enable it:

> add_network
1
> set_network 1 ssid "MYSSID-5G"  
OK
> set_network 1 psk "password"
OK
> enable_network 1
```

If the SSID does not have password authentication, you must explicitly configure the network as keyless by replacing the command set_network 1 psk "password" with set_network 1 key_mgmt NONE.

Finally save this network in the configuration file and quit wpa_cli:
```
> save_config
OK
> quit
```

```
[root@milkv-duo]~# ip a
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP qlen 1000
    link/ether 66:69:54:69:1c:2c brd ff:ff:ff:ff:ff:ff
    inet 169.254.206.92/16 brd 169.254.255.255 scope global noprefixroute eth0
       valid_lft forever preferred_lft forever
3: usb0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP qlen 1000
    link/ether 7e:eb:9c:48:33:3c brd ff:ff:ff:ff:ff:ff
    inet 192.168.42.1/24 brd 192.168.42.255 scope global usb0
       valid_lft forever preferred_lft forever
4: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP qlen 1000
    link/ether 88:00:33:77:f1:0f brd ff:ff:ff:ff:ff:ff
    inet 192.168.0.175/24 brd 192.168.0.255 scope global dynamic noprefixroute wlan0
       valid_lft 84829sec preferred_lft 74029sec
[root@milkv-duo]~# ping baidu.com
PING baidu.com (39.156.66.10): 56 data bytes
64 bytes from 39.156.66.10: seq=0 ttl=49 time=46.170 ms
64 bytes from 39.156.66.10: seq=1 ttl=49 time=53.166 ms
64 bytes from 39.156.66.10: seq=2 ttl=49 time=54.292 ms
```
Auto connect on startup

```
cat >> /mnt/system/duo-init.sh <<END
# Auto connect to WIFI on startup
wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant.conf
END
reboot
```
## (WiFi/LAN) MAC changes everytime

```
echo "pre-up ifconfig eth0 hw ether 78:01:B3:FC:E8:55" >> /etc/network/interfaces
echo "pre-up ifconfig wlan0 hw ether 78:01:B3:FC:E8:55" >> /etc/network/interfaces
```

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