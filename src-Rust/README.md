# GMS-DCTokenGrabber

Simple extension for *[GameMaker Studio 2](https://www.yoyogames.com/gamemaker/features)* to get Discord tokens from given path. 

Made only for educational purpose! <br>
Be aware that author of this extension is **NOT** responisble for your actions! <br>
Using it in harmful way is against [Discord](https://discord.com/terms) and [YoYo](https://www.yoyogames.com/legal/eula) ToS, as well as for some of the local laws!

## Build

Install a Rust target *i686-pc-windows-msvc*:
```
rustup target add i686-pc-windows-msvc
```

Build the lib:
```
cargo build --lib --release --target=i686-pc-windows-msvc
```
Than you can find your DLL file in `(...)/target/i686-pc-windows-msvc/release/`

## Depedencies
```
regex = "1.3.9"
encoding = "0.2.33"
```

## Notes
Grabber won't find any mfa tokens. (I don't fully understand *regex* crate). **(FIXED)** <br>
64bit version won't work for GameMaker Studio. <br>
It is my second project in this language so code can be pretty bad. <br>
