use std::path::PathBuf;

#[macro_export]
macro_rules! snap {
    ($name:ident, $subject:expr) => {
        #[test]
        fn $name() {
            let subject = $subject;
            let snapshot = $crate::Snapper::snapshot_path(stringify!($name));

            if !snapshot.exists() {
                std::fs::create_dir_all(snapshot.parent().unwrap()).unwrap();
                std::fs::write(&snapshot, subject.to_string()).unwrap();
            } else {
                let expected = std::fs::read_to_string(&snapshot).unwrap();
                assert_eq!(expected, format!("{}", subject));
            }
        }
    };
}

pub struct Snapper;

impl Snapper {
    /// Returns the path to the snapshot directory: $CARGO_MANIFEST_DIR/__snapshots__
    pub fn snapshot_directory() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("__snapshots__");
        path
    }

    /// Returns the path to a particular snapshot file: $CARGO_MANIFEST_DIR/__snapshots__/$name.snap
    pub fn snapshot_path(name: &str) -> PathBuf {
        let mut path = Self::snapshot_directory();
        path.push(format!("{}.snap", name));
        path
    }
}

#[cfg(test)]
mod tests {
    use super::snap;

    snap!(it_can_say_hello_world, say_hello("world"));

    fn say_hello(name: &str) -> String {
        return format!("Hello, {name}!");
    }
}