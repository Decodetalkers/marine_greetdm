# Marine Greetdm

a simple dm cli, gui one is a toy, do not use it

It can run without dm, so you will do not worry if nvidia is started before sddm --- this dm even does not use any display!!

# Config

This dm allow you to config different environment for different wm , config file is under `/etc/marine_greetd/config.toml`,for exmaple

```toml
[[envs]]
UseIn = "Sway" # this should be the key in desktop, show the name
Values = [
        { key = "QT_QPA_PLATFORMTHEME", value = "qt5ct" },
        { key = "_JAVA_AWT_WM_NONREPARENTING", value = "1" },
        { key = "XDG_CURRENT_DESKTOP", value = "sway" },
]
```
