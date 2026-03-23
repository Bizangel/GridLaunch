## README

TODO

## Limitations / To Improve

- Uses user-based approach. All users names must be created with game-user prefix.
    - Creation of gamepad group is also manual.
    - This means that creating profiles is manual and requires privileges.

- Bigscreen mode requires --kwin flag - does not support fullscreen so looks ugly.
- 3 Player UI has visual bugs. Chosen sides will not be perfectly honored.

- In Bazzite handhelds it appears to conflict with Handheld daemon outside of game-mode (need to look more into it.).

- When launching steam sometimes there can be flickering black dots. Not really sure why and it's a bit hard to reproduce.

## TODOS

- Generic window allocation system. -> Should be allowed to be another script or something else.
    - Right now it is hardcoded to use kwin scripts.

## Licenses

This project includes portions of code borrowed from [Partydeck](https://github.com/partydeck/partydeck)
Mainly related to window handling logic and x11 monitor snippets.
