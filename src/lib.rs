#![no_std]
use embedded_hal_async::i2c::{ErrorType, I2c, Operation};
use heapless::Vec;

pub struct SmbusAdaptor<BUS: I2c> {
    i2c: BUS,
}

impl<BUS: I2c> SmbusAdaptor<BUS> {
    pub fn new(i2c: BUS) -> Self {
        Self { i2c }
    }

    pub async fn quick_command(&mut self, addr: u8, bit: bool) -> Result<(), BUS::Error> {
        if bit {
            self.i2c.read(addr, &mut []).await
        } else {
            self.i2c.write(addr, &[]).await
        }
    }

    pub async fn receive_byte(&mut self, addr: u8) -> Result<u8, BUS::Error> {
        let mut buf = [0x00];
        self.i2c.read(addr, &mut buf).await?;

        Ok(buf[0])
    }

    pub async fn send_byte(&mut self, addr: u8, byte: u8) -> Result<(), BUS::Error> {
        self.i2c.write(addr, &[byte]).await
    }

    pub async fn read_byte(&mut self, addr: u8, register: u8) -> Result<u8, BUS::Error> {
        let mut buf = [0x00];
        self.i2c.write_read(addr, &[register], &mut buf).await?;

        Ok(buf[0])
    }

    pub async fn read_word(&mut self, addr: u8, register: u8) -> Result<u16, BUS::Error> {
        let mut buf = [0x00; 2];
        self.i2c.write_read(addr, &[register], &mut buf).await?;

        Ok(u16::from_le_bytes(buf))
    }

    pub async fn write_byte(&mut self, addr: u8, register: u8, byte: u8) -> Result<(), BUS::Error> {
        self.i2c.write(addr, &[register, byte]).await
    }

    pub async fn write_word(
        &mut self,
        addr: u8,
        register: u8,
        word: u16,
    ) -> Result<(), BUS::Error> {
        let word_bytes = word.to_le_bytes();
        self.i2c
            .write(addr, &[register, word_bytes[0], word_bytes[1]])
            .await
    }

    pub async fn process_call(
        &mut self,
        addr: u8,
        register: u8,
        word: u16,
    ) -> Result<u16, BUS::Error> {
        let word_bytes = word.to_le_bytes();
        let cmd = [register, word_bytes[0], word_bytes[1]];
        let mut buff = [0x00; 2];
        let mut ops = [Operation::Write(&cmd), Operation::Read(&mut buff)];

        self.i2c.transaction(addr, &mut ops).await?;

        Ok(u16::from_le_bytes(buff))
    }

    pub async fn block_read(&mut self, addr: u8, register: u8) -> Result<Vec<u8, 32>, BUS::Error> {
        let mut v = Vec::new();
        v.resize_default(32).unwrap();

        self.i2c.write_read(addr, &[register], &mut v).await?;

        let len = core::cmp::min(v[0] + 1, 32) as usize;
        v.resize_default(len).unwrap();
        Ok(v)
    }

    pub async fn block_write(
        &mut self,
        addr: u8,
        register: u8,
        data: &[u8],
    ) -> Result<(), BUS::Error> {
        let cmd = [register, data.len() as u8];
        let mut ops = [Operation::Write(&cmd), Operation::Write(&data)];

        self.i2c.transaction(addr, &mut ops).await
    }

    pub async fn block_read_process_call(
        &mut self,
        addr: u8,
        register: u8,
        data: &[u8],
    ) -> Result<Vec<u8, 32>, BUS::Error> {
        let cmd = [register, data.len() as u8];
        let mut v = Vec::new();
        v.resize_default(32).unwrap();
        let mut ops = [
            Operation::Write(&cmd),
            Operation::Write(&data),
            Operation::Read(&mut v),
        ];

        self.i2c.transaction(addr, &mut ops).await?;

        let len = core::cmp::min(v[0] + 1, 32) as usize;
        v.resize_default(len).unwrap();
        Ok(v)
    }
}

impl<'a, BUS> ErrorType for SmbusAdaptor<BUS>
where
    BUS: ErrorType + I2c,
{
    type Error = BUS::Error;
}

impl<BUS> I2c for SmbusAdaptor<BUS>
where
    BUS: I2c + 'static,
{
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), BUS::Error> {
        self.i2c.read(address, read).await?;
        Ok(())
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), BUS::Error> {
        self.i2c.write(address, write).await?;
        Ok(())
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), BUS::Error> {
        self.i2c.write_read(address, write, read).await?;
        Ok(())
    }

    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), BUS::Error> {
        self.i2c.transaction(address, operations).await?;
        Ok(())
    }
}
