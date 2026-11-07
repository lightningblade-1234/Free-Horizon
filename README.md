<p align="center">
  <img src="docs/media/banner.webp" alt="Horizon: the game you know, rebuilt in Rust" width="100%">
</p>

<p align="center">
  <a href="https://playhorizon.dev"><b>Website</b></a> ·
  <a href="#offline-fork-setup-windows"><b>Offline fork setup</b></a> ·
  <a href="#download"><b>Download</b></a> ·
  <a href="https://playhorizon.dev/docs/"><b>Docs</b></a> ·
  <a href="https://discord.gg/playhorizon"><b>Discord</b></a> ·
  <a href="ROADMAP.md"><b>Roadmap</b></a> ·
  <a href="#contributing"><b>Contributing</b></a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/vanilla%20parity-100%25-F0795A" alt="Vanilla parity 100%">
  <a href="#project-status"><img src="https://img.shields.io/badge/player--visible%20gaps-0%20open-F0795A" alt="Player-visible gaps open"></a>
</p>

# Horizon

**Minecraft: Java Edition 26.3, rewritten in Rust.** Your worlds, the same rules and the same terrain as the Java game, checked against it by hundreds of differential tests, in one native program with no Java. Free, open source (MIT) and unofficial: you play it with your own copy of Java Edition.

**This repository is [lightningblade-1234's offline-play fork](https://github.com/lightningblade-1234/horizon).** Its normal launcher lets you enter a player name and press Play without Microsoft login. Follow the [fork setup guide](#offline-fork-setup-windows) below; the upstream downloads have a different account flow.

<p align="center">
  <img src="docs/media/readme/valley.webp" alt="A river valley with a plains village, savanna hills and a cherry grove, seen from above with a long view distance" width="100%">
</p>

## What works today

Everything below is in the current release.

- **The game.** Survival, creative, hardcore and spectator; vanilla world generation in all three dimensions, mobs, villagers, raids, redstone, every command, the HUD and the recipe book. Vanilla saves open as they are, and worlds from 1.8 to 26.2 are upgraded.
- **Multiplayer.** Join vanilla and Horizon servers (online mode included), open a world to LAN, or invite friends with a code from the launcher's Home page.
- **Graphics.** Distant terrain up to 1,024 chunks, graphics presets, HDR and controllers, all off by default where they would change the vanilla look ([settings](https://playhorizon.dev/docs/settings)).
- **Packs.** Resource packs, data packs and shader packs ([below](#mods-and-shader-packs)).
- **Mods.** Content mods and native Rust mods, installed from the launcher ([below](#mods-and-shader-packs)).
- **Servers.** Host one from the launcher, or run `horizon-server` on a VPS ([below](#host-a-server)).
- **The launcher.** It installs and updates the game, prepares its files from your copy of Minecraft, and updates itself.

<table>
  <tr>
    <td width="50%"><img src="docs/media/readme/sunset.webp" alt="Sunset behind a cherry grove on a hill, with a river below"></td>
    <td width="50%"><img src="docs/media/readme/nether.webp" alt="A crimson forest in the Nether, with a piglin among the fungi"></td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/media/readme/survival.webp" alt="Survival in a village square: two villagers, flowers, the hotbar, health and hunger"></td>
    <td width="50%"><img src="docs/media/readme/multiplayer.webp" alt="Two players in a LAN world: the other player's name tag and the player list"></td>
  </tr>
</table>

<sub>Nightly 2026-11-05 on a Mac mini (Apple M4 Pro, Metal): vanilla's textures, no mods or shader packs, render distance 32 with distant terrain on.</sub>

## Performance

**Horizon draws more frames per second than Java Edition, and loads faster, on the same machine, world and path.** `tools/perf_bench.py` runs each game in turn, from a fresh world (seed 1234), waiting for the terrain and then flying it over a village, a forest, an ocean and a mountain. Measured on 2026-10-29/30 on an idle Windows 11 machine (Ryzen 7 9800X3D, RTX 5080), Horizon v0.2.97 as the launcher installs it, 1280x720 in a window, VSync off, three games per figure. Frames per second, mean of the four scenes, with the slowest 1% of frames in brackets:

| Render distance | Horizon v0.2.97 | Vanilla 26.3 | Vanilla + mods |
| --- | --- | --- | --- |
| 8 chunks | 3,289 (1,327) | 2,437 (640) | 2,375 (709) |
| 16 chunks | 2,074 (598) | 1,661 (375) | 1,586 (432) |
| 32 chunks | 1,207 (333) | 1,031 (176) | 879 (264) |

"Vanilla + mods" is Sodium, Lithium, FerriteCore and ImmediatelyFast. Horizon reaches the title screen in 2.9–3.2 s against vanilla's 7.5–7.7 s (the modded client's 10.8–11.1 s), loads a scene's chunks 2.9–3.8 times faster, and playing holds a steady working set of 1.1–1.7 GB against 1.9–2.9 GB. Every scene with its confidence interval and the run's load log: [the 2026-10-29 reports](benchmarks/perf/windows-9800x3d-5080-2026-10-29.md). Method, and how to measure your own machine: [docs/benchmark.md](docs/benchmark.md). [playhorizon.dev/benchmarks](https://playhorizon.dev/benchmarks) puts these runs beside a 2016 laptop, a Mac mini and a gaming desktop.

## Offline fork setup (Windows)

This guide applies to this fork's **`main` branch**, using Minecraft **26.3** resources and Rust **1.95.0**. The existing Home/Play interface, version picker, Minecraft discovery, instance folders, Console and crash handling are used throughout.

### Already set up on this computer

Open the compiled launcher directly:

```powershell
& "C:\Users\shree\Desktop\freerustminecraft\horizon\launcher\target\debug\horizon-launcher.exe"
```

Enter your name in **Offline name**, then press **Play**. This computer already has a verified server JAR in `%APPDATA%\horizon-launcher\server-jars\26.3.jar` and a prepared Horizon installation with sounds. There is no `--local` flag or separate offline page.

### 1. Install the build prerequisites

- Git and [Rust through rustup](https://rustup.rs/).
- Visual Studio Build Tools with the **Desktop development with C++** workload and a Windows SDK. The SDK's `fxc.exe` compiles the launcher's embedded DirectX shaders; `GPUI_FXC_PATH` can select it explicitly if discovery fails.
- A local **Java 25 JDK** for resource preparation. On the configured computer it is `C:\Program Files\Eclipse Adoptium\jdk-25.0.4.101-hotspot`. Java is used during preparation; the prepared game runs in Rust.

### 2. Build this fork's launcher

For a fresh checkout, run these commands in PowerShell:

```powershell
git clone --branch main https://github.com/lightningblade-1234/horizon.git
Set-Location horizon
rustup toolchain install 1.95.0

$env:CARGO_INCREMENTAL = '0'
$env:CARGO_PROFILE_DEV_DEBUG = '0'
$env:CARGO_BUILD_JOBS = '4'
cargo +1.95.0 build --manifest-path launcher/Cargo.toml --locked
```

These settings reduce build-cache size and memory pressure. The resulting executable is `launcher\target\debug\horizon-launcher.exe`. Windows shaders are embedded, so that executable can run after disposable Cargo build caches are removed.

### 3. Provide Minecraft 26.3's local files

The launcher discovers an existing Minecraft install. In the usual Windows layout, preparation needs:

```text
%APPDATA%\.minecraft\versions\26.3\26.3.jar
%APPDATA%\.minecraft\versions\26.3\26.3.json
%APPDATA%\.minecraft\libraries\
%APPDATA%\.minecraft\assets\
```

Use the launcher's **Minecraft folder** chooser if discovery selects the wrong folder. The JAR and version JSON must match the selected Horizon build's pins; files from another Minecraft version will not work. Minecraft files and prepared resources are not included in this repository.

Preparation also needs the genuine **26.3 server JAR** to export server-side game data used by single player. Its official download is listed as `downloads.server.url` in the local `26.3.json`. For this version, the following one-time setup command downloads it into the launcher's default cache and verifies the pinned checksum:

```powershell
$serverJar = Join-Path $env:APPDATA 'horizon-launcher\server-jars\26.3.jar'
New-Item -ItemType Directory -Force (Split-Path $serverJar) | Out-Null
Invoke-WebRequest -UseBasicParsing `
    -Uri 'https://piston-data.mojang.com/v1/objects/33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c/server.jar' `
    -OutFile "$serverJar.partial"
$expected = '33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c'
if ((Get-FileHash "$serverJar.partial" -Algorithm SHA1).Hash.ToLowerInvariant() -ne $expected) {
    throw 'The downloaded server JAR does not match Minecraft 26.3.'
}
Move-Item "$serverJar.partial" $serverJar -Force
```

If you already have the verified JAR elsewhere, set `$env:HORIZON_SERVER_JAR = 'C:\your\path\server.jar'` before opening the launcher. The server JAR is a preparation input; ordinary Play does not start a Java server.

### 4. Open Home and play offline

From the checkout folder:

```powershell
# Adjust JAVA_HOME if your JDK is installed elsewhere.
$env:JAVA_HOME = 'C:\Program Files\Eclipse Adoptium\jdk-25.0.4.101-hotspot'
$env:HORIZON_LAUNCHER_NO_SELF_UPDATE = '1'
& .\launcher\target\debug\horizon-launcher.exe
```

1. Use **Versions** to select/install a Horizon game build if none is installed. Building the launcher alone does not install the game.
2. In **Home**, enter an offline name: **1–16 printable ASCII characters, without spaces or control characters**. The launcher remembers it. Keep the same spelling and capitalization to keep the same offline UUID.
3. Click **Play**. If needed, the existing preparer uses your local Minecraft files, server JAR and Java to prepare the selected build. Missing inputs appear as errors; offline Play does not authenticate or fall back to downloading Mojang files.
4. In the game, choose **Singleplayer** and create or open a world. LAN and offline-mode servers also support this identity; online-mode servers require an authenticated account.

The first resource preparation takes longer than later launches. Progress and game output are available in the existing **Console**. The launcher stays running and displays startup failures and crash reports.

`HORIZON_LAUNCHER_NO_SELF_UPDATE=1` prevents a fork release build from replacing itself with an upstream launcher. Development builds already disable launcher self-updates. Game versions are still managed through the normal launcher interface.

### Files, updates and troubleshooting

Default Windows locations:

| What | Location |
| --- | --- |
| Launcher settings and remembered name | `%APPDATA%\horizon-launcher\settings.json` |
| Installed game builds and prepared resources | `%APPDATA%\horizon-launcher\versions\<tag>\` |
| Server JAR cache | `%APPDATA%\horizon-launcher\server-jars\26.3.jar` |
| Default game folder | `%APPDATA%\horizon\` |

Each instance can have its own game folder; Home's **Folder** control opens the selected one. Custom data locations and `HORIZON_LAUNCHER_HOME` / `HORIZON_HOME` overrides change these defaults.

- **Missing resources or server JAR:** check the paths above, then use Play or Prepare again. Local inputs must match that build's version.
- **An older launcher fails after cache cleanup:** rebuild from current `main`; the embedded-shader fix removes the Windows debug launcher's runtime dependency on shader files in Cargo caches.
- **Extra arguments rejected:** remove `--name`, `--session-file` and `--session-stdin` from extra game arguments. Home supplies the offline name.
- **Update the checkout:** run `git pull --ff-only` on `main`, then repeat the launcher build command. Pulling source changes alone does not rebuild the executable.

Further details: [this fork's launcher documentation](launcher/README.md#personal-fork-offline-play). The game's parser accepts `--name`, but it is not an advertised external Launch API option.

## Download

**The downloads below are upstream releases.** For this fork's offline launcher, use the [setup guide above](#offline-fork-setup-windows).

Get **the Horizon launcher** from [playhorizon.dev](https://playhorizon.dev), or from [the launcher releases](https://github.com/horizon-rust/horizon/releases?q=launcher&expanded=true):

| Platform | File | Needs |
| --- | --- | --- |
| Windows x64 | `horizon-launcher-setup.exe` | Windows 10 or 11, a GPU with Vulkan or DirectX 12 |
| Windows on Arm | `horizon-launcher-setup-arm64.exe` | Windows 11 on Arm |
| macOS, Apple Silicon | `horizon-launcher-macos-arm64.dmg` | macOS 12 or later |
| macOS, Intel | `horizon-launcher-macos-x64.dmg` | macOS 12 or later |
| Linux x86-64 | `horizon-launcher-linux-x86_64.AppImage` or `.tar.gz` | a glibc distribution, X11 or Wayland, a GPU with Vulkan or OpenGL 3.3 |

**The upstream launcher requires a Microsoft account that owns Minecraft: Java Edition.** You sign in with it in that launcher, which plays only with an account that owns the game ([why](docs/microsoft-account.md)). Horizon contains no Minecraft code or assets: an existing install (official launcher, Prism, MultiMC...) is reused, and without one the upstream launcher downloads 26.3 from Mojang for your account. Step by step for upstream releases: [Install and first launch](https://playhorizon.dev/docs/install). This fork's normal offline Home/Play flow uses the local files described above.

<table>
  <tr>
    <td width="50%"><img src="docs/media/readme/launcher-home.webp" alt="The launcher's Home page: the Horizon instance, Play, the version picker, the friend invite field, screenshots and the patch log"></td>
    <td width="50%"><img src="docs/media/readme/launcher-mods.webp" alt="The launcher's Mods page: Horizon's mod registry with Zoom selected, and tabs for modpacks, resource packs, shader packs and data packs"></td>
  </tr>
</table>

## Mods and shader packs

- **Content mods** (`.hmod`): blocks, items, recipes, loot, models and textures, with no code.
- **Native mods** (`.hnmod`): Rust that the launcher compiles into the game for you, with nothing to install.
- **Java mods** (Fabric, NeoForge, Forge) don't run: Horizon has no Java. The launcher carries over the data and assets inside their jars, and imports modpacks (`.mrpack`, CurseForge zips, Prism instances) the same way ([modpacks](https://playhorizon.dev/docs/modpacks)).
- **Shader packs**: Iris/OptiFine-format packs (`shaders/` folders) and Horizon's own WGSL packs, under Video Settings → Shader Packs ([pack format](docs/shaders.md)).

Mods are off by default: a fresh install plays exactly like vanilla 26.3. The launcher's Mods page browses Horizon's registry and Modrinth. [Mods](https://playhorizon.dev/docs/mods) · [make a mod](docs/mods.md).

## Host a server

The launcher's **Servers** page creates, starts, backs up and updates a server, and invites friends. On a VPS, `horizon-server` installs with one command, needs no Java, and takes vanilla 26.3 and Horizon players alike. [Host a server](https://playhorizon.dev/docs/hosting) ([reference](docs/hosting.md)).

## Agent control

AI agents and scripts can play Horizon. `horizon-ctl`, shipped with the game, reads its state as JSON and acts like a player; `horizon-ctl mcp` serves the same commands over MCP.

```sh
horizon-ctl launch --world Survival --seed 5 --wait   # finds the version the launcher installed
horizon-ctl state                                       # position, health, food, target, screen...
horizon-ctl goto -0.5 65 -2.5 --near 2.5                # walks there along a path
horizon-ctl break -1 65 -3                              # mines it; replies with the drops
```

`horizon-ctl help` lists every command. [Agent control](docs/native-client.md#agent-control-horizon-ctl-and-mcp).

## Build from source

Needs Rust 1.90+, Python 3.10+, JDK 25, and on Linux `libasound2-dev libudev-dev`. `horizon-prepare` builds the game's resources from **your own copy** of Java Edition 26.3 (checked against `research-lock.json`) and never downloads it:

1. **Client:** play 26.3 once in the official launcher. Horizon reads `~/.minecraft` (or `--minecraft-dir` / `HORIZON_MINECRAFT_DIR`).
2. **Server jar:** the 26.3 server jar (`downloads.server.url` in `versions/26.3/26.3.json`), passed once with `--server-jar` or `HORIZON_SERVER_JAR`.

```sh
git clone https://github.com/horizon-rust/horizon.git
cd horizon
cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar   # needs JDK 25
cargo run --release -p horizon-prepare -- sounds   # optional: sounds from your launcher install
cargo run --release -p horizon-client
```

Prepared files land in the gitignored `.local/`: never commit or share them. Options, controls and architecture: [the native client guide](docs/native-client.md); all developer docs: [docs/README.md](docs/README.md).

## Contributing

Issues and pull requests are welcome: read [CONTRIBUTING.md](CONTRIBUTING.md) (vanilla first, small pull requests), and see [ROADMAP.md](ROADMAP.md) or [playhorizon.dev/hub](https://playhorizon.dev/hub) for what's next. Questions: [Discord](https://discord.gg/playhorizon). Before a pull request, run the self-check and paste its output:

```sh
git config core.hooksPath .githooks
python3 tools/pr_ready.py
```

It runs rustfmt, clippy, the touched crates' tests, the Python tests, the public-tree check and the vanilla harnesses your change affects. Never commit game files, decompiled source or prepared resources.

## Project status

**Player-visible gaps: 0 open** of the 833 the [port ledger](tools/port_ledger.py) found, reading every vanilla 26.3 method against Horizon's code: vanilla behaviour a player would notice that Horizon doesn't have yet. By area: Animals [0](https://github.com/horizon-rust/horizon/issues/3512); Monsters and bosses [0](https://github.com/horizon-rust/horizon/issues/3513); Entities, projectiles, vehicles and decorations [0](https://github.com/horizon-rust/horizon/issues/3514); Items, enchantments and inventories [0](https://github.com/horizon-rust/horizon/issues/3515); Blocks, redstone and fluids [0](https://github.com/horizon-rust/horizon/issues/3516); World, world generation and saves [0](https://github.com/horizon-rust/horizon/issues/3517); Screens, HUD and client packets [0](https://github.com/horizon-rust/horizon/issues/3518); Entity and block rendering, models [0](https://github.com/horizon-rust/horizon/issues/3519); Server and players on the server [0](https://github.com/horizon-rust/horizon/issues/3520); Mob AI, villagers and raids [0](https://github.com/horizon-rust/horizon/issues/3521). This number goes down as gaps close (`tools/parity_gaps.py`); the harness results below say that what is tested matches.

The table is the project's own audit: every subsystem matches vanilla in every harness that compares it. 100% means 100% of what the harnesses compare, not that nothing else differs. What players still run into: [known issues](docs/known-issues.md).

### Java parity: 100%

Scores: 0 (absent), 0.25 (started), 0.5 (playable, major features missing), 0.75 (mostly complete) or 1 (complete and verified by differential tests); the percentage is their average. Last audited 2026-10-13: the harnesses pass on Linux and macOS. The [port ledger](tools/port_ledger.py) adds a method-by-method view of what is ported.

| Subsystem | Score | Main gaps |
| --- | ---: | --- |
| Window, input and game loop | 1 | None: game loop, options, camera, auto-jump, toggle sprint/sneak and frame limiter match |
| Assets and resources | 1 | None: item models match over 196,290 stacks; the resource pack screen over 3,000 sessions |
| Network protocol and multiplayer | 1 | None: joins vanilla servers; the dedicated server matches vanilla's files, login, configuration, play and command tree; LAN discovery over 462 cases (offline mode only) |
| Chunk storage and palettes | 1 | None: client and saves share paletted containers, identical to vanilla over 266 scenarios |
| Section meshing | 1 | None: 112 scenes and over a million quads identical to vanilla |
| Rendering | 1 | None: entity models, block entity renderers (407 cases) and animations (296), HUD overlays (2,200 states), the recipe book (600), player list, subtitles and world border match |
| Lighting | 1 | None: saved and relit light matches vanilla, slabs and stairs included |
| World generation | 1 | None: 2,352 whole chunks over 225 structure sites, in all three dimensions, identical to vanilla |
| Region files and saves | 1 | None: saves round-trip with vanilla; worlds from 1.8 to 26.2 upgrade as vanilla's DataFixers do |
| Entities and physics | 1 | None: attributes, spawn rules, natural spawning (15,500 cases), raids and patrols (1,900), `finalizeSpawn` (24,000) and mob AI (92 scenarios) match |
| Interaction and inventory | 1 | None: menu clicks over 8,000 sequences; recipe book placement (6,000), buttons (1,200), saved book (1,500), loom and crafter screens (1,600) |
| Menus, HUD and chat | 1 | None: every command (758 over 39,685 inputs), scoreboards (81,000 operations), the player list (872,000 draw calls), chat and its suggestions (3,000 sessions) match |
| Audio | 1 | None: the sound engine, sound events, music, situational music, ambience and subtitles match vanilla |
| Block behaviours and ticking | 1 | None: block updates, random ticks, redstone, dispensers (6,000 cases), crafters (4,000), container spills (4,000), copper golems and the sculk family (435 scenarios) match |

## Licence

[MIT](LICENSE), for the project's original work; dependencies keep their own licences ([notices](THIRD_PARTY_NOTICES.md)). Horizon is made by [Ludovic Marie](https://lu4ovic.com) ([@ludovic111](https://github.com/ludovic111)) and [its contributors](https://github.com/horizon-rust/horizon/graphs/contributors); support it on [GitHub Sponsors](https://github.com/sponsors/ludovic111).

NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT. Minecraft is a trademark of Mojang Synergies AB; its code and assets belong to Mojang and Microsoft under the [Minecraft EULA](https://www.minecraft.net/en-us/eula).
