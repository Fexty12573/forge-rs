// Pure-Rust ELF -> NRO conversion.

use anyhow::{Context, Result, bail};
use std::path::Path;

// ELF / NRO constants
const PT_LOAD: u32 = 1;
const EM_ARM: u16 = 40;
const SHT_NOTE: u32 = 7;
const NT_GNU_BUILD_ID: u32 = 3;

const ELF_HEADER_SIZE: usize = 52;
const PHDR_SIZE: usize = 32;
const PAGE: u32 = 0x1000;

/// Size of the leading NroStart (MOD0 pointer block) written by crt0.
/// The NRO header begins immediately after it.
const NRO_START_SIZE: usize = 0x10;
const NRO_HEADER_SIZE: usize = 0x70;

/// One loadable segment, as extracted from the ELF program headers.
struct LoadSegment {
    offset: u32,
    vaddr: u32,
    filesz: u32,
    memsz: u32,
}

/// Reads elf_path, converts it to an NRO, and writes the result to nro_path.
pub fn convert(elf_path: &Path, nro_path: &Path) -> Result<()> {
    let elf = std::fs::read(elf_path).with_context(|| format!("cannot read ELF {}", elf_path.display()))?;
    let nro = elf_to_nro(&elf).with_context(|| format!("failed to convert {}", elf_path.display()))?;
    std::fs::write(nro_path, &nro).with_context(|| format!("cannot write NRO {}", nro_path.display()))?;
    Ok(())
}

/// Converts the bytes of a linked ELF into the bytes of an NRO.
pub fn elf_to_nro(elf: &[u8]) -> Result<Vec<u8>> {
    if elf.len() < ELF_HEADER_SIZE {
        bail!("input is too small to be an ELF");
    }
    if &elf[0..4] != b"\x7fELF" {
        bail!("not an ELF file (bad magic)");
    }
    if elf[4] != 1 {
        bail!("expected a 32-bit ELF (EI_CLASS != ELFCLASS32)");
    }
    if elf[5] != 1 {
        bail!("expected a little-endian ELF (EI_DATA != ELFDATA2LSB)");
    }

    let e_machine = read_u16(elf, 18)?;
    if e_machine != EM_ARM {
        bail!("expected an ARM ELF (e_machine = {e_machine}, want {EM_ARM})");
    }

    // Program headers: collect the loadable segments in order.
    // Assume a fixed 32-byte Elf32_Phdr and ignore e_phentsize.
    let e_phoff = read_u32(elf, 28)? as usize;
    let e_phnum = read_u16(elf, 44)? as usize;
    let ph_end = e_phoff
        .checked_add(e_phnum.checked_mul(PHDR_SIZE).context("phdr table size overflow")?)
        .context("phdr table offset overflow")?;
    if ph_end > elf.len() {
        bail!("invalid ELF: program headers extend past end of file");
    }

    let mut loads = Vec::new();
    for k in 0..e_phnum {
        let base = e_phoff + k * PHDR_SIZE;
        if read_u32(elf, base)? == PT_LOAD {
            loads.push(LoadSegment {
                offset: read_u32(elf, base + 4)?,
                vaddr: read_u32(elf, base + 8)?,
                filesz: read_u32(elf, base + 16)?,
                memsz: read_u32(elf, base + 20)?,
            });
        }
    }
    if loads.len() < 3 {
        bail!("invalid ELF: expected at least 3 loadable segments, found {}", loads.len());
    }

    // Segment table (file offset == virtual address, page-aligned size) and the
    // running total that becomes the NRO size.
    let mut seg_off = [0u32; 3];
    let mut seg_size = [0u32; 3];
    let mut total: u32 = 0;
    for i in 0..3 {
        let size = align_up(loads[i].filesz, PAGE)?;
        seg_off[i] = loads[i].vaddr;
        seg_size[i] = size;
        total = align_up(total.checked_add(size).context("NRO size overflow")?, PAGE)?;
    }

    // .bss is the tail of the data segment (segment index 2) that has no file
    // backing: memsz beyond the page-aligned filesz.
    let data = &loads[2];
    let data_filesz_aligned = align_up(data.filesz, PAGE)?;
    let bss_size = if data.memsz > data_filesz_aligned {
        align_up(data.memsz - data_filesz_aligned, PAGE)?
    } else {
        0
    };

    // Optional GNU build-id, copied from the first matching SHT_NOTE section.
    let build_id = read_build_id(elf)?;

    // Assemble the flat segment image, then stamp the header over it.
    let mut image = vec![0u8; total as usize];
    for i in 0..3 {
        let dst = seg_off[i] as usize;
        let src = loads[i].offset as usize;
        let n = loads[i].filesz as usize;
        let src_slice = elf
            .get(src..src.checked_add(n).context("segment file range overflow")?)
            .context("invalid ELF: segment data extends past end of file")?;
        image
            .get_mut(dst..dst.checked_add(n).context("segment image range overflow")?)
            .context("invalid ELF: segment does not fit within the NRO image (unexpected segment layout)")?
            .copy_from_slice(src_slice);
    }

    let header = build_header(&seg_off, &seg_size, total, bss_size, &build_id);
    image
        .get_mut(NRO_START_SIZE..NRO_START_SIZE + NRO_HEADER_SIZE)
        .context("NRO image is too small to hold its header")?
        .copy_from_slice(&header);

    Ok(image)
}

/// Builds the 0x70-byte NRO0 header.
fn build_header(
    seg_off: &[u32; 3],
    seg_size: &[u32; 3],
    total: u32,
    bss_size: u32,
    build_id: &[u8; 0x20],
) -> [u8; NRO_HEADER_SIZE] {
    let mut hdr = [0u8; NRO_HEADER_SIZE];
    hdr[0..4].copy_from_slice(b"NRO0");
    // 0x04 version, 0x0C flags: both 0 (we never set the AlignedHeader flag).
    write_u32(&mut hdr, 0x08, total); // size
    for i in 0..3 {
        write_u32(&mut hdr, 0x10 + i * 8, seg_off[i]);
        write_u32(&mut hdr, 0x14 + i * 8, seg_size[i]);
    }
    write_u32(&mut hdr, 0x28, bss_size);
    // 0x2C Unk3: 0
    hdr[0x30..0x50].copy_from_slice(build_id);
    // 0x50..0x70 padding: 0
    hdr
}

/// Scans the section headers for a GNU build-id note, returning it padded to
/// 0x20 bytes (zeroed if absent).
fn read_build_id(elf: &[u8]) -> Result<[u8; 0x20]> {
    let mut build_id = [0u8; 0x20];

    let e_shoff = read_u32(elf, 32)? as usize;
    let e_shnum = read_u16(elf, 48)? as usize;
    let mut e_shentsize = read_u16(elf, 46)? as usize;
    if e_shentsize == 0 {
        e_shentsize = 40;
    }

    for k in 0..e_shnum {
        let sh = e_shoff + k * e_shentsize;
        if read_u32(elf, sh + 4)? != SHT_NOTE {
            continue;
        }
        let note_off = read_u32(elf, sh + 16)? as usize;
        let n_namesz = read_u32(elf, note_off)? as usize;
        let n_descsz = read_u32(elf, note_off + 4)? as usize;
        let n_type = read_u32(elf, note_off + 8)?;

        let name_off = note_off + 12;
        let desc_off = name_off + n_namesz;
        if n_type == NT_GNU_BUILD_ID && n_namesz == 4 && elf.get(name_off..name_off + 4) == Some(&b"GNU\0"[..]) {
            let len = n_descsz.min(0x20);
            let desc = elf
                .get(desc_off..desc_off + len)
                .context("invalid ELF: build-id note extends past end of file")?;
            build_id[..len].copy_from_slice(desc);
        }
    }

    Ok(build_id)
}

/// Rounds v up to a multiple of align (a power of two), erroring on
/// overflow.
fn align_up(v: u32, align: u32) -> Result<u32> {
    v.checked_add(align - 1)
        .map(|x| x & !(align - 1))
        .context("alignment overflow")
}

fn read_u16(buf: &[u8], off: usize) -> Result<u16> {
    let b = buf.get(off..off + 2).context("unexpected end of ELF while reading u16")?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

fn read_u32(buf: &[u8], off: usize) -> Result<u32> {
    let b = buf.get(off..off + 4).context("unexpected end of ELF while reading u32")?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn write_u32(buf: &mut [u8], off: usize, val: u32) {
    buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align_up_rounds_to_page() {
        assert_eq!(align_up(0, PAGE).unwrap(), 0);
        assert_eq!(align_up(1, PAGE).unwrap(), 0x1000);
        assert_eq!(align_up(0x1000, PAGE).unwrap(), 0x1000);
        assert_eq!(align_up(0x1001, PAGE).unwrap(), 0x2000);
    }

    /// Builds a minimal but valid ELF32 (ARM, little-endian) with three
    /// PT_LOAD segments laid out like the forge linker script.
    fn synthetic_elf() -> Vec<u8> {
        let phoff = ELF_HEADER_SIZE;
        // Segment payloads live well past the program headers.
        let (text_off, text_filesz) = (0x200usize, 0x40u32);
        let (rodata_off, rodata_filesz) = (0x300usize, 0x20u32);
        let (data_off, data_filesz, data_memsz) = (0x400usize, 0x10u32, 0x1010u32);

        let mut elf = vec![0u8; 0x420];

        // ELF header
        elf[0..4].copy_from_slice(b"\x7fELF");
        elf[4] = 1; // ELFCLASS32
        elf[5] = 1; // ELFDATA2LSB
        write_u16(&mut elf, 18, EM_ARM);
        write_u32(&mut elf, 28, phoff as u32); // e_phoff
        write_u16(&mut elf, 42, PHDR_SIZE as u16); // e_phentsize
        write_u16(&mut elf, 44, 3); // e_phnum
        // No section headers (e_shnum = 0) -> no build-id.

        let segs = [
            (text_off as u32, 0x0000u32, text_filesz, text_filesz),
            (rodata_off as u32, 0x1000u32, rodata_filesz, rodata_filesz),
            (data_off as u32, 0x2000u32, data_filesz, data_memsz),
        ];
        for (i, (off, vaddr, filesz, memsz)) in segs.iter().enumerate() {
            let p = phoff + i * PHDR_SIZE;
            write_u32(&mut elf, p, PT_LOAD);
            write_u32(&mut elf, p + 4, *off);
            write_u32(&mut elf, p + 8, *vaddr);
            write_u32(&mut elf, p + 16, *filesz);
            write_u32(&mut elf, p + 20, *memsz);
        }

        // Mark each payload so we can confirm it landed at the right vaddr.
        elf[text_off] = 0xAA;
        elf[rodata_off] = 0xBB;
        elf[data_off] = 0xCC;
        elf
    }

    fn write_u16(buf: &mut [u8], off: usize, val: u16) {
        buf[off..off + 2].copy_from_slice(&val.to_le_bytes());
    }

    #[test]
    fn converts_three_segment_elf() {
        let elf = synthetic_elf();
        let nro = elf_to_nro(&elf).expect("conversion should succeed");

        // Three page-aligned segments back to back.
        assert_eq!(nro.len(), 0x3000);

        // The header is written at file offset 0x10, so every header field
        // below is at 0x10 + its in-header offset.
        assert_eq!(&nro[0x10..0x14], b"NRO0"); // magic   (hdr 0x00)
        assert_eq!(read_u32(&nro, 0x18).unwrap(), 0x3000); // size    (hdr 0x08)

        // Segment table: (file_off, size) for text / rodata / data (hdr 0x10..).
        assert_eq!(read_u32(&nro, 0x20).unwrap(), 0x0000); // text   file_off
        assert_eq!(read_u32(&nro, 0x24).unwrap(), 0x1000); // text   size
        assert_eq!(read_u32(&nro, 0x28).unwrap(), 0x1000); // rodata file_off
        assert_eq!(read_u32(&nro, 0x2C).unwrap(), 0x1000); // rodata size
        assert_eq!(read_u32(&nro, 0x30).unwrap(), 0x2000); // data   file_off
        assert_eq!(read_u32(&nro, 0x34).unwrap(), 0x1000); // data   size

        // bss: memsz 0x1010 minus page-aligned filesz 0x1000 -> 0x10 -> page 0x1000.
        assert_eq!(read_u32(&nro, 0x38).unwrap(), 0x1000); // bssSize (hdr 0x28)

        // Payloads landed at their virtual addresses (text byte 0 survives, as
        // the header only overwrites 0x10..0x80).
        assert_eq!(nro[0x0000], 0xAA);
        assert_eq!(nro[0x1000], 0xBB);
        assert_eq!(nro[0x2000], 0xCC);
    }

    #[test]
    fn rejects_non_elf() {
        assert!(elf_to_nro(&[0u8; 64]).is_err());
    }
}
