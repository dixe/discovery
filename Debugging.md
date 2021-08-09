# Openocd debugging server:

In powershell in `C:\Users\PC\AppData\Local\Temp>` run `openocd -s C:\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`

# itm read output text file
In linux terminal also in `C:\Users\PC\AppData\Local\Temp>` run `itmdump -F -f itm.txt`

# Flash and debug
I project root run `cargo run`
