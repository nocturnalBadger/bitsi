#include <ServoEasing.h>

#define BUFFER_SIZE 32
const uint8_t NUM_SERVOS = 3;

const uint8_t SERVO_PINS[NUM_SERVOS] = {
  14,
  12,
  13,
};

const int SOFT_LIMITS[NUM_SERVOS][2] = {
   {0, 180},  // Base rotation
   {60,150},  // Lower joint
   {40, 120}, // Upper joint
};

char buffer[BUFFER_SIZE];
ServoEasing servos[NUM_SERVOS];

void initBuffer(char *buffer) {
  for (int i = 0; i < BUFFER_SIZE; i++) {
    buffer[i] = '\0';
  }
}

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  for (int i = 0; i < NUM_SERVOS; i++) {
      servos[i].attach(SERVO_PINS[i]);
      servos[i].setSpeed(50);
      servos[i].setEasingType(EASE_CUBIC_IN_OUT);
      servos[i].write(90);
  }
  initBuffer(buffer);
}

int i = 0;
void loop() {
    while (Serial.available()) {
        char c = Serial.read();

        //Serial.printf("%i ", c);
        
        if (c == '\n') {
            buffer[i + 1] = '\0';
            Serial.println(buffer);
            setServos(buffer);
            initBuffer(buffer);
            i = 0;
        } else {
            buffer[i++] = c;
        }
    }
}

void setServos(char *input) {
    char *token;
    token = strtok(input, " ");
    int servoIndex = 0;
    while (token != NULL) {
        //Serial.println(token);
        int val = atoi(token);
        if (val < SOFT_LIMITS[servoIndex][0] || val > SOFT_LIMITS[servoIndex][1]) {
            Serial.printf("%i is beyond soft limits for servo %i. Skipping", val, servoIndex);
        } else {
          Serial.printf("Setting servo %i to %i\n", servoIndex % NUM_SERVOS, val);
          servos[servoIndex++ % NUM_SERVOS].startEaseTo(val);
        }
        token = strtok(NULL, " ");
    }
}
