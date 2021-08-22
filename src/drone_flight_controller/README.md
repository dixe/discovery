# Drone Flight Controller

## Requirements

Should be able to keep a current state of the drone.

The state has the following properties

* Orientation
* Acceleration
* Velocity


Should also have a target state, with the same properties.


Should be able to output a speed for each motor such that the current state will
tend towards the target state.



#  Milestones

## General milestones
- [ ] Real time update of the current state.



## Milestone specifications
### Real time update of the current state
This will be tilting the controller accelerating it, moving it, should all be captured and
updated in the current state.
