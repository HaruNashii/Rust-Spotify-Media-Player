> [!WARNING]
> THIS PROJECT IS NOT MADE FOR THE PUBLIC, IT HAVE POOR PERFORMANCE, THE PURPOSE OF THIS REPO IS JUST TO ME LEARN RUST.
> IF YOU USE IT PLEASE KEEP CHECKING YOUR PERFORMANCE, IT MAY HAVE GIANT MEMORIES LEAK'S


## THEMING
**don't change the window size because the elements will not be aligned correctly**
This APP Supports theming the app by changing the icons png on the ui folder or changing some icons via code
also ever elements in this app can change position, size and color!




## Dependencies
- Curl
**How To Istall Curl**
Fedora
```sudo dnf install curl```
Debian, Ubuntu, Linux Mint, Etc...
```sudo apt install curl```
Arch Linux
```sudo pacman -S curl```
Void Linux
```sudo xbps-install -y -f curl```

- [playerctl](https://github.com/altdesktop/playerctl)
**How To Istall PlayerCtl**
Fedora
```sudo dnf install playerctl```
Debian, Ubuntu, Linux Mint, Etc...
```sudo apt install playerctl```
Arch Linux
```sudo pacman -S playerctl```
Void Linux
```sudo xbps-install -y -f curl```
Any Others Distros Linux Can Be Download Via The Guix Package Manager with
```guix install playerctl```

- Rust/Cargo
**How To Istall Rust/Cargo**
Fedora
```sudo dnf install cargo```
Debian, Ubuntu, Linux Mint, Etc...
```sudo apt install -y cargo```
Arch Linux 
```sudo pacman -Sy cargo```
Void Linux
```sudo xbps-install -y -f cargo```

- sdl2, sdl2-image sdl2-ttf
- **How To Istall sdl2, sdl2-image and sdl2-ttf**
Fedora
```sudo dnf install SDL2 SDL2_image SDL2_ttf```
Debian, Ubuntu, Linux Mint, Etc...
```sudo apt install -y libsdl2-image-dev libsdl2-ttf-dev```
Arch Linux 
```sudo pacman -S sdl2 sdl2_image sdl2_ttf```
Void Linux
```sudo xbps-install -y -f SDL2_image SDL2_image-devel SDL2_ttf SDL2_ttf-devel```





## Building
Clone The Repo
```https://github.com/HaruNashii/Rust-Spotify-Media-Player/edit/main/README.md```
Go To Repo Folder
```cd Rust-Spotify-Media-Player```
Run Build Script
```./build.sh```





## Running
After Building the app will be localized in "target/release" just run the "media" app with
```./media``` OR run with one file manager like (thunar, nemo, nautilus, etc...).


