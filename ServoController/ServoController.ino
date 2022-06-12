/*
 Controlling a servo position using a potentiometer (variable resistor)
 by Michal Rinott <http://people.interaction-ivrea.it/m.rinott>

 modified on 8 Nov 2013
 by Scott Fitzgerald
 http://www.arduino.cc/en/Tutorial/Knob
*/

#include <Servo.h>
#include <Arduino.h>
#include <Wire.h>
#include <stdint.h>

//#define SERIAL

#define SERVO_N 5
Servo servos[5];
int pins[5] = {9, 8, 7, 6, 5};


void setup() {
  Wire.begin(4);
  Wire.onReceive(receiveEvent);
#ifdef SERIAL
  Serial.begin(9600);
  Serial.println("Serial begin");
#endif
  
  for (int i = 0; i < 5; i++)
  {
    servos[i].attach(pins[i]);
    servos[i].write(90);
  }
}

void loop() {
  delay(1000);
#ifdef SERIAL
  Serial.println(String(Wire.available()));
#endif
}

void receiveEvent(int howMany)
{
  while (2 <= Wire.available())
  {
    uint8_t id = Wire.read();
    uint8_t value = Wire.read();
#ifdef SERIAL
    Serial.println("id: " + String(id) + "; value: " + String(value));
#endif
    if ((id < SERVO_N) && (value <= 180))
    {
      servos[id].write(value);
    }
  }
}
