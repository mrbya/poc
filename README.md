# poc

Describe poc here

## SDK setup

Project based on the [Zephyr](https://www.zephyrproject.org/) RTOS.

- [Environment setup](https://docs.zephyrproject.org/latest/develop/getting_started/index.html)

## Supported targets

List supported targets here

# Building and flashing

Zephyr OS uses a CMake-based build system:

1. configure cmake project

```bash
mkdir build && cd build
cmake .. -GNinja
```

2. build using `ninja`
```bash
# inside build dir
ninja
```

## Flashing binary

Project is configured to use the `openocd` runner, meaning you do not have to manually flash using an STM32 cube programmer. Simply flash the app using `ninja`:
```bash
# still inside the build dir
ninja flash
```

# Project structure

Project repo is structured as follows:
```bash
hgone
├── boards              # Board-specific devicetree overlays and configs
│   └── ...
├── lib                 # App libraries
│   └── ...
├── include             # App includes
│   └── ...
├── src                 # App sources
│   └── ...
├── .clangd             # Zephyr OS specific clangd LSP server setup
├── CMakeLists.txt      # App CMake config
├── fix_glibcpp.py      # Utility script to fix glibcpp includes for clangd LSP server
├── LICENSE             # MIT License
├── prj.conf            # Zpehyr OS default kconfig
└── READAME.md          # This READAME
```

        