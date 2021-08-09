# POC pwm esc


Simple control with analog stick Y to increase/decrease motor speed.

User button (blue on board) will stop the motor.


# Startup
Init start with low and wait for moter 3 beep then 2 beep and a long beep now the motor i ready and we can use the motor



##  Wiring
This only works if the board has power from the ESC into 5v, and common ground. Debugging stil seems to work, it just need the power.

### Esc
- Control(yellow) to PA4
- ESC Brown to ground
- ESC red to board 5v INPUT


### Analog Stick
- Control(yellow) to PA1
- Brown to common ground
- Red to board 3v OUTPUT


# Detail

The pwn signal generated on PA4 is a 50hz signal with a pulse width between 1 ms and 2 ms. So between 5% and 10% duty cycle, using a resolution of 65535.


The analog stick uses the adc1 on PA1, with values between 0 and 4094. Using 3v as input gives a value of about 2048 when stick is in neutral. Using 5V intput for analog stick results in neutral value of about 3000.
