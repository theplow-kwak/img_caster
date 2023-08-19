use crossterm::event::{self, KeyCode, KeyEvent};

pub mod bitarray;
pub mod datafifo;
pub mod disk;
pub mod multicast;
pub mod packet;
pub mod receiver;
pub mod sender;
pub mod slice;
pub mod statistics;

pub const RUNNING: bool = true;
pub const ENDLOOP: bool = false;

pub const SECTOR_SIZE: usize = 512;
pub const CHUNK_SIZE: usize = SECTOR_SIZE * 512;
pub const READ_CHUNK: usize = CHUNK_SIZE * 16;
pub const WRITE_CHUNK: usize = CHUNK_SIZE * 16;
pub const MAX_BUFFER_SIZE: usize = CHUNK_SIZE * 32;

pub const BLOCK_SIZE: u32 = 512 * 3;
pub const UDP_PACK_SIZE: usize = 2048;

pub const MAX_CLIENTS: u32 = 1024;
pub const MAX_SLICE_SIZE: u32 = 2048;
pub const BITS_PER_CHAR: u32 = 8;

pub const CAP_NEW_GEN: u32 = 0x0001;
pub const CAP_BIG_ENDIAN: u32 = 0x0008;
pub const CAP_LITTLE_ENDIAN: u32 = 0x0010;
pub const CAP_ASYNC: u32 = 0x0020;
pub const SENDER_CAPABILITIES: u32 = CAP_NEW_GEN | CAP_BIG_ENDIAN;
pub const RECEIVER_CAPABILITIES: u32 = CAP_NEW_GEN | CAP_BIG_ENDIAN;

pub const FLAG_PASSIVE: u16 = 0x0010;
pub const FLAG_NOSYNC: u16 = 0x0040;
pub const FLAG_NOKBD: u16 = 0x0080;
pub const FLAG_SYNC: u16 = 0x0100;
pub const FLAG_STREAMING: u16 = 0x200;
pub const FLAG_IGNORE_LOST_DATA: u16 = 0x400;

pub const PORTBASE: u16 = 9000;

pub fn getch(secs: u64) -> Option<char> {
    if event::poll(std::time::Duration::from_secs(secs)).unwrap() {
        if let event::Event::Key(KeyEvent {
            code, modifiers: _, ..
        }) = event::read().unwrap()
        {
            if let KeyCode::Char(c) = code {
                return Some(c);
            }
            if let KeyCode::Enter = code {
                return Some('\r');
            }
        }
    }
    return None;
}
