use std::{fs, path};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn exists(&self) -> bool {
        self.exists()
    }

    fn is_writeable(&self) -> bool {
        // possibility 4 - with map & unwrap_or
        fs::metadata(self)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)

        // possibility 3 - with if let
        // if let Ok(m) = fs::metadata(self) {
        //     !m.permissions().readonly()
        // } else {
        //     false
        // }

        // possibility 2 - with let else
        // let Ok(m) = fs::metadata(self) else {
        //     return false;
        // };
        // println!("{:#?}", m);
        // true

        // possibility 1 - with match
        // let maybe_m = fs::metadata(self);
        // match maybe_m {
        //     Ok(m) => {
        //         println!("{:#?}", m);
        //         true
        //     }
        //     Err(_) => false,
        // }
    }

    fn is_readable(&self) -> bool {
        fs::File::open(&self).is_ok()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        let file = path::Path::new("foo.txt");
        assert!(file.exists());
    }

    #[test]
    fn test_is_readable() {
        let file = path::Path::new("foo.txt");
        assert!(file.is_readable());
    }

    #[test]
    fn test_is_writeable() {
        let file = path::Path::new("foo.txt");
        assert!(file.is_writeable());
    }
}
