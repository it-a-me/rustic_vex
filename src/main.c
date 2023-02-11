#include "main.h"
#define PRIMARY_MOTOR_PORT 1

void rust_initalize();
void rust_disabled();
void rust_competition_initalize();
void rust_autonomous();
void rust_usercontrol();

void initialize() { rust_initalize(); }

void disabled() { rust_disabled(); }

void competition_initialize() { rust_competition_initalize(); }

void autonomous() { rust_autonomous(); }

void opcontrol() { rust_usercontrol(); }
