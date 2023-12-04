#[cfg(feature = "i_like_breaking_production")]
use std::{io::{Write, Read}, path::Path};
#[cfg(feature = "i_like_breaking_production")]
use tempfile::Builder;

/// Struct with memory backed by a file.
/// The file is deleted when the struct is dropped.
/// The file is created in the system's temporary directory.
#[cfg(feature = "i_like_breaking_production")]
pub struct MemPickle {
    file: tempfile::NamedTempFile,
}

#[cfg(feature = "i_like_breaking_production")]
impl MemPickle {
    /// Create a new pickle from the given data, and write it to a temporary file.
    /// Copies all the memory from the given data to the file. so no serialization is done.
    pub fn new<T>(data: T) -> Result<Self, std::io::Error> {
        // access raw data using unsafe
        let ptr = &data as *const T as *const u8;
        let len = std::mem::size_of::<T>();
        let data = unsafe { std::slice::from_raw_parts(ptr, len) };

        // create a temporary file

        let file = Builder::new()
            .prefix("tmp_pickle")
            .suffix(".pickle")
            .tempfile()?;
        // write the data to the file

        file.as_file().write_all(data)?;

        // return the struct
        Ok(Self { file })
    }

    /// Get the path to the temporary file.
    pub fn path(&self) -> &Path {
        self.file.path()
    }


    /// Load the file back into memory and return it, dropping the file.
    pub fn unwrap<T>(self) -> Result<T, std::io::Error> {
        // read the file back into memory
        let mut file = self.file;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        // drop the file
        drop(file);

        // access raw data using unsafe
        let data: T = *unsafe { Box::from_raw(data.as_ptr() as *mut T) };

        // return the data
        Ok(data)
    }
}

#[cfg(feature = "i_like_breaking_production")]
impl Drop for MemPickle {
    fn drop(&mut self) {
        // delete the file
        let _ = self.file.close();
    }
}
