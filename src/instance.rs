use crate::msg::{Action, InstanceIdentifier, Message, Response, Status};
use std::io::Result;
use std::os::unix::net::UnixStream;
use std::path::Path;


#[derive(Default)]
pub struct InstanceFilter
{
    pub(crate) instance: InstanceIdentifier,
}


impl InstanceFilter
{
    pub fn with_instance(instance: InstanceIdentifier) -> Self
    {
        Self { instance }
    }

    pub fn create_msg(&self, action: Action) -> Message
    {
        Message {
            instance: self.instance.clone(),
            action,
        }
    }

    pub fn filter<'a>(&self, response: &'a Response) -> Option<&'a Response>
    {
        if response.instance == self.instance
        {
            return Some(response);
        }

        None
    }

    pub fn filter_owned(&self, response: Response) -> Option<Status>
    {
        if response.instance == self.instance
        {
            return Some(response.status);
        }

        None
    }
}


pub struct Instance
{
    filter: InstanceFilter,
    socket: UnixStream,
}


impl Instance
{
    pub fn connect_with(filter: InstanceFilter, addr: impl AsRef<Path>) -> Result<Self>
    {
        Ok(Self {
            filter,
            socket: UnixStream::connect(addr)?,
        })
    }

    pub fn connect() -> Result<Self>
    {
        Self::connect_with(InstanceFilter::default(), crate::SOCKET_ADDR)
    }

    pub fn write_blocking(&mut self, action: Action) -> Result<()>
    {
        Ok(serde_json::to_writer(
            &mut self.socket,
            &self.filter.create_msg(action),
        )?)
    }

    pub fn read_blocking(&mut self) -> Result<Status>
    {
        loop
        {
            let response = serde_json::from_reader::<_, Response>(&mut self.socket)?;

            if let Some(status) = self.filter.filter_owned(response)
            {
                return Ok(status);
            }
        }
    }

    pub fn message(&mut self, action: Action) -> Result<Status>
    {
        self.write_blocking(action)?;
        self.read_blocking()
    }
}
