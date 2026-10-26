//! File-backed memories: read-only ROMs (`*.rom.txt`) and writable RAMs
//! (`*.ram.txt`).
//!
//! Both live in the `memory/` directory next to `Config.toml` and share the
//! same file syntax:
//!
//! ```text
//! v0.4.1-beta hex words addressed
//! origin = 0
//! clk = 0
//! address coordinate = [
//! -8, 64, 0
//! ]
//! data coordinate = [
//! 2, 64, 0
//! ]
//! 00: 0f 3c 69
//! ```
//!
//! A RAM additionally declares `write enable`, `read enable`,
//! `byte write enable`, `write data coordinate` and `read data coordinate`,
//! and its `clk` is mandatory. RAM contents are written back to the same file
//! (sparse, header preserved, atomically replaced).
//!
//! See `BACKLOG.md` BUG-015 (ROM) and BUG-021 (RAM).

use mchprs_blocks::BlockPos;
use mchprs_redpiler::backend::IoElement;
use mchprs_redpiler::Compiler;
use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Directory holding memory definitions, relative to the server working
/// directory (the directory that holds `Config.toml`).
pub const MEMORY_DIR: &str = "memory";

const ROM_SUFFIX: &str = ".rom.txt";
const RAM_SUFFIX: &str = ".ram.txt";

/// Address values are assembled into a `usize`, so the address bus cannot be
/// wider than this. Not a business limit, just what one integer can hold.
const MAX_ADDRESS_BITS: usize = 63;

/// A RAM writes its contents back every this often (or every
/// [`FLUSH_WRITES`] writes, whichever comes first).
const FLUSH_INTERVAL: Duration = Duration::from_secs(2);
const FLUSH_WRITES: usize = 4096;
/// How many consecutive addresses share one written line.
const FLUSH_GROUP: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Rom,
    Ram,
}

/// The memory's port layout.
pub enum Ports {
    /// ROM: the server drives `data` from the stored words.
    Rom { data: Vec<BlockPos> },
    /// RAM: the server samples the read side and drives `read_data`.
    Ram {
        write_data: Vec<BlockPos>,
        read_data: Vec<BlockPos>,
        write_enable: BlockPos,
        read_enable: BlockPos,
        byte_enable: Vec<BlockPos>,
    },
}

/// A parsed memory definition.
pub struct Memory {
    /// File name, used in log and chat messages.
    pub name: String,
    pub kind: Kind,
    /// Address bus, most significant bit first.
    pub address: Vec<BlockPos>,
    /// Clock element. `None` only for ROMs (a RAM clock is mandatory).
    pub clock: Option<BlockPos>,
    /// Word width in bits.
    pub bits: usize,
    /// Contents. For a ROM this never changes after loading; for a RAM it is
    /// the live state that gets persisted.
    pub words: HashMap<usize, Vec<bool>>,
    pub ports: Ports,
}

/// A parsed memory plus its runtime state.
pub struct MemoryState {
    pub memory: Memory,
    /// Whether the memory passed validation against the current compile.
    pub active: bool,
    /// Previous clock level, for rising edge detection.
    prev_clk: bool,
    /// Previous assembled address, for the ROM "no clock" mode.
    prev_address: Option<usize>,
    /// Where the definition was loaded from; RAM contents are saved here.
    path: PathBuf,
    /// Everything before the first data line, verbatim (comments included).
    header: String,
    dirty: bool,
    writes: usize,
    last_flush: Instant,
}

impl MemoryState {
    fn new(memory: Memory, path: PathBuf, header: String) -> MemoryState {
        MemoryState {
            memory,
            active: false,
            prev_clk: false,
            prev_address: None,
            path,
            header,
            dirty: false,
            writes: 0,
            last_flush: Instant::now(),
        }
    }

    /// Runs one sample pass when the memory is active.
    pub fn sample(&mut self, redpiler: &mut Compiler) {
        if !self.active {
            return;
        }
        if matches!(self.memory.kind, Kind::Rom) {
            self.run_rom(redpiler);
        } else {
            self.run_ram(redpiler);
        }
    }

    /// Drops the edge detectors so a fresh compile does not fire immediately.
    pub fn reset_edges(&mut self) {
        self.prev_clk = false;
        self.prev_address = None;
    }

    /// Whether a RAM has unsaved contents and enough time/writes have passed.
    pub fn flush_due(&self) -> bool {
        self.dirty
            && (self.writes >= FLUSH_WRITES || self.last_flush.elapsed() >= FLUSH_INTERVAL)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Writes the RAM contents back to its file if anything changed. Returns an
    /// error string on I/O failure (the memory stays dirty).
    pub fn save(&mut self) -> Result<(), String> {
        if !self.dirty {
            return Ok(());
        }
        if !matches!(self.memory.kind, Kind::Ram) {
            self.dirty = false;
            return Ok(());
        }
        let text = self.render();
        let parent = self.path.parent().unwrap_or_else(|| Path::new("."));
        let file_name = self
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "memory.ram.txt".to_string());
        let tmp = parent.join(format!(".{}.tmp", file_name));
        if let Err(err) = fs::write(&tmp, text) {
            return Err(format!("could not write {}: {}", tmp.display(), err));
        }
        if let Err(err) = fs::rename(&tmp, &self.path) {
            let _ = fs::remove_file(&tmp);
            return Err(format!("could not replace {}: {}", self.path.display(), err));
        }
        self.dirty = false;
        self.writes = 0;
        self.last_flush = Instant::now();
        Ok(())
    }

    /// Renders the file: the original header (first line rewritten to the
    /// running kernel version) followed by the sparse data section.
    fn render(&self) -> String {
        let mut text = String::new();
        text.push_str(&format!("v{} hex words addressed\n", env!("CARGO_PKG_VERSION")));
        let mut header_lines = self.header.lines();
        header_lines.next();
        for line in header_lines {
            text.push_str(line);
            text.push('\n');
        }

        let mut addresses: Vec<usize> = self.memory.words.keys().copied().collect();
        addresses.sort_unstable();
        let mut index = 0;
        while index < addresses.len() {
            let start = addresses[index];
            let mut end = index;
            while end + 1 < addresses.len()
                && addresses[end + 1] == addresses[end] + 1
                && end + 1 - index < FLUSH_GROUP
            {
                end += 1;
            }
            text.push_str(&format!("{:x}:", start));
            for address in &addresses[index..=end] {
                text.push(' ');
                text.push_str(&pack_word_hex(&self.memory.words[address]));
            }
            text.push('\n');
            index = end + 1;
        }
        text
    }

    /// ROM: drive the data bus from the stored word, either on the clock's
    /// rising edge or (no clock) whenever the address changes.
    fn run_rom(&mut self, redpiler: &mut Compiler) {
        let Ports::Rom { data } = &self.memory.ports else {
            return;
        };
        let Some(address) = read_address(redpiler, &self.memory.address) else {
            return;
        };
        let fire = match self.memory.clock {
            Some(clk) => {
                let Some((_, powered, _)) = redpiler.query_io(clk) else {
                    return;
                };
                let edge = !self.prev_clk && powered;
                self.prev_clk = powered;
                edge
            }
            None => self.prev_address != Some(address),
        };
        self.prev_address = Some(address);
        if !fire {
            return;
        }
        let width = self.memory.bits;
        let word = self
            .memory
            .words
            .get(&address)
            .cloned()
            .unwrap_or_else(|| vec![false; width]);
        for (pos, bit) in data.iter().zip(word.iter()) {
            let _ = redpiler.try_set_powered(*pos, *bit);
        }
    }

    /// RAM: on the clock's rising edge, sample the read side, drive the read
    /// data bus with the OLD word, then apply the (byte-masked) write.
    fn run_ram(&mut self, redpiler: &mut Compiler) {
        let Some(clk) = self.memory.clock else {
            return;
        };
        let Some((_, now, _)) = redpiler.query_io(clk) else {
            return;
        };
        let edge = !self.prev_clk && now;
        self.prev_clk = now;
        if !edge {
            return;
        }

        // Copy the buses out of `self` so the writes below do not conflict with
        // the borrow. This only runs on a clock edge.
        let (address_bus, write_bus, read_bus, we, re, byte_bus, bits) = {
            let Ports::Ram {
                write_data,
                read_data,
                write_enable,
                read_enable,
                byte_enable,
            } = &self.memory.ports
            else {
                return;
            };
            (
                self.memory.address.clone(),
                write_data.clone(),
                read_data.clone(),
                *write_enable,
                *read_enable,
                byte_enable.clone(),
                self.memory.bits,
            )
        };

        let Some(address) = read_address(redpiler, &address_bus) else {
            return;
        };
        let write_enabled = matches!(redpiler.query_io(we), Some((_, true, _)));
        let read_enabled = matches!(redpiler.query_io(re), Some((_, true, _)));

        // Read priority: the word is sampled before this edge's write applies.
        if read_enabled {
            let word = self
                .memory
                .words
                .get(&address)
                .cloned()
                .unwrap_or_else(|| vec![false; bits]);
            for (pos, bit) in read_bus.iter().zip(word.iter()) {
                let _ = redpiler.try_set_powered(*pos, *bit);
            }
        }

        if !write_enabled {
            return;
        }
        let Some(new_word) = read_bits(redpiler, &write_bus) else {
            return;
        };
        let enables: Vec<bool> = byte_bus
            .iter()
            .map(|pos| matches!(redpiler.query_io(*pos), Some((_, true, _))))
            .collect();
        if !enables.is_empty() && enables.len() != (bits + 7) / 8 {
            return;
        }
        let old_word = self
            .memory
            .words
            .get(&address)
            .cloned()
            .unwrap_or_else(|| vec![false; bits]);
        let merged = merge_word(&old_word, &new_word, &enables);
        if self.memory.words.get(&address) != Some(&merged) {
            self.memory.words.insert(address, merged);
            self.dirty = true;
            self.writes += 1;
        }
    }
}

/// Loads every `*.rom.txt` and `*.ram.txt` file in `dir`, skipping (and
/// logging) broken ones.
/// The result of loading the memory directory.
pub struct LoadReport {
    pub memories: Vec<MemoryState>,
    /// One human-readable message per file that could not be loaded.
    pub rejected: Vec<String>,
}

/// Loads every `*.rom.txt` and `*.ram.txt` file in `dir`, skipping broken ones
/// and reporting why (BACKLOG.md BUG-015 / BUG-021).
pub fn load_all(dir: &Path) -> LoadReport {
    let mut memories: Vec<MemoryState> = Vec::new();
    let mut rejected: Vec<String> = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            if err.kind() != std::io::ErrorKind::NotFound {
                warn!(
                    "Could not read the memory directory {}: {}. No memories loaded.",
                    dir.display(),
                    err
                );
            }
            return LoadReport {
                memories,
                rejected,
            };
        }
    };

    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            paths.push(path);
        }
    }
    paths.sort();

    for path in paths {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let kind = if name.ends_with(RAM_SUFFIX) {
            Kind::Ram
        } else if name.ends_with(ROM_SUFFIX) {
            Kind::Rom
        } else {
            continue;
        };
        match fs::read_to_string(&path) {
            Ok(text) => match parse(&name, &text, kind) {
                Ok((memory, header)) => {
                    memories.push(MemoryState::new(memory, path.clone(), header))
                }
                Err(err) => {
                    let message = format!("Memory {} rejected: {}", name, err);
                    warn!("{}", message);
                    rejected.push(message);
                }
            },
            Err(err) => {
                let message = format!("Memory {} could not be read: {}", name, err);
                warn!("{}", message);
                rejected.push(message);
            }
        }
    }

    // Two memories may share address and data lines, but a clock element may
    // only drive one memory: on a conflict both are rejected.
    let mut clock_users: HashMap<BlockPos, usize> = HashMap::new();
    for state in &memories {
        if let Some(clk) = state.memory.clock {
            *clock_users.entry(clk).or_insert(0) += 1;
        }
    }
    memories.retain(|state| {
        let Some(clk) = state.memory.clock else {
            return true;
        };
        if clock_users[&clk] > 1 {
            let message = format!(
                "Memory {} rejected: clock element ({},{},{}) is used by more than one memory",
                state.memory.name, clk.x, clk.y, clk.z
            );
            warn!("{}", message);
            rejected.push(message);
            false
        } else {
            true
        }
    });

    if !memories.is_empty() {
        let roms = memories
            .iter()
            .filter(|state| matches!(state.memory.kind, Kind::Rom))
            .count();
        info!(
            "Loaded {} memory definition(s) from {} ({} ROM, {} RAM)",
            memories.len(),
            dir.display(),
            roms,
            memories.len() - roms
        );
    }
    LoadReport {
        memories,
        rejected,
    }
}
/// How a memory failed validation, for the player-facing message.
pub enum Invalid {
    /// A block is there, but it is the wrong kind of element.
    WrongElement {
        pos: BlockPos,
        role: &'static str,
        wanted: &'static str,
        found: &'static str,
    },
    /// Nothing was compiled at that position.
    Missing {
        pos: BlockPos,
        role: &'static str,
        wanted: &'static str,
    },
    /// A driven element touches an element the memory also reads.
    Adjacent { first: BlockPos, second: BlockPos },
    /// The same block is used by both the write and the read data bus.
    Overlap { first: BlockPos, second: BlockPos },
    /// An output element has incoming links, so the memory's write would fight
    /// with the circuit.
    DrivenOutput { pos: BlockPos },
    /// The address bus is wider than the lookup can handle.
    AddressTooWide,
}

const OUTPUT_TYPES: &str = "a redstone lamp, trapdoor or note block";
const INPUT_TYPES: &str = "a lever or a pressure plate";

impl Memory {
    pub fn validate(&self, redpiler: &Compiler) -> Result<(), Invalid> {
        if self.address.len() > MAX_ADDRESS_BITS {
            return Err(Invalid::AddressTooWide);
        }
        for pos in &self.address {
            expect_output(redpiler, *pos, "an address line")?;
        }
        if let Some(clk) = self.clock {
            expect_output(redpiler, clk, "the clock")?;
        }
        match &self.ports {
            Ports::Rom { data } => {
                for pos in data {
                    expect_input(redpiler, *pos, "a data line")?;
                }
                for data_pos in data {
                    for read_pos in self.address.iter().chain(self.clock.iter()) {
                        if touches(*data_pos, *read_pos) {
                            return Err(Invalid::Adjacent {
                                first: *data_pos,
                                second: *read_pos,
                            });
                        }
                    }
                }
            }
            Ports::Ram {
                write_data,
                read_data,
                write_enable,
                read_enable,
                byte_enable,
            } => {
                expect_output(redpiler, *write_enable, "the write enable")?;
                expect_output(redpiler, *read_enable, "the read enable")?;
                for pos in write_data {
                    expect_output(redpiler, *pos, "a write data line")?;
                }
                for pos in byte_enable {
                    expect_output(redpiler, *pos, "a byte write enable")?;
                }
                for pos in read_data {
                    expect_input(redpiler, *pos, "a read data line")?;
                    if redpiler.has_inputs(*pos).unwrap_or(false) {
                        return Err(Invalid::DrivenOutput { pos: *pos });
                    }
                }
                for write_pos in write_data {
                    for read_pos in read_data {
                        if write_pos == read_pos {
                            return Err(Invalid::Overlap {
                                first: *write_pos,
                                second: *read_pos,
                            });
                        }
                    }
                }
                let read_side: Vec<BlockPos> = self
                    .address
                    .iter()
                    .copied()
                    .chain(self.clock.iter().copied())
                    .chain(std::iter::once(*write_enable))
                    .chain(std::iter::once(*read_enable))
                    .chain(byte_enable.iter().copied())
                    .collect();
                for data_pos in read_data {
                    for other in &read_side {
                        if touches(*data_pos, *other) {
                            return Err(Invalid::Adjacent {
                                first: *data_pos,
                                second: *other,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// Formats a memory validation failure for chat and the log.
pub fn describe_invalid(memory: &Memory, invalid: &Invalid) -> String {
    match invalid {
        Invalid::WrongElement {
            pos,
            role,
            wanted,
            found,
        } => format!(
            "Memory {} disabled: ({},{},{}) is {}, but {} must be {}",
            memory.name, pos.x, pos.y, pos.z, found, role, wanted
        ),
        Invalid::Missing { pos, role, wanted } => format!(
            "Memory {} disabled: ({},{},{}) is not part of the compiled circuit, but {} must be {}",
            memory.name, pos.x, pos.y, pos.z, role, wanted
        ),
        Invalid::Adjacent { first, second } => format!(
            "Memory {} disabled: ({},{},{}) touches ({},{},{}); writing it would drive the signal it reads",
            memory.name, first.x, first.y, first.z, second.x, second.y, second.z
        ),
        Invalid::Overlap { first, second } => format!(
            "Memory {} disabled: ({},{},{}) is on the write data bus and ({},{},{}) on the read data bus",
            memory.name, first.x, first.y, first.z, second.x, second.y, second.z
        ),
        Invalid::DrivenOutput { pos } => format!(
            "Memory {} disabled: ({},{},{}) is driven by the circuit, so the memory cannot write it",
            memory.name, pos.x, pos.y, pos.z
        ),
        Invalid::AddressTooWide => format!(
            "Memory {} disabled: address bus is wider than {} bits",
            memory.name, MAX_ADDRESS_BITS
        ),
    }
}

/// A human-readable name for an element, used in validation messages.
fn element_name(element: IoElement) -> &'static str {
    match element {
        IoElement::Button => "a button",
        IoElement::Lever => "a lever",
        IoElement::PressurePlate => "a pressure plate",
        IoElement::Lamp => "a redstone lamp",
        IoElement::Trapdoor => "a trapdoor",
        IoElement::NoteBlock => "a note block",
    }
}

fn expect_output(redpiler: &Compiler, pos: BlockPos, role: &'static str) -> Result<(), Invalid> {
    match redpiler.query_io(pos) {
        Some((
            IoElement::Lamp | IoElement::Trapdoor | IoElement::NoteBlock,
            _,
            _,
        )) => Ok(()),
        Some((element, _, _)) => Err(Invalid::WrongElement {
            pos,
            role,
            wanted: OUTPUT_TYPES,
            found: element_name(element),
        }),
        None => Err(Invalid::Missing {
            pos,
            role,
            wanted: OUTPUT_TYPES,
        }),
    }
}

fn expect_input(redpiler: &Compiler, pos: BlockPos, role: &'static str) -> Result<(), Invalid> {
    match redpiler.query_io(pos) {
        Some((IoElement::Lever | IoElement::PressurePlate, _, _)) => Ok(()),
        Some((element, _, _)) => Err(Invalid::WrongElement {
            pos,
            role,
            wanted: INPUT_TYPES,
            found: element_name(element),
        }),
        None => Err(Invalid::Missing {
            pos,
            role,
            wanted: INPUT_TYPES,
        }),
    }
}
fn touches(a: BlockPos, b: BlockPos) -> bool {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs() == 1
}

/// Assembles the address bus value. `None` when any element is missing from
/// the compiled graph.
fn read_address(redpiler: &Compiler, points: &[BlockPos]) -> Option<usize> {
    let mut value = 0usize;
    for pos in points {
        let (_, powered, _) = redpiler.query_io(*pos)?;
        value = (value << 1) | usize::from(powered);
    }
    Some(value)
}

/// Reads every element of a bus as 0/1.
fn read_bits(redpiler: &Compiler, points: &[BlockPos]) -> Option<Vec<bool>> {
    let mut bits = Vec::with_capacity(points.len());
    for pos in points {
        let (_, powered, _) = redpiler.query_io(*pos)?;
        bits.push(powered);
    }
    Some(bits)
}

/// Applies a byte-masked write: enabled groups take the new bits, disabled
/// groups keep the old ones. Groups are cut from the most significant end,
/// eight bits at a time; the most significant group may be shorter, and the
/// LAST group is always the low 8 bits (BACKLOG.md BUG-021).
fn merge_word(old: &[bool], new: &[bool], enables: &[bool]) -> Vec<bool> {
    if enables.is_empty() {
        return new.to_vec();
    }
    let width = old.len();
    let mut merged = old.to_vec();
    let mut start = 0usize;
    let mut group = 0usize;
    let leftover = width % 8;
    if leftover != 0 {
        if enables.first().copied().unwrap_or(true) {
            merged[..leftover].copy_from_slice(&new[..leftover]);
        }
        start = leftover;
        group = 1;
    }
    while start < width && group < enables.len() {
        if enables[group] {
            let end = (start + 8).min(width);
            merged[start..end].copy_from_slice(&new[start..end]);
        }
        start += 8;
        group += 1;
    }
    merged
}

/// Packs one word into hex the same way the protocol does: the bit vector as a
/// single unsigned integer, zero-padded at the front (BACKLOG.md BUG-016).
fn pack_word_hex(bits: &[bool]) -> String {
    let mut hex = String::with_capacity(bits.len() / 4 + 1);
    let leftover = bits.len() % 4;
    let mut index = 0;
    if leftover != 0 {
        let mut value = 0u8;
        for bit in &bits[..leftover] {
            value = (value << 1) | u8::from(*bit);
        }
        hex.push(char::from_digit(u32::from(value), 16).unwrap());
        index = leftover;
    }
    while index < bits.len() {
        let mut value = 0u8;
        for bit in &bits[index..index + 4] {
            value = (value << 1) | u8::from(*bit);
        }
        hex.push(char::from_digit(u32::from(value), 16).unwrap());
        index += 4;
    }
    hex
}

// === PART 2: PARSING ===
/// Parses one memory file. Returns the memory definition and the verbatim
/// header (every original line before the first data line), which a RAM
/// write-back preserves.
/// Prefixes a 0-based line index with its 1-based number.
fn at_line(index: usize, message: impl Into<String>) -> String {
    format!("line {}: {}", index + 1, message.into())
}

/// Parses one memory file. Returns the memory definition and the verbatim
/// header (every original line before the first data line), which a RAM
/// write-back preserves.
pub fn parse(name: &str, text: &str, kind: Kind) -> Result<(Memory, String), String> {
    let stripped =
        strip_comments(text).map_err(|(line, message)| format!("line {}: {}", line, message))?;
    let original: Vec<&str> = text.lines().collect();
    let lines: Vec<&str> = stripped.lines().collect();
    let count = lines.len().min(original.len());

    let mut idx = 0usize;
    while idx < count && lines[idx].trim().is_empty() {
        idx += 1;
    }
    if idx >= count {
        return Err("empty file".into());
    }

    let version_token = format!("v{}", env!("CARGO_PKG_VERSION"));
    let header = lines[idx].trim();
    let mut header_parts = header.split_whitespace();
    let header_ok = header_parts.next() == Some(version_token.as_str())
        && header_parts.next() == Some("hex")
        && header_parts.next() == Some("words")
        && header_parts.next() == Some("addressed")
        && header_parts.next().is_none();
    if !header_ok {
        return Err(at_line(
            idx,
            format!(
                "first line must be `{} hex words addressed`, found `{}`",
                version_token, header
            ),
        ));
    }
    idx += 1;

    let mut origin: Option<BlockPos> = None;
    let mut clock: Option<BlockPos> = None;
    let mut clock_seen = false;
    let mut address: Option<Vec<BlockPos>> = None;
    let mut data: Option<Vec<BlockPos>> = None;
    let mut write_data: Option<Vec<BlockPos>> = None;
    let mut read_data: Option<Vec<BlockPos>> = None;
    let mut write_enable: Option<BlockPos> = None;
    let mut read_enable: Option<BlockPos> = None;
    let mut byte_enable: Option<Vec<BlockPos>> = None;
    let mut data_start = count;

    while idx < count {
        let trimmed = lines[idx].trim();
        if trimmed.is_empty() {
            idx += 1;
            continue;
        }
        if is_data_line(trimmed) {
            data_start = idx;
            break;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            return Err(at_line(idx, format!("unexpected line `{}`", trimmed)));
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "clk" => {
                if clock_seen {
                    return Err(at_line(idx, "`clk` is defined twice"));
                }
                clock_seen = true;
                clock = match parse_single_or_triple(value)
                    .map_err(|err| at_line(idx, format!("`clk`: {}", err)))?
                {
                    Coordinate::Absent => None,
                    Coordinate::At(pos) => Some(pos),
                };
            }
            "origin" => {
                if origin.is_some() {
                    return Err(at_line(idx, "`origin` is defined twice"));
                }
                origin = Some(
                    match parse_single_or_triple(value)
                        .map_err(|err| at_line(idx, format!("`origin`: {}", err)))?
                    {
                        Coordinate::Absent => BlockPos::new(0, 0, 0),
                        Coordinate::At(pos) => pos,
                    },
                );
            }
            "address coordinate" => {
                if address.is_some() {
                    return Err(at_line(idx, "`address coordinate` is defined twice"));
                }
                address = Some(parse_list(
                    &lines,
                    &mut idx,
                    value,
                    count,
                    "address coordinate",
                )?);
            }
            "data coordinate" => {
                if data.is_some() {
                    return Err(at_line(idx, "`data coordinate` is defined twice"));
                }
                data = Some(parse_list(&lines, &mut idx, value, count, "data coordinate")?);
            }
            "write data coordinate" => {
                if write_data.is_some() {
                    return Err(at_line(idx, "`write data coordinate` is defined twice"));
                }
                write_data = Some(parse_list(
                    &lines,
                    &mut idx,
                    value,
                    count,
                    "write data coordinate",
                )?);
            }
            "read data coordinate" => {
                if read_data.is_some() {
                    return Err(at_line(idx, "`read data coordinate` is defined twice"));
                }
                read_data = Some(parse_list(
                    &lines,
                    &mut idx,
                    value,
                    count,
                    "read data coordinate",
                )?);
            }
            "write enable" => {
                if write_enable.is_some() {
                    return Err(at_line(idx, "`write enable` is defined twice"));
                }
                write_enable = Some(
                    match parse_single_or_triple(value)
                        .map_err(|err| at_line(idx, format!("`write enable`: {}", err)))?
                    {
                        Coordinate::At(pos) => pos,
                        Coordinate::Absent => {
                            return Err(at_line(
                                idx,
                                "`write enable` must be a coordinate, not `0`",
                            ))
                        }
                    },
                );
            }
            "read enable" => {
                if read_enable.is_some() {
                    return Err(at_line(idx, "`read enable` is defined twice"));
                }
                read_enable = Some(
                    match parse_single_or_triple(value)
                        .map_err(|err| at_line(idx, format!("`read enable`: {}", err)))?
                    {
                        Coordinate::At(pos) => pos,
                        Coordinate::Absent => {
                            return Err(at_line(
                                idx,
                                "`read enable` must be a coordinate, not `0`",
                            ))
                        }
                    },
                );
            }
            "byte write enable" => {
                if byte_enable.is_some() {
                    return Err(at_line(idx, "`byte write enable` is defined twice"));
                }
                byte_enable = Some(parse_list(
                    &lines,
                    &mut idx,
                    value,
                    count,
                    "byte write enable",
                )?);
            }
            _ => return Err(at_line(idx, format!("unknown key `{}`", key))),
        }
        idx += 1;
    }

    if !clock_seen {
        return Err(match kind {
            Kind::Ram => "a RAM requires a clock element: write `clk = x, y, z`".into(),
            Kind::Rom => {
                "`clk` is required (use `clk = 0` for a ROM without a clock)".into()
            }
        });
    }
    let origin = origin.ok_or("`origin` is required (use `origin = 0` for world coordinates)")?;
    let address = address.ok_or("`address coordinate` is required")?;
    if address.len() > MAX_ADDRESS_BITS {
        return Err(format!(
            "address bus is {} bits wide, the maximum is {}",
            address.len(),
            MAX_ADDRESS_BITS
        ));
    }
    if kind == Kind::Ram && clock.is_none() {
        return Err("a RAM requires a clock element: `clk = 0` is not allowed".into());
    }

    let (bits, ports) = match kind {
        Kind::Rom => {
            if write_data.is_some() || read_data.is_some() {
                return Err("a ROM does not take `write`/`read data coordinate`".into());
            }
            let data = data.ok_or("`data coordinate` is required")?;
            if data.is_empty() {
                return Err("`data coordinate` must not be empty".into());
            }
            let bits = data.len();
            (bits, Ports::Rom { data: offset_all(data, origin) })
        }
        Kind::Ram => {
            if data.is_some() {
                return Err(
                    "a RAM does not take `data coordinate`; use `write data coordinate` and `read data coordinate`"
                        .into(),
                );
            }
            let write_data = write_data.ok_or("`write data coordinate` is required")?;
            let read_data = read_data.ok_or("`read data coordinate` is required")?;
            if write_data.is_empty() {
                return Err("`write data coordinate` must not be empty".into());
            }
            if write_data.len() != read_data.len() {
                return Err(format!(
                    "`write data coordinate` has {} entries but `read data coordinate` has {}",
                    write_data.len(),
                    read_data.len()
                ));
            }
            let bits = write_data.len();
            let write_enable = write_enable.ok_or("`write enable` is required")?;
            let read_enable = read_enable.ok_or("`read enable` is required")?;
            let byte_enable = byte_enable.unwrap_or_default();
            let expected = (bits + 7) / 8;
            if !byte_enable.is_empty() && byte_enable.len() != expected {
                return Err(format!(
                    "`byte write enable` has {} entries but {} data bits need {}",
                    byte_enable.len(),
                    bits,
                    expected
                ));
            }
            (
                bits,
                Ports::Ram {
                    write_data: offset_all(write_data, origin),
                    read_data: offset_all(read_data, origin),
                    write_enable: write_enable.add(origin),
                    read_enable: read_enable.add(origin),
                    byte_enable: offset_all(byte_enable, origin),
                },
            )
        }
    };

    let address = offset_all(address, origin);
    let clock = clock.map(|pos| pos.add(origin));
    let words = parse_words(&lines, data_start, bits)?;

    Ok((
        Memory {
            name: name.to_string(),
            kind,
            address,
            clock,
            bits,
            words,
            ports,
        },
        original[..data_start.min(original.len())].join("\n"),
    ))
}

fn offset_all(points: Vec<BlockPos>, origin: BlockPos) -> Vec<BlockPos> {
    points.into_iter().map(|pos| pos.add(origin)).collect()
}

/// The outcome of parsing a `clk` / `origin` value.
enum Coordinate {
    /// The single token `0`, meaning "not present".
    Absent,
    At(BlockPos),
}

fn parse_single_or_triple(value: &str) -> Result<Coordinate, String> {
    let parts: Vec<&str> = value.split(',').map(|part| part.trim()).collect();
    match parts.as_slice() {
        [single] => {
            let number: i32 = single
                .parse()
                .map_err(|_| format!("`{}` is not a number", single))?;
            if number == 0 {
                Ok(Coordinate::Absent)
            } else {
                Err(format!(
                    "`{}` is neither `0` nor a coordinate; write `0` for \"absent\" or `x, y, z`",
                    single
                ))
            }
        }
        [x, y, z] => Ok(Coordinate::At(BlockPos::new(
            parse_i32(x)?,
            parse_i32(y)?,
            parse_i32(z)?,
        ))),
        _ => Err(format!(
            "`{}` must hold either one number (`0`) or three (a coordinate)",
            value
        )),
    }
}

/// Parses a `[ ... ]` coordinate list. The opening bracket may sit on the same
/// line as the `=` or on a later one (BACKLOG.md BUG-015). `idx` is advanced to
/// the last line the list consumed.
fn parse_list(
    lines: &[&str],
    idx: &mut usize,
    value: &str,
    count: usize,
    key: &str,
) -> Result<Vec<BlockPos>, String> {
    let mut buffer = String::new();
    let mut opened = false;
    let mut closed = false;
    let start_index = *idx;

    feed_list(value, &mut buffer, &mut opened, &mut closed);
    while !opened && !closed {
        *idx += 1;
        if *idx >= count {
            return Err(format!("`{}`: expected `[` after the key", key));
        }
        feed_list(lines[*idx], &mut buffer, &mut opened, &mut closed);
    }
    while !closed {
        *idx += 1;
        if *idx >= count {
            return Err(at_line(
                *idx,
                format!("`{}`: unclosed `[` in a coordinate list", key),
            ));
        }
        feed_list(lines[*idx], &mut buffer, &mut opened, &mut closed);
    }

    let mut points = Vec::new();
    for (offset, line) in buffer.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let index = start_index + offset;
        let parts: Vec<&str> = line.split(',').map(|part| part.trim()).collect();
        let [x, y, z] = parts.as_slice() else {
            return Err(at_line(
                index,
                format!("`{}`: coordinate `{}` must be a triple `x, y, z`", key, line),
            ));
        };
        points.push(BlockPos::new(
            parse_i32(x).map_err(|err| at_line(index, format!("`{}`: {}", key, err)))?,
            parse_i32(y).map_err(|err| at_line(index, format!("`{}`: {}", key, err)))?,
            parse_i32(z).map_err(|err| at_line(index, format!("`{}`: {}", key, err)))?,
        ));
    }
    Ok(points)
}
fn feed_list(text: &str, buffer: &mut String, opened: &mut bool, closed: &mut bool) {
    if *closed {
        return;
    }
    let mut rest = text;
    if !*opened {
        match rest.find('[') {
            Some(index) => {
                *opened = true;
                rest = &rest[index + 1..];
            }
            None => return,
        }
    }
    match rest.find(']') {
        Some(index) => {
            buffer.push_str(&rest[..index]);
            *closed = true;
        }
        None => {
            buffer.push_str(rest);
            buffer.push('\n');
        }
    }
}

/// Parses the Logisim payload into words, keyed by address.
fn parse_words(
    lines: &[&str],
    start: usize,
    bits: usize,
) -> Result<HashMap<usize, Vec<bool>>, String> {
    let mut words: HashMap<usize, Vec<bool>> = HashMap::new();
    let mut next: usize = 0;
    for (offset, line) in lines.iter().skip(start).enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let index = start + offset;
        let mut tokens: Vec<&str> = Vec::new();
        if let Some((address_text, rest)) = line.split_once(':') {
            let address_text = address_text.trim();
            let parsed = parse_hex_u64(address_text)
                .map_err(|_| at_line(index, format!("invalid address `{}`", address_text)))?;
            next = usize::try_from(parsed)
                .map_err(|_| at_line(index, format!("address `{}` is too large", address_text)))?;
            tokens.extend(rest.split_whitespace());
        } else {
            tokens.extend(line.split_whitespace());
        }
        for token in tokens {
            let word = parse_word(token, bits).map_err(|err| at_line(index, err))?;
            words.insert(next, word);
            next = next
                .checked_add(1)
                .ok_or_else(|| at_line(index, "address overflow"))?;
        }
    }
    Ok(words)
}
fn parse_i32(text: &str) -> Result<i32, String> {
    text.parse()
        .map_err(|_| format!("`{}` is not a whole number", text))
}

fn parse_hex_u64(text: &str) -> Result<u64, ()> {
    let text = text.trim();
    let text = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .unwrap_or(text);
    if text.is_empty() {
        return Err(());
    }
    u64::from_str_radix(text, 16).map_err(|_| ())
}

/// Parses one Logisim word into `width` bits, most significant bit first. A
/// word wider than the bus keeps only its low bits; a narrower one is
/// zero-extended at the top.
fn parse_word(word: &str, width: usize) -> Result<Vec<bool>, String> {
    let text = word.trim();
    let text = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .unwrap_or(text);
    if text.is_empty() {
        return Err("empty word".into());
    }
    let mut bits = Vec::with_capacity(text.len() * 4);
    for ch in text.chars() {
        let digit = ch
            .to_digit(16)
            .ok_or_else(|| format!("`{}` is not a hexadecimal word", word))?;
        for shift in (0..4).rev() {
            bits.push(digit & (1 << shift) != 0);
        }
    }
    let mut out = vec![false; width];
    let take = bits.len().min(width);
    let offset = width - take;
    out[offset..].copy_from_slice(&bits[bits.len() - take..]);
    Ok(out)
}

/// Whether a line starts the Logisim payload (`1f: 00 0a ...`).
fn is_data_line(line: &str) -> bool {
    match line.split_once(':') {
        Some((address, _)) => {
            let address = address.trim();
            !address.is_empty() && parse_hex_u64(address).is_ok()
        }
        None => false,
    }
}

/// Removes `#`, `//` and `/* */` comments, preserving line structure. Returns
/// `(line number, message)` on an unterminated block comment.
fn strip_comments(text: &str) -> Result<String, (usize, String)> {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    let mut line = 1usize;

    while let Some((_, ch)) = chars.next() {
        match ch {
            '\n' => {
                line += 1;
                out.push('\n');
            }
            '#' => {
                while let Some((_, next)) = chars.peek() {
                    if *next == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '/' => match chars.peek() {
                Some((_, '/')) => {
                    chars.next();
                    while let Some((_, next)) = chars.peek() {
                        if *next == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
                Some((_, '*')) => {
                    chars.next();
                    let start_line = line;
                    loop {
                        let Some((_, next)) = chars.next() else {
                            return Err((start_line, "unterminated `/*` comment".into()));
                        };
                        if next == '\n' {
                            line += 1;
                            out.push('\n');
                        } else if next == '*' {
                            if let Some((_, '/')) = chars.peek() {
                                chars.next();
                                break;
                            }
                        }
                    }
                }
                _ => out.push(ch),
            },
            _ => out.push(ch),
        }
    }

    Ok(out)
}
