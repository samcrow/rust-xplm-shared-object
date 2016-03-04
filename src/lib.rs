use std::marker::PhantomData;

extern crate xplm;
use xplm::data::{Shared, Readable, ArrayWriteable, SearchError, ReadWrite};

extern crate serde;
use serde::ser::Serialize;
use serde::de::Deserialize;

extern crate serde_json;

///
/// A shared object that multiple plugins can access
///
pub struct SharedObject<T> {
    /// The dataref that stores the information
    data: Shared<Vec<u8>, ReadWrite>,
    /// Phantom data that stores T
    phantom: PhantomData<T>,
}

impl<T> SharedObject<T> where T: Serialize + Deserialize {
    ///
    /// Finds or creates a shared object with the provided name
    ///
    pub fn find(name: &str) -> Result<SharedObject<T>, SearchError> {
        let dataref = try!(Shared::find(name));

        Ok(SharedObject {
            data: dataref,
            phantom: PhantomData,
        })
    }

    ///
    /// Reads and returns this shared object
    ///
    pub fn read(&self) -> Result<T, Box<std::error::Error>> {
        let result = try!(serde_json::de::from_slice(&self.data.get()));
        Ok(result)
    }
    ///
    /// Writes this shared object
    ///
    pub fn write(&mut self, value: &T) -> Result<(), Box<std::error::Error>> {
        let bytes = try!(serde_json::ser::to_vec(value));
        self.data.set_from_slice(&bytes);
        Ok(())
    }
}
