# meshbuzz
MeshBuzz software and hardware

TODO: Try to add some minimal code to
meshbuzz-base, timelets and meshbuzz-pc

## meshbuzz-base

Defines core types to be used by all MeshBuzz modules.

## timelets

Timelets are part of a "chess clock", but can be combined with
other timelets to create a chess clock for n players.

## meshbuzz-esp32

This module is the one that will run on the ESP32.
It imports the meshbuzz-base and timelets modules.

It implements some hardware interface trait defined in meshbuzz-base.

## meshbuzz-pc

This module is the one that will run on the PC.
It imports the meshbuzz-base and timelets modules.

It implements some hardware interface trait defined in meshbuzz-base.

Then you can run all the modules right on the PC and try them there.

You should be able to run multiple MeshBuzz modules on a single PC. 