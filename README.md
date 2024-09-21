# Lupy

General purpose Luau to Python 3 transpiler.

Lupy parses Luau abstract syntax trees to emit functionally similar structures
in Python. Enjoy typed Python packages from Luau.

TODO: Install & beta release

## Usage

Python modules can be required with the `@pypkg` scope:

```Luau
local vex = require("@pypkg/vex")
local brain = vex:Brain()

local function main()
  brain.screen:print("Hello from Luau!")
end
```

## But Why‽

Because of stupid limitations. VEX Robotics only offers C++ and Python. Nobody
likes learning C++, and Python lacks strong typing. Please do not tell me I have
to write doc comments just to know what I am using.

Lupy solves this using Luau, which features a gradual structured type system.
It's more fun than C++ and more caring than Python. That's all I care.
