[![Build Status](https://travis-ci.com/magdaddy/ableton-link-rs.svg?branch=master)](https://travis-ci.com/magdaddy/ableton-link-rs)

# ableton-link-rs

A wrapper for the [Ableton Link](https://github.com/Ableton/link) library, a technology that synchronizes musical beat, 
tempo, and phase across multiple applications running on one or more devices. Applications on devices connected to a 
local network discover each other automatically and form a musical session in which each participant can perform 
independently: anyone can start or stop while still staying in time. Anyone can change the tempo, the others will 
follow. Anyone can join or leave without disrupting the session.

For further details, see [Ableton Link](https://github.com/Ableton/link).

## Remarks

This is my first shot at wrapping a c++ library for Rust. Also, I'm pretty much a c/c++ noob. So expect some rough edges.

I'm not totally sure about the all the design choices, especially the `with_app/audio_session_state`- methods. If you see a better way, PR's are welcome.

This has only been tested on macOS and seems to compile on Linux as well. Don't know about Windows or others...
