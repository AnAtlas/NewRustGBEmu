
use crate::types::Address;
use std::ops::Range;
use std::slice::SliceIndex;

const HEADER_SIZE: usize = 0x150;
const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;
const LOGO_SIZE: usize = 0x30;
const MAX_ROM_BANKS: usize = 96;
const MAX_RAM_BANKS: usize = 16;

enum GameboyMode {
    GameBoy,
    GameBoyColor,
}

enum SuperGameboy {
    NotSupported,
    Supported,
}

enum RAM {
    None,
    Some(u8),
}

enum ROM {
    RomOnly,
    Banks(u8),
}

enum Battery {
    None,
    Present,
}

enum Timer {
    None,
    Present,
}

enum Rumble {
    None,
    Present,
}

enum CartridgeType {
    RomOnly,
    MBC1(ROM, RAM, Battery),
    MBC2(ROM, Battery),
    MBC3(ROM, RAM, Battery, Timer),
    MBC5(ROM, RAM, Battery, Rumble),
}

enum CartridgeTypeError {
    NotSupported(u8),
}

enum CartridgeAddress {
    Logo = 0x104,
    LogoEnd = 0x133,
    Title = 0x134,
    ColorGameboyFlag = 0x143,
}

pub struct Header {
    logo: [u8; LOGO_SIZE],
    title: String,
    gameboy_mode: GameboyMode,
    super_gameboy: SuperGameboy,
    cartridge_type: CartridgeType,
    rom_banks: usize,
    ram_banks: usize,
}

impl Into<usize> for CartridgeAddress {
    fn into(self) -> usize {
        self as usize
    }
}

impl From<u8> for GameboyMode {
    fn from(byte: u8) -> Self {
        match byte & 0x80 {
            0x80 => GameboyMode::GameBoyColor,
            _ => GameboyMode::GameBoy,
        }
    }
}

impl TryFrom<u8> for CartridgeType {
    type Error = CartridgeTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CartridgeType::RomOnly),

            _ => Err(CartridgeTypeError::NotSupported(value)),
        }
    }
}

pub trait Cartridge {
    fn read(&self, address: Address) -> u8;
    fn write(&self, address: Address, value: u8);

    fn read_header(bytes: &Vec<u8>) -> Result<Header, ()> {
        if bytes.len() < HEADER_SIZE {
            return Err(())
        }

        let mut header = Header {
            logo: [0; LOGO_SIZE],
            title: "".to_string(),
            gameboy_mode: GameboyMode::from(bytes[CartridgeAddress::ColorGameboyFlag.into()]),
            super_gameboy: SuperGameboy::NotSupported,
            cartridge_type: CartridgeType::RomOnly,
            rom_banks: 0,
            ram_banks: 0
        };
        header.logo.copy_from_slice(&bytes[CartridgeAddress::Logo.into()..CartridgeAddress::LogoEnd.into()]);
        Err(())
    }
}