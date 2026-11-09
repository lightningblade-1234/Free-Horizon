//! Local Minecraft discovery and the preparation mode's metadata-download policy.
use super::{sources, MinecraftFrom};
use std::path::{Path, PathBuf};

pub(super) fn resolve(
    from: MinecraftFrom,
    minecraft: &Path,
    pins: &sources::Pins,
    known: &[(&'static str, PathBuf)],
    linked: &Path,
    fetch: sources::Fetch,
    log: &dyn Fn(String),
) -> Result<sources::Resolved, String> {
    let offline_fetch = |url: &str| {
        Err(crate::i18n::trf(
            "Offline preparation needs a local Minecraft version JSON matching this build. Choose its Minecraft folder; no files were downloaded ({url}).",
            &[("url", url)],
        ))
    };
    let fetch: sources::Fetch = if from == MinecraftFrom::Local {
        &offline_fetch
    } else {
        fetch
    };
    // resolve first verifies a local client JAR. Only its missing pinned version JSON
    // can be fetched here; the checksum and managed-cache write belong to sources.
    sources::resolve(minecraft, pins, known, linked, fetch, log)
}

#[cfg(test)]
#[path = "minecraft_sources_tests.rs"]
mod tests;
