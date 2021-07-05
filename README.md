# B.I.T.S.I.
Bionic Interactive Technological Scientific Instrument

Code for my [eezybotarm mk2](http://www.eezyrobots.it/eba_mk2.html#)

## Arduino Sketch
All the sketch does is loop and read bytes from serial. For every nth byte it reads, it sets the nth servo to that value. e.g. if it reads in `0x5a5a5a5a`, it will set all motors to 90 degrees. 

Set the PWM pins to use for the 4 servo motors. I have them in this order: base rotation, vertical arm, horizontal arm, claw.  
Set the limits in SOFT_LIMITS. These aren't perfect but the're the main thing keeping the machine from contorting itself horribly.  
I'm using an esp8266 board. YMMV.

## Control Program
Reads SDL input from a game controller (I'm using a Switch pro controller) and sends position updates to the microcontroller. The program has to track the position and speed as a decimal number but can only send updates to the server every so often (too many updates will flood the serial port and crash the microcontroller...apparently).  
Soft limits are enforced on both ends so if you update those, be sure to change both.

### Mappings:
| Input         | Action         |
|---------------|----------------|
| Left Stick X  | Base rotation  |
| Left Stick Y  | Horizontal arm |
| Right Stick Y | Vertical Arm   |
| A Button      | Toggle Claw    |

### How to run:
1. Connect a game controller
2. Plug in microcontroller for robot arm
3. `cargo run`
