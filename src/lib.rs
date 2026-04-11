use serde_json::Error;

pub mod instance;
pub mod msg;


pub const SOCKET_ADDR: &str = "/tmp/blured.sock";


pub trait FromToJson
where
    Self: Sized,
{
    fn from_json_bytes(bytes: &[u8]) -> Result<Self, Error>;
    fn from_json_str(string: &str) -> Result<Self, Error>;
    fn to_json_bytes(&self) -> Result<Vec<u8>, Error>;
    fn to_json_str(&self) -> Result<String, Error>;
}


impl<T> FromToJson for T
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    fn from_json_bytes(bytes: &[u8]) -> Result<Self, Error>
    {
        serde_json::from_slice(bytes)
    }

    fn from_json_str(string: &str) -> Result<Self, Error>
    {
        serde_json::from_str(string)
    }

    fn to_json_bytes(&self) -> Result<Vec<u8>, Error>
    {
        serde_json::to_vec(self)
    }

    fn to_json_str(&self) -> Result<String, Error>
    {
        serde_json::to_string(self)
    }
}
