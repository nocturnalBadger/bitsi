#include <ServoEasing.h>

#define BUFFER_SIZE 32
const uint8_t NUM_SERVOS = 4;

const uint8_t SERVO_PINS[NUM_SERVOS] = {
  14,
  12,
  13,
  2,
};

const int SOFT_LIMITS[NUM_SERVOS][2] = {
   {0, 180},  // Base rotation
   {60,150},  // Lower joint
   {40, 120}, // Upper joint
   {0, 180}, // Claw
};

ServoEasing servos[NUM_SERVOS];

void setup() {
  // put your setup code here, to run once:
  Serial.begin(57600);
  for (int i = 0; i < NUM_SERVOS; i++) {
      servos[i].attach(SERVO_PINS[i]);
      servos[i].setSpeed(50);
      servos[i].setEasingType(EASE_CUBIC_IN_OUT);
      servos[i].write(90);
  }
}

long long int i = 0;
void loop() {
    int si = i % NUM_SERVOS;
    if (Serial.available()) {
        uint8_t val = Serial.read();
        if (val < SOFT_LIMITS[si][0] || val > SOFT_LIMITS[si][1]) {
            Serial.printf("%i is beyond soft limits for servo %i. Skipping", val, si);
        } else {
            Serial.printf("Setting servo %i to %i\n", si, val);
            servos[si].write(val);
        }
        i++;
    }
}
