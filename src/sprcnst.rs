// ui
pub const UI_WIDTH: u32 = 24;
pub const UI_FLAGS: u32 = 0; // BLIT_1BPP
pub const UI: [u8; 30] = [ 0x00,0x10,0x00,0x00,0x30,0x3e,0x00,0x60,0x06,0x01,0xc0,0x0a,0x03,0x00,0x12,0x06,0x00,0x22,0x1c,0x00,0x40,0x30,0x00,0x80,0x60,0x00,0x00,0xc0,0x00,0x00 ];


// atlas
pub const ATLAS_WIDTH: u32 = 48;
pub const ATLAS_FLAGS: u32 = 1; // BLIT_2BPP
pub const ATLAS: [u8; 312] = [ 0xaa,0xaa,0xa9,0xaa,0xaa,0xa9,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xa4,0xaa,0xaa,0xa4,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0x90,0xaa,0xaa,0x90,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xa9,0x40,0xaa,0xa9,0x40,0xaa,0xaa,0xa9,0x6a,0xaa,0xaa,0xaa,0xa4,0x00,0xaa,0xa4,0x00,0xaa,0xaa,0xa5,0x6a,0xaa,0xaa,0xaa,0x90,0x00,0xaa,0x90,0x00,0xaa,0xaa,0x94,0x56,0xaa,0xaa,0xa9,0x40,0x00,0xa9,0x40,0x00,0xaa,0xaa,0x51,0x05,0xaa,0xaa,0xa4,0x00,0x00,0xa4,0x00,0x00,0xaa,0xaa,0x41,0x01,0xaa,0xaa,0x90,0x00,0x00,0x90,0x00,0x00,0xaa,0xa9,0x40,0x41,0x6a,0xaa,0x40,0x00,0x00,0x40,0x00,0x00,0xaa,0xa5,0x00,0x10,0x6a,0xaa,0x50,0x00,0x00,0x50,0x00,0x00,0xaa,0x94,0x00,0x00,0x6a,0xaa,0x44,0x00,0x00,0x44,0x00,0x00,0xaa,0x50,0x00,0x00,0x5a,0xaa,0x41,0x40,0x00,0x41,0x40,0x00,0xaa,0x40,0x10,0x40,0x16,0xaa,0x40,0x10,0x00,0x90,0x10,0x00,0xa9,0x40,0x10,0x10,0x05,0x6a,0x40,0x04,0x00,0xa4,0x04,0x00,0xa5,0x00,0x40,0x10,0x00,0x5a,0x40,0x01,0x40,0xa9,0x41,0x40,0x94,0x01,0x00,0x14,0x00,0x16,0x40,0x00,0x10,0xaa,0x90,0x10,0x50,0x01,0x00,0x11,0x00,0x05,0x90,0x00,0x04,0xaa,0xa4,0x04,0x90,0x04,0x00,0x11,0x00,0x06,0xa4,0x00,0x01,0xaa,0xa9,0x41,0xa4,0x00,0x00,0x10,0x50,0x1a,0xa9,0x40,0x01,0xaa,0xaa,0x91,0xa9,0x40,0x00,0x40,0x01,0x6a,0xaa,0x90,0x01,0xaa,0xaa,0xa5,0xaa,0x90,0x00,0x40,0x06,0xaa,0xaa,0xa4,0x01,0xaa,0xaa,0xa9,0xaa,0xa4,0x00,0x00,0x1a,0xaa,0xaa,0xa9,0x41,0xaa,0xaa,0xaa,0xaa,0xa9,0x40,0x01,0x6a,0xaa,0xaa,0xaa,0x91,0xaa,0xaa,0xaa,0xaa,0xaa,0x90,0x06,0xaa,0xaa,0xaa,0xaa,0xa5,0xaa,0xaa,0xaa,0xaa,0xaa,0xa4,0x1a,0xaa,0xaa,0xaa,0xaa,0xa9,0xaa,0xaa,0xaa,0xaa,0xaa,0xa9,0x6a,0xaa,0xaa ];


// chars
pub const CHARS_WIDTH: u32 = 28;
pub const CHARS_FLAGS: u32 = 1; // BLIT_2BPP
pub const CHARS: [u8; 1260] = [ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x2a,0x80,0x00,0x00,0x00,0x00,0x00,0xaa,0xa0,0x00,0x00,0x00,0x00,0x02,0x6b,0xa0,0x00,0x00,0x00,0x00,0x0a,0x9a,0xa4,0x00,0x00,0x00,0x00,0x2a,0xa5,0x5a,0x00,0x00,0x00,0x00,0xaa,0xaa,0xaa,0x80,0x00,0x00,0x00,0xa9,0xaa,0xaa,0x80,0x00,0x00,0x00,0xa9,0xaa,0xaa,0x80,0x00,0x00,0x00,0xaa,0x6a,0xa6,0x00,0x00,0x00,0x00,0x2a,0x9a,0xa8,0x00,0x00,0x00,0x00,0x0a,0x98,0xa8,0x00,0x00,0x00,0x00,0x05,0x60,0x2a,0x00,0x00,0x00,0x00,0x02,0xa0,0x2a,0x80,0x00,0x00,0x00,0x0a,0xa0,0x2a,0x80,0x00,0x00,0x00,0x0a,0xa0,0x00,0x00,0x00,0x00,0x00,0x02,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0a,0x00,0x00,0x00,0x00,0x0a,0xa0,0x2a,0x00,0x00,0x00,0x00,0x09,0x60,0x00,0x00,0x00,0x02,0xa0,0x26,0xa0,0x02,0xa0,0x00,0x09,0x60,0x2a,0xa0,0x02,0xaa,0x00,0x26,0xa0,0x2a,0xa0,0x02,0xaa,0xa0,0x2a,0xa2,0xa9,0xa2,0x8a,0xaa,0xa0,0x2a,0xaa,0xa9,0xa2,0xa0,0xaa,0xa0,0x2a,0xab,0xa9,0xa0,0x00,0x20,0x02,0x29,0xaa,0xaa,0xa0,0x00,0x80,0x0a,0x29,0xaa,0x2a,0xa0,0x02,0xa0,0x00,0x29,0xa0,0x2a,0x80,0x00,0xa0,0x00,0x2a,0xa0,0x28,0x00,0x00,0x00,0x00,0x2a,0x80,0x00,0x00,0x00,0x00,0x00,0x28,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x20,0xa0,0x00,0x00,0x00,0x00,0x00,0x0a,0xa0,0x00,0x00,0x00,0x00,0x00,0xaa,0x00,0x00,0x00,0x00,0x00,0x0a,0xa0,0x00,0x00,0x00,0x00,0x00,0xaa,0x00,0x00,0x00,0x00,0x00,0x0a,0xaa,0x00,0x00,0x00,0x00,0x00,0xaa,0x0a,0x00,0x00,0x00,0x00,0x00,0xaa,0x0a,0x80,0x00,0x00,0x00,0x00,0x0a,0xaa,0xa0,0x00,0x00,0x00,0x00,0x2a,0xaa,0x69,0x60,0x00,0x00,0x00,0x2a,0xa9,0x69,0xa0,0x00,0x00,0x00,0x5a,0xa5,0xa8,0xa0,0x00,0x00,0x00,0x95,0xaa,0xa4,0xa0,0x00,0x00,0x00,0xa6,0x4a,0x98,0x80,0x00,0x00,0x00,0xa0,0x80,0x28,0x00,0x00,0x00,0x00,0xa0,0x00,0x28,0x00,0x00,0x00,0x00,0x20,0x00,0x28,0x00,0x00,0x00,0x00,0x00,0x00,0x20,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0xa0,0x00,0x00,0x00,0x00,0x00,0x0a,0x20,0x00,0x00,0x00,0x00,0x00,0x20,0x28,0x00,0x00,0x00,0x00,0x00,0x20,0x08,0x00,0x00,0x00,0x00,0x00,0x20,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0xa0,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x09,0x80,0x00,0x00,0x00,0x00,0x00,0x09,0x80,0x00,0x00,0x28,0x00,0x00,0x26,0x02,0x80,0x00,0x96,0x00,0x00,0x98,0x09,0x60,0x02,0x68,0x00,0x00,0x98,0xa6,0x80,0x09,0x80,0x80,0x00,0x26,0x58,0x00,0x09,0x8a,0x68,0x00,0x29,0xa0,0x00,0x02,0x65,0x96,0x26,0xaa,0xa8,0x00,0x02,0x9a,0x28,0x1a,0x80,0xaa,0x02,0x6a,0xaa,0x80,0x2a,0x40,0x56,0x01,0xa8,0x0a,0xa0,0x29,0x80,0xa5,0x02,0xa4,0x05,0x60,0x00,0x00,0xa8,0x02,0x98,0x0a,0x50,0x00,0x01,0x54,0x00,0x00,0x0a,0x80,0x00,0x02,0xa0,0x00,0x00,0x15,0x40,0x00,0x0a,0x80,0x00,0x00,0x2a,0x00,0x00,0x28,0x00,0x00,0x00,0xa8,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x02,0x6a,0x40,0x00,0x00,0x00,0x00,0x0a,0x99,0x80,0x00,0x00,0x00,0x00,0x06,0xa6,0xa0,0x00,0x00,0x00,0x00,0x09,0xaa,0xa0,0x00,0x00,0x00,0x00,0x2a,0x6a,0x90,0x00,0x00,0x00,0x00,0x09,0xaa,0x60,0x00,0x00,0x00,0x00,0x06,0xa9,0xa8,0x00,0x00,0x00,0x00,0x0a,0xaa,0x60,0x00,0x00,0x00,0x00,0x09,0xaa,0x90,0x00,0x00,0x00,0x00,0x0a,0xa6,0xa0,0x00,0x00,0x00,0x00,0x16,0xaa,0x94,0x00,0x00,0x00,0x00,0x24,0x6a,0x58,0x00,0x00,0x00,0x00,0x80,0x91,0x42,0x00,0x00,0x00,0x00,0x82,0x00,0x82,0x00,0x00,0x00,0x00,0x22,0x00,0x8a,0x00,0x00,0x00,0x00,0xa2,0x82,0x00,0x00,0x00,0x00,0x00,0x00,0xa2,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x28,0x00,0x00,0x00,0x00,0x00,0x00,0xa8,0x00,0x00,0x00,0x00,0x00,0x02,0xaa,0x80,0x00,0x00,0x00,0x00,0x0a,0xa5,0xa0,0x00,0x00,0x00,0x00,0x0a,0x99,0xa0,0x00,0x00,0x00,0x00,0x2a,0x6a,0x60,0x00,0x00,0x00,0x00,0xa9,0xaa,0x80,0x00,0x00,0x00,0x00,0xa6,0xaa,0x00,0x00,0x00,0x00,0x92,0x9a,0xa0,0x80,0x00,0x00,0x00,0x6a,0xaa,0x08,0x80,0x00,0x00,0x00,0xa9,0x20,0x88,0x00,0x00,0x00,0x00,0xa6,0x08,0x80,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x2a,0x82,0x80,0x00,0x00,0x00,0x00,0xaa,0xa6,0xa0,0x00,0x00,0x00,0x00,0xaa,0xa6,0xa0,0x00,0x00,0x00,0x02,0xaa,0xa6,0xa0,0x00,0x00,0x00,0x02,0xaa,0x9a,0x80,0x00,0x00,0x00,0x02,0x6a,0x98,0x20,0x00,0x00,0x00,0x2a,0xa6,0x82,0x20,0x00,0x00,0x00,0xa2,0xaa,0x22,0x00,0x00,0x00,0x00,0x82,0xa8,0x20,0x00,0x00,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x0a,0x00,0x00,0x00,0x00,0x00,0x00,0x28,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xa0,0x00,0x00,0x00,0x00,0x00,0x02,0x02,0x80,0x00,0x00,0x00,0x00,0x02,0x00,0x20,0x00,0x00,0x00,0x00,0x02,0x00,0x20,0x00,0x00,0x00,0x00,0x02,0x00,0x98,0x00,0x00,0x00,0x00,0x00,0x8a,0x9a,0x00,0x00,0x00,0x00,0x02,0xaa,0x9a,0x80,0x00,0x00,0x00,0x02,0xaa,0x9a,0x80,0x00,0x00,0x00,0x0a,0xaa,0x6a,0x00,0x00,0x00,0x00,0x09,0xaa,0xa0,0x80,0x00,0x00,0x00,0x22,0x6a,0x08,0x80,0x00,0x00,0x00,0x20,0xa0,0x88,0x00,0x00,0x00,0x00,0x00,0x08,0x80,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00 ];


