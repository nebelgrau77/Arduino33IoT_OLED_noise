# White noise on SSD1306 OLED display

Based on an example from https://github.com/jamwaffles/ssd1306

![random noise](whitenoise.gif)

Ported to Arduino Nano 33 IoT (Microchip-Atmel SAMD21).

Sends random raw data to the display, emulating an old untuned TV. 
Retrieves the underlying display properties struct and allows calling of the low-level `draw()` method,
sending a 1024 byte buffer straight to the display.

Uses SmallRng as random number generator.

_NOTE: these are pseudorandom numbers, not suitable for cryptographic or similar purposes_