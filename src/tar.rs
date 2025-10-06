use std::{char, fmt::Display};

pub const BLOCK_SIZE: usize = 512;
pub const TMAGIC: &str = "ustar\0"; // ustar\0
pub const TMAGIC_LEN: usize = 6;
pub const TVERSION: &str = "00"; // 00
pub const TVERSION_LEN: usize = 2;

// typeflag values
pub const REGTYPE: u8 = b'0'; // regular file
pub const AREGTYPE: u8 = b'\0'; // regular file
pub const LNKTYPE: u8 = b'1'; // link
pub const SYMTYPE: u8 = b'2'; // "reserved"
pub const CHRTYPE: u8 = b'3'; // character special
pub const BLKTYPE: u8 = b'4'; // block special
pub const DIRTYPE: u8 = b'5'; // directory
pub const FIFOTYPE: u8 = b'6'; // FIFO special
pub const CONTTYPE: u8 = b'7'; // reserved

// Bits used in mode fields
const TSUID: u32 = 0o4000; // set UID on execution
const TSGID: u32 = 0o2000; // set GID on execution
const TSVTX: u32 = 0o1000; // reserved
// file permissions
const TUREAD: u32 = 0o0400; // read by owner
const TUWRITE: u32 = 0o0200; // write by owner
const TUEXEC: u32 = 0o0100; // execute/search by owner
const TGREAD: u32 = 0o0040; // read by group
const TGWRITE: u32 = 0o0020; // write by group
const TGEXEC: u32 = 0o0010; // execute/search by group
const TOREAD: u32 = 0o0004; // read by other
const TOWRITE: u32 = 0o0002; // write by other
const TOEXEC: u32 = 0o0001; // execute/search by other

//struct posix_header
//{                              /* byte offset */
//  char name[100];               /*   0 */
//  char mode[8];                 /* 100 */
//  char uid[8];                  /* 108 */
//  char gid[8];                  /* 116 */
//  char size[12];                /* 124 */
//  char mtime[12];               /* 136 */
//  char chksum[8];               /* 148 */
//  char typeflag;                /* 156 */
//  char linkname[100];           /* 157 */
//  char magic[6];                /* 257 */
//  char version[2];              /* 263 */
//  char uname[32];               /* 265 */
//  char gname[32];               /* 297 */
//  char devmajor[8];             /* 329 */
//  char devminor[8];             /* 337 */
//  char prefix[155];             /* 345 */
//                                /* 500 */
//};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PosixHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    chksum: [u8; 8],
    typeflag: u8,
    linkname: [u8; 100],
    magic: [u8; 6],
    version: [u8; 2],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 155],
}

impl Display for PosixHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PosixHeader {{
    name: {},
    mode: {},
    uid: {},
    gid: {},
    size: {},
    mtime: {},
    chksum: {},
    typeflag: {},
    linkname: {},
    magic: {},
    version: {},
    uname: {},
    gname: {},
    devmajor: {},
    devminor: {},
    prefix: {}
}}",
            lossy_string(&self.name),
            lossy_string(&self.mode),
            lossy_string(&self.uid),
            lossy_string(&self.gid),
            self.size(),
            lossy_string(&self.mtime),
            lossy_string(&self.chksum),
            lossy_string(&[self.typeflag]),
            lossy_string(&self.linkname),
            lossy_string(&self.magic),
            lossy_string(&self.version),
            lossy_string(&self.uname),
            lossy_string(&self.gname),
            lossy_string(&self.devmajor),
            lossy_string(&self.devminor),
            lossy_string(&self.prefix),
        )
    }
}

fn lossy_string(s: &[u8]) -> String {
    String::from_utf8_lossy(s)
        .trim_end_matches(char::from(0))
        .to_string()
}

impl PosixHeader {
    pub fn size(&self) -> usize {
        let s = lossy_string(&self.size);
        usize::from_str_radix(s.trim(), 8).unwrap_or(0)
    }
    
}
