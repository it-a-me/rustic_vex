#include "main.h"
#define PRIMARY_MOTOR_PORT 1
void dong();

void initialize() {}

void disabled() {}

void competition_initialize() {}

void autonomous() {}

void opcontrol() {
  int i = 0;
  while (true) {
      i++;

    printf("%d\n", i);
    dong();
    sleep(1);
  }
}
