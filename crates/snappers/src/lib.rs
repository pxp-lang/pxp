use std::path::PathBuf;

#[macro_export]
macro_rules! snap {
    ($snapper:ident, $name:ident, $subject:expr) => {
        #[test]
        fn $name() {
            let snapper = $snapper();
            let subject = $subject;
            let snapshot = snapper.snapshot_path(stringify!($name));

            if !snapshot.exists() || snapper.should_regenerate_snapshots() {
                std::fs::create_dir_all(snapshot.parent().unwrap()).unwrap();
                std::fs::write(&snapshot, subject.to_string()).unwrap();

                println!("Snapshot created: {}", stringify!($name));
            } else {
                let expected = std::fs::read_to_string(&snapshot).unwrap();
                snapper.assert_eq(expected, format!("{}", subject));
            }
        }
    };
}

pub struct Snapper {
    directory: PathBuf,
}

impl Snapper {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    /// Returns the path to a particular snapshot file: $CARGO_MANIFEST_DIR/__snapshots__/$name.snap
    pub fn snapshot_path(&self, name: &str) -> PathBuf {
        let mut path = self.directory.clone();
        path.push(format!("{}.snap", name));
        path
    }

    pub fn should_regenerate_snapshots(&self) -> bool {
        std::env::var("SNAPPERS_REGENERATE").is_ok()
    }

    pub fn assert_eq(&self, expected: String, actual: String) {
        pretty_assertions::assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod tests {
    use crate::Snapper;

    use super::snap;

    snap!(snapper, it_can_say_hello_world, say_hello("world"));

    fn say_hello(name: &str) -> String {
        return format!("Hello, {name}!");
    }

    fn snapper() -> Snapper {
        Snapper::new(format!("{}/__snapshots__", env!("CARGO_MANIFEST_DIR")).into())
    }
}