[![Build Status](https://travis-ci.org/Malkaviel/KindredEngine.svg?branch=master)](https://travis-ci.org/Malkaviel/KindredEngine)  [![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![codecov](https://codecov.io/gh/Malkaviel/KindredEngine/branch/master/graph/badge.svg)](https://codecov.io/gh/Malkaviel/KindredEngine)

# WARNING
This is a personal project for learning game engine architecture, game engine implementation, and Rust.

The purpose and general architecture is still not defined :
 - General-purpose or highly genre specific ?
 - features ?
 - "keep it simple", or "bulletproof" ?
 - ...
 
 It is far from being usable at this time, but the idea is to implement a game engine, see what works well,
 what should be refactored and what should become its own library (which should be pushed on crates.io).
 
 If you are looking for a promising Rust game engine or game engine resources, just take a look at : 
 http://arewegameyet.com/index.html

# Kindred game engine
game engine written in Rust.

### Math library :
The engine uses the 'cgmath' crate, which provides the fundamental tools for computer graphics (Vectors, Matrices, Quaternions...).
cgmath uses the simd crate to take advantage of SIMD instructions for calculations.
It also uses Serde, a popular serialization/deserialization library.
A very basic Xorshift random number generator is also included, using the 'rand' crate.


