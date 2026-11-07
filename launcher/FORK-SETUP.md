# Offline fork: first-time setup

## Download and run (Windows x64)

1. Open [this fork's latest Release](https://github.com/lightningblade-1234/horizon/releases/latest).
2. Download **horizon-launcher-setup.exe** and run the existing installer, or open **horizon-launcher.exe** directly.
3. In **Home**, enter your **Offline name** and press **Play**. Use **Versions** if you need to choose/install a game build.

Running the released launcher needs no Rust, Visual Studio, source checkout or Microsoft login.
The release contains only the launcher and license/setup notices. Minecraft JARs, assets and
prepared game files are never distributed with it.

Preparation uses your own local **Minecraft 26.3** installation (normally `%APPDATA%\.minecraft`)
and **Java 25**. Minecraft-managed Java runtimes can be reused. If Home reports missing input,
use its Minecraft folder chooser and provide the file requested. The public server JAR's URL
is `downloads.server.url` in your local `versions/26.3/26.3.json`; put the verified JAR in
`%APPDATA%\horizon-launcher\server-jars\26.3.jar`, or set `HORIZON_SERVER_JAR` to its path.
Its pinned SHA-1 is `33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`.

The preparer writes resources locally, then the game runs in Rust. Offline names accept
1-16 printable ASCII characters without spaces/control characters. The same spelling keeps
the same offline UUID. Single-player, LAN and offline-mode servers work with this identity.

## Clone and set up with one command

From the cloned repository folder, run:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\setup_launcher.ps1
```

The developer helper installs missing C++/Windows SDK tools and Rust through winget, selects
Rust 1.95.0, builds the launcher with low-cache settings, installs a Horizon game build through
the existing launcher, reuses/installs Java 25, downloads the checksum-verified public server JAR
into your local cache, prepares from your own Minecraft install, and opens the normal Home screen.
Windows installers may need elevation or a restart. Missing local Minecraft client files must be
supplied by you; the helper does not download them.

For a non-default Minecraft folder, append `-MinecraftDir 'D:\your\minecraft'`.
The compiled launcher is `dist\horizon-launcher.exe` (gitignored). `-BuildOnly` builds just the
launcher; `-NoLaunch` completes preparation without opening the window.

To update a source checkout, run `git pull --ff-only` on `main` and repeat the setup command.
Released fork launchers keep the upstream self-updater disabled; download a newer fork Release
to update the launcher. The normal version picker still manages game builds.
