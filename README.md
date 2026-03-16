# About The Project
This is a project that simulates the evolution of digital life. Both an organisms structure and brain can evolve as the generations pass. Through random mutation and natural selection organisms become better adapted to their environment

# Features
There are no concrete features as part of this project, I'm just playing with the code making tweaks here an there until I get some behavior that I think looks interesting

# Architecture
The architecture of the project can be split into the environment and the organisms with organisms being able to read the state of the environment and write to certain layers

## Environment
The environment consists of several layers that each evolve using convolution. Each layer has a specific role as follows:
* Energy - This layer lets organisms that have an energy node extract energy from it. The values of the cells in this layers increases globally periodically simulating how the sun provides energy to plants during the day and doesn't provide any energy at night.
* Decompose - This layer lets organisms that have a decomposer node extract energy from it. The values of the cells in this layers increases locally when an organism dies in the area. This allows for creatures that specialize as decomposers, similar to fungi.
* Pheromone - This layer lets organisms communicate. Read/write nodes can read/write the values of the cells in this layer.

## Organism
* Organisms are represented using a graph like structure where there are joints, bones between joints. Muscles also exist between bones but are a bit of a legacy feature when the simulation was used to evolve creatures that could learn to walk.
* An organism dies when it runs out of energy.
* Each organism has a neural network that takes and input created by the organisms nodes (e.g. the read node's current value) and outputs another vector used to control the state of the organism (e.g. the thruster node direction and thrust strength).
* When an organism gets enough energy a portion of the energy is consumed and an egg is laid. The egg will hatch after a period of time.
* Random mutations are applied to an egg when it is laid. The structure, nodes and neural network can be mutated.

Each joint also has a list of attached nodes with each node type performing a different function:
* Energy - Collects energy from the energy layer.
* Decomposer - Collects energy from the decompose layer.
* Read - Reads the value from any environment layer at the current position of the joint.
* Write - Writes a value to pheromone layer at the current position of the joint.
* Thruster - Consumes energy to apply a force the the joint, moving the organism.
* Spike (WIP) - Siphons energy from nearby creatures.

# An example
A single organism was spawned with a single joint containing an energy node allowing it to collect energy from the red hotspots of energy.
After a few generations mutations occurred with some developing thrusters so they could move out of the energy coldspots and others developing more multiple joints with energy collecting nodes.
<img width="640" height="797" alt="Screenshot 2026-03-16 203619" src="https://github.com/user-attachments/assets/ae0234d2-cec3-4700-8d15-2cbb4e7c8bf0" />


# Getting started
Clone this repo and open a terminal window in the root of this directory. Run `cargo r` and the WASD keys to pan and up/down arrow to zoom in/out.
