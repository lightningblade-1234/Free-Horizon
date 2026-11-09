use super::*;
use sha1::{Digest, Sha1};
use std::fs;

const CLIENT: &[u8] = b"local client fixture, not Minecraft";
const MANIFEST: &[u8] = br#"{"id":"9.9","fixture":true}"#;

struct Fixture {
    root: PathBuf,
    pins: sources::Pins,
}

impl Fixture {
    fn new(name: &str, manifest: Option<&[u8]>) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let hash = |bytes: &[u8]| format!("{:x}", Sha1::digest(bytes));
        let fixture = Self {
            root: std::env::temp_dir().join(format!(
                "horizon-local-metadata-{name}-{}-{nonce}",
                std::process::id()
            )),
            pins: sources::Pins {
                version: "9.9".into(),
                client_sha1: hash(CLIENT),
                manifests: vec![hash(MANIFEST)],
            },
        };
        fixture.write("minecraft/versions/9.9/9.9.jar", CLIENT);
        fixture.write("minecraft/libraries/fixture.jar", b"local library");
        fixture.write("minecraft/assets/indexes/fixture.json", b"{}");
        if let Some(manifest) = manifest {
            fixture.write("minecraft/versions/9.9/9.9.json", manifest);
        }
        fixture
    }

    fn write(&self, relative: &str, bytes: &[u8]) {
        let path = self.root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn linked(&self) -> PathBuf {
        self.root.join("managed")
    }

    fn resolve(
        &self,
        from: MinecraftFrom,
        fetch: sources::Fetch,
    ) -> Result<sources::Resolved, String> {
        resolve(
            from,
            &self.root.join("minecraft"),
            &self.pins,
            &[],
            &self.linked(),
            fetch,
            &|_| {},
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn automatic_setup_recovers_missing_or_rewritten_metadata_and_reuses_the_cache() {
    for original in [
        None,
        Some(br#"{"id":"9.9","launcher_rewrite":true}"#.as_slice()),
    ] {
        let fixture = Fixture::new("recover", original);
        let expected_url = format!(
            "https://piston-meta.mojang.com/v1/packages/{}/9.9.json",
            fixture.pins.manifests[0]
        );
        let got = fixture
            .resolve(MinecraftFrom::LocalSetup, &|url| {
                assert_eq!(url, expected_url);
                Ok(MANIFEST.to_vec())
            })
            .unwrap();
        assert_eq!(got, sources::Resolved::Folder(fixture.linked()));
        assert_eq!(
            fs::read(fixture.linked().join("versions/9.9/9.9.json")).unwrap(),
            MANIFEST
        );
        assert_eq!(
            fs::read(fixture.linked().join("versions/9.9/9.9.jar")).unwrap(),
            CLIENT
        );
        assert_eq!(
            fs::read(fixture.linked().join("libraries/fixture.jar")).unwrap(),
            b"local library"
        );
        assert_eq!(
            fs::read(fixture.linked().join("assets/indexes/fixture.json")).unwrap(),
            b"{}"
        );
        let original_path = fixture.root.join("minecraft/versions/9.9/9.9.json");
        assert_eq!(fs::read(original_path).ok().as_deref(), original);
        assert_eq!(
            fixture
                .resolve(MinecraftFrom::LocalSetup, &|_| panic!("metadata is cached"))
                .unwrap(),
            got
        );
    }
}

#[test]
fn strict_local_preparation_still_requires_local_metadata_without_fetching() {
    let fixture = Fixture::new("strict", None);
    let error = fixture
        .resolve(MinecraftFrom::Local, &|_| panic!("strict-local download"))
        .unwrap_err();
    assert!(error.contains("local Minecraft version JSON"), "{error}");
    assert!(!fixture.linked().join("versions/9.9/9.9.json").exists());
    assert_eq!(
        fs::read(fixture.root.join("minecraft/versions/9.9/9.9.jar")).unwrap(),
        CLIENT
    );
}

#[test]
fn automatic_setup_rejects_bad_metadata_without_overwriting_cached_or_local_files() {
    let fixture = Fixture::new("checksum", None);
    fixture.write("managed/versions/9.9/9.9.json", b"interrupted metadata");
    let error = fixture
        .resolve(MinecraftFrom::LocalSetup, &|_| {
            Ok(b"wrong download".to_vec())
        })
        .unwrap_err();
    assert!(error.contains("not the one Horizon pins"), "{error}");
    assert_eq!(
        fs::read(fixture.linked().join("versions/9.9/9.9.json")).unwrap(),
        b"interrupted metadata"
    );
    assert_eq!(
        fs::read(fixture.root.join("minecraft/versions/9.9/9.9.jar")).unwrap(),
        CLIENT
    );
    assert!(!fixture.linked().join("versions/9.9/9.9.jar").exists());
}

#[test]
fn automatic_setup_needs_a_matching_local_client_before_requesting_metadata() {
    let fixture = Fixture::new("missing-client", None);
    fs::remove_file(fixture.root.join("minecraft/versions/9.9/9.9.jar")).unwrap();
    for client in [None, Some(b"modified client".as_slice())] {
        if let Some(client) = client {
            fixture.write("minecraft/versions/9.9/9.9.jar", client);
        }
        let got = fixture
            .resolve(MinecraftFrom::LocalSetup, &|_| {
                panic!("no matching client was found")
            })
            .unwrap();
        assert!(matches!(got, sources::Resolved::NotFound(_)));
        assert!(!fixture.linked().exists());
    }
}

#[test]
fn automatic_setup_reuses_accepted_local_metadata_without_fetching() {
    let fixture = Fixture::new("accepted", Some(MANIFEST));
    assert_eq!(
        fixture
            .resolve(MinecraftFrom::LocalSetup, &|_| panic!("metadata is local"))
            .unwrap(),
        sources::Resolved::Folder(fixture.root.join("minecraft"))
    );
    assert!(!fixture.linked().exists());
}
