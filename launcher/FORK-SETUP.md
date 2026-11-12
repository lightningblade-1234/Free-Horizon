# Offline fork: first-time setup

## Download and run (Windows x64)

1. Open [this fork's latest Release](https://github.com/lightningblade-1234/Free-Horizon-RustMC/releases/latest).
2. Download **horizon-launcher-setup.exe** and run the existing installer, or open **horizon-launcher.exe** directly.
3. In **Home**, enter your **Offline name** and press **Play**. Use **Versions** if you need to choose/install a game build.

Running the released launcher needs no Rust, Visual Studio, source checkout or Microsoft login.
The release contains only the launcher and license/setup notices. Minecraft JARs, assets and
prepared game files are never distributed with it.

## Linux x86_64

Download **horizon-launcher-linux-x86_64.AppImage** from [this fork's latest Release](https://github.com/lightningblade-1234/Free-Horizon-RustMC/releases/latest). Run the native Linux launcher:

```sh
chmod +x horizon-launcher-linux-x86_64.AppImage
./horizon-launcher-linux-x86_64.AppImage
```

The AppImage includes GPUI's UI libraries. Your system supplies **glibc 2.35+** (Ubuntu 22.04 / Debian 12 or newer), an X11 or Wayland desktop, and graphics drivers. If FUSE is missing or disabled, use:

```sh
./horizon-launcher-linux-x86_64.AppImage --appimage-extract-and-run
```

The **horizon-offline-launcher-linux-x86_64.tar.gz** download contains the native binary and license/setup notices:

```sh
mkdir -p horizon-offline-launcher
tar -xzf horizon-offline-launcher-linux-x86_64.tar.gz -C horizon-offline-launcher
cd horizon-offline-launcher
./horizon-launcher
```

For the archive, install the UI libraries from your distribution. On Ubuntu 22.04 / Debian 12:

```sh
sudo apt-get install libxkbcommon-x11-0 libwayland-client0 libwayland-cursor0 libx11-xcb1 libfontconfig1 libvulkan1
```

Check `ldd ./horizon-launcher` for any missing libraries. Install your GPU's Vulkan driver as well (for Mesa-supported GPUs, `mesa-vulkan-drivers`). Launch from a terminal to see errors. Release files have checksums in **SHA256SUMS.txt**; verify a download with `sha256sum --ignore-missing -c SHA256SUMS.txt` from the download folder.

Home's **Play** flow installs/selects a native Linux game build. Minecraft normally lives at `~/.minecraft`; use the folder chooser for another location. Launcher data defaults to `~/.local/share/horizon-launcher`, and game data to `~/.local/share/horizon`, respecting `XDG_DATA_HOME`. The launcher also honors `HORIZON_LAUNCHER_HOME` and `HORIZON_HOME` overrides.

### Build from source on Linux

Install Rust **1.95.0**, a C/C++ build toolchain, and GPUI's development libraries. On Ubuntu 22.04 / Debian 12:

```sh
sudo apt-get install build-essential pkg-config libxkbcommon-x11-dev libwayland-dev libx11-xcb-dev libfontconfig-dev libvulkan-dev libasound2-dev
rustup toolchain install 1.95.0
HORIZON_LAUNCHER_NO_SELF_UPDATE=1 cargo +1.95.0 build --manifest-path launcher/Cargo.toml --release --locked
./launcher/target/release/horizon-launcher
```

## Automatic resource setup (both platforms)

Runtime prerequisites are set up automatically in the normal launcher, without PowerShell or
a source checkout. It installs/selects a Horizon build, reuses local Java or obtains managed
**Java 25**, downloads a checksum-verified public server JAR into your local cache when needed,
and prepares from your own **Minecraft 26.3** installation (normally `%APPDATA%\.minecraft` on Windows or `~/.minecraft` on Linux).
Progress and failures appear in the existing Console/Home interface. After finding a matching local
client JAR, setup reuses its version JSON or downloads the checksum-verified public JSON into
Horizon's cache when metadata is missing or rewritten. Your launcher's metadata is preserved.
If local Minecraft client files are missing, use the Minecraft folder chooser and supply the
requested files; the launcher does not authenticate or download those missing client files.

An existing server JAR in `%APPDATA%\horizon-launcher\server-jars\26.3.jar` on Windows, `~/.local/share/horizon-launcher/server-jars/26.3.jar` on Linux, or at
`HORIZON_SERVER_JAR` can be reused after checksum verification. The 26.3 pin is
`33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`.

The preparer writes resources locally, then the game runs in Rust. Offline names accept
1-16 printable ASCII characters without spaces/control characters. The same spelling keeps
the same offline UUID. Single-player, LAN and offline-mode servers work with this identity.

## Clone and set up with one command (Windows)

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
