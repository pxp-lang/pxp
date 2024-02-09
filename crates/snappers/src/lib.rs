use std::path::PathBuf;

#[macro_export]
macro_rules! snap {
    ($snapper:ident, $name:ident, $subject:expr) => {
        #[test]
        fn $name() {
            let subject = $subject;
            let snapshot = $snapper().snapshot_path(stringify!($name));

            if !snapshot.exists() {
                std::fs::create_dir_all(snapshot.parent().unwrap()).unwrap();
                std::fs::write(&snapshot, subject.to_string()).unwrap();

                println!("Snapshot created: {}", stringify!($name));
            } else {
                let expected = std::fs::read_to_string(&snapshot).unwrap();
                assert_eq!(expected, format!("{}", subject));
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
        Snapper::new(env!("CARGO_MANIFEST_DIR").into())
    }
}