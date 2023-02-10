#include "main.h"
#define PRIMARY_MOTOR_PORT 1

void rust_initalize();
void rust_disabled();
void rust_autonomous();
void rust_usercontrol();

void initialize() { rust_initalize(); }

void disabled() { }

void competition_initialize() {}

void autonomous() { rust_autonomous(); }

void opcontrol() { rust_usercontrol(); }
