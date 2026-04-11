use crate::FromToJson;
use crate::msg::{Action, InstanceIdentifier, Message, Response, Status};
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
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
    reader: BufReader<UnixStream>,
    writer: BufWriter<UnixStream>,
}


impl Instance
{
    pub fn connect_with(filter: InstanceFilter, addr: impl AsRef<Path>) -> Result<Self>
    {
        let stream = UnixStream::connect(addr)?;

        Ok(Self {
            filter,
            reader: BufReader::new(stream.try_clone()?),
            writer: BufWriter::new(stream),
        })
    }

    pub fn connect() -> Result<Self>
    {
        Self::connect_with(InstanceFilter::default(), crate::SOCKET_ADDR)
    }

    pub fn write_blocking(&mut self, action: Action) -> Result<()>
    {
        serde_json::to_writer(&mut self.writer, &self.filter.create_msg(action))?;
        self.writer.write_all(b"\0")?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn read_blocking(&mut self) -> Result<Status>
    {
        let mut buf = Vec::with_capacity(256);

        loop
        {
            self.reader.read_until(b'\0', &mut buf)?;
            let response = Response::from_json_bytes(&buf[..buf.len().saturating_sub(1)])?;
            buf.clear();

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
