
use crate::cartridge;
const MAX_WORK_RAM_SIZE: usize = 0x8000;

struct Memory {
    work_ram: [u8; MAX_WORK_RAM_SIZE],
}