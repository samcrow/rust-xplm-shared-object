use std::marker::PhantomData;

extern crate xplm;
use xplm::data::{Shared, Readable, ArrayWriteable, DataAccess, SearchError, ReadWrite};

extern crate serde;
use serde::ser::Serialize;
use serde::de::Deserialize;

extern crate serde_json;

///
/// A shared object that multiple plugins can access
///
pub struct SharedObject<T, A> {
    /// The dataref that stores the information
    data: Shared<Vec<u8>, A>,
    /// Phantom data that stores T
    phantom: PhantomData<T>,
}

impl<T, A> SharedObject<T, A> where T: Serialize + Deserialize, A: DataAccess {
    ///
    /// Finds or creates a shared object with the provided name
    ///
    pub fn find(name: &str) -> Result<SharedObject<T, A>, SearchError> {
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
}

impl<T> SharedObject<T, ReadWrite> where T: Serialize + Deserialize {
    ///
    /// Writes this shared object
    ///
    pub fn write(&mut self, value: &T) -> Result<(), Box<std::error::Error>> {
        let bytes = try!(serde_json::ser::to_vec(value));
        self.data.set_from_slice(&bytes);
        Ok(())
    }
}
