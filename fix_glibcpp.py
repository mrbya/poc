#!/usr/bin/python

import json

ccpath = "compile_commands.json"

missing_paths = "-isystem/usr/lib/gcc/arm-none-eabi/14.2.0/../../../../arm-none-eabi/include/c++/14.2.0 -isystem/usr/lib/gcc/arm-none-eabi/14.2.0/../../../../arm-none-eabi/include/c++/14.2.0/arm-none-eabi -isystem/usr/lib/gcc/arm-none-eabi/14.2.0/../../../../arm-none-eabi/include/c++/14.2.0/backward -isystem/usr/lib/gcc/arm-none-eabi/14.2.0/include -isystem/usr/lib/gcc/arm-none-eabi/14.2.0/include-fixed -isystem/usr/lib/gcc/arm-none-eabi/14.2.0/../../../../arm-none-eabi/include"

with open(ccpath, "r") as f:
    commands = json.load(f)

for entry in commands:
    entry["command"] = " ".join((entry["command"], missing_paths))

with open(ccpath, "w") as f:
    json.dump(commands, f, indent=2)

