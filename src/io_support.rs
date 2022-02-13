pub(crate) struct file_of_text_char {
    file_state: FileState<text_char>,
    error_state: usize,
}

impl Default for file_of_text_char {
    fn default() -> Self {
        file_of_text_char {
            file_state: FileState::default(),
            error_state: usize::default(),
        }
    }
}

crate::impl_debug_with_literal!(file_of_text_char, "file_of_text_char");

impl PascalFile for file_of_text_char {
    type Unit = text_char;

    fn is_text_file() -> bool {
        true
    }

    fn is_eoln_unit(unit: &Self::Unit) -> bool {
        unit.0 == b'\n' as char_repr
    }

    fn eoln_unit() -> Self::Unit {
        text_char::new(b'\n' as _)
    }

    fn convert_line_string_crlf_to_lf(input: &mut String) {
        if input.ends_with("\r\n") {
            input.pop();
            input.pop();
            input.push('\n');
        }
    }

    fn convert_line_string_to_units(input: &str, units: &mut Vec<Self::Unit>) {
        #[cfg(not(feature = "unicode_support"))]
        {
            for byte in input.bytes() {
                units.push(text_char(byte))
            }
        }
        #[cfg(feature = "unicode_support")]
        {
            use unicode_segmentation::UnicodeSegmentation;
            for grapheme in input.graphemes(true) {
                if grapheme.as_bytes().len() == 1 {
                    units.push(text_char::new(grapheme.as_bytes()[0] as _))
                } else {
                    let (byte_offset, ch) = grapheme.char_indices().rev().next().unwrap();
                    if byte_offset == 0 {
                        units.push(text_char::new(ch as _))
                    } else {
                        todo!("not yet implemented in {}", file!());
                    }
                }
            }
        }
    }

    fn convert_blob_to_unit(_: &[u8]) -> Self::Unit {
        unreachable!();
    }

    fn convert_unit_to_blob(_: Self::Unit, _: &mut dyn for<'a> FnMut(&'a [u8])) {
        unreachable!();
    }

    fn file_state(&self) -> &FileState<text_char> {
        &self.file_state
    }

    fn file_state_mut(&mut self) -> &mut FileState<text_char> {
        &mut self.file_state
    }

    fn error_state(&self) -> usize {
        self.error_state
    }

    fn set_error_state(&mut self, error_state: usize) {
        self.error_state = error_state;
    }

    fn open_text_file_for_read(path: &str) -> Result<(Box<dyn pascal_io::ReadLine>, bool), usize> {
        let path = path.trim_end_matches(' ');
        if path == crate::section_0011::pool_name.trim_end_matches(' ') {
            Ok((Box::new(crate::string_pool::pool_file()), false))
        } else {
            IO_HANDLER_OPEN_TEXT_FILE_FOR_READ.with(|c| c.borrow()(path))
        }
    }

    fn open_binary_file_for_read(path: &str) -> Result<Box<dyn Read>, usize> {
        let path = path.trim_end_matches(' ');
        IO_HANDLER_OPEN_BINARY_FILE_FOR_READ.with(|c| c.borrow()(path))
    }

    fn open_file_for_write(path: &str) -> Result<Box<dyn Write>, usize> {
        let path = path.trim_end_matches(' ');
        IO_HANDLER_OPEN_FILE_FOR_WRITE.with(|c| c.borrow()(path))
    }
}

pub(crate) type packed_file_of_text_char = file_of_text_char;

pub(crate) struct file_of<T> {
    file_state: FileState<T>,
    error_state: usize,
}

crate::impl_debug_with_literal!(file_of[T], "file_of<T>");

impl<T> Default for file_of<T> {
    fn default() -> Self {
        file_of {
            file_state: FileState::default(),
            error_state: usize::default(),
        }
    }
}

impl<T: FromBlob + ToBlob> PascalFile for file_of<T> {
    type Unit = T;

    fn is_text_file() -> bool {
        false
    }

    fn is_eoln_unit(_: &T) -> bool {
        unreachable!()
    }

    fn eoln_unit() -> Self::Unit {
        unreachable!()
    }

    fn convert_line_string_crlf_to_lf(_: &mut String) {
        unreachable!()
    }

    fn convert_line_string_to_units(_: &str, _: &mut Vec<Self::Unit>) {
        unreachable!()
    }

    fn convert_blob_to_unit(input: &[u8]) -> Self::Unit {
        T::from_blob(input)
    }

    fn convert_unit_to_blob(v: Self::Unit, f: &mut dyn for<'a> FnMut(&'a [u8])) {
        use core::borrow::Borrow;
        let blob = v.to_blob();
        f(blob.borrow());
    }

    fn file_state(&self) -> &FileState<T> {
        &self.file_state
    }

    fn file_state_mut(&mut self) -> &mut FileState<T> {
        &mut self.file_state
    }

    fn error_state(&self) -> usize {
        self.error_state
    }

    fn set_error_state(&mut self, error_state: usize) {
        self.error_state = error_state;
    }

    fn open_text_file_for_read(path: &str) -> Result<(Box<dyn pascal_io::ReadLine>, bool), usize> {
        let path = path.trim_end_matches(' ');
        IO_HANDLER_OPEN_TEXT_FILE_FOR_READ.with(|c| c.borrow()(path))
    }

    fn open_binary_file_for_read(path: &str) -> Result<Box<dyn Read>, usize> {
        let path = path.trim_end_matches(' ');
        IO_HANDLER_OPEN_BINARY_FILE_FOR_READ.with(|c| c.borrow()(path))
    }

    fn open_file_for_write(path: &str) -> Result<Box<dyn Write>, usize> {
        let path = path.trim_end_matches(' ');
        IO_HANDLER_OPEN_FILE_FOR_WRITE.with(|c| c.borrow()(path))
    }
}

pub(crate) type packed_file_of<T> = file_of<T>;

/// IO Handlers that allows to override all disk and terminal IO accesses, only used for testing purpose.
pub struct TeXIoHandler {
    /// Callback function for reading text files.
    pub open_text_file_for_read: Box<dyn Fn(&str) -> Result<(Box<dyn ReadLine>, bool), usize>>,
    /// Callback function for reading binary files.
    pub open_binary_file_for_read: Box<dyn Fn(&str) -> Result<Box<dyn Read>, usize>>,
    /// Callback function for writing files.
    pub open_file_for_write: Box<dyn Fn(&str) -> Result<Box<dyn Write>, usize>>,
}

crate::impl_debug_with_literal!(TeXIoHandler, "TeXIoHandler");

/// Install a specific IO handler, only used for testing purpose.
pub fn install_io_handler(handler: TeXIoHandler) {
    let TeXIoHandler {
        open_text_file_for_read,
        open_binary_file_for_read,
        open_file_for_write,
    } = handler;
    IO_HANDLER_OPEN_TEXT_FILE_FOR_READ.with(|c| {
        *c.borrow_mut() = open_text_file_for_read;
    });
    IO_HANDLER_OPEN_BINARY_FILE_FOR_READ.with(|c| {
        *c.borrow_mut() = open_binary_file_for_read;
    });
    IO_HANDLER_OPEN_FILE_FOR_WRITE.with(|c| {
        *c.borrow_mut() = open_file_for_write;
    });
}

/// Revert to the default IO handler, only used for testing purpose.
pub fn reset_io_handler() {
    install_io_handler(TeXIoHandler {
        open_text_file_for_read: Box::new(builtin_open_text_file_for_read),
        open_binary_file_for_read: Box::new(builtin_open_binary_file_for_read),
        open_file_for_write: Box::new(builtin_open_file_for_write),
    })
}

thread_local! {
    pub(crate) static IO_HANDLER_OPEN_TEXT_FILE_FOR_READ:
        RefCell<Box<dyn Fn(&str)-> Result<(Box<dyn ReadLine>, bool), usize>>>
        = RefCell::new(Box::new(builtin_open_text_file_for_read));
    pub(crate) static IO_HANDLER_OPEN_BINARY_FILE_FOR_READ:
        RefCell<Box<dyn Fn(&str)-> Result<Box<dyn Read>, usize>>>
        = RefCell::new(Box::new(builtin_open_binary_file_for_read));
    pub(crate) static IO_HANDLER_OPEN_FILE_FOR_WRITE:
        RefCell<Box<dyn Fn(&str)-> Result<Box<dyn Write>, usize>>>
        = RefCell::new(Box::new(builtin_open_file_for_write));
}

fn builtin_open_text_file_for_read(
    path: &str,
) -> Result<(Box<dyn pascal_io::ReadLine>, bool), usize> {
    let mut term_special_handling = false;
    let new_read_target: Box<dyn ReadLine> = if path == "TTY:" {
        term_special_handling = true;
        Box::new(io::stdin())
    } else {
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                return Err(1);
            }
        };
        Box::new(io::BufReader::new(file))
    };
    Ok((new_read_target, term_special_handling))
}

fn builtin_open_binary_file_for_read(mut path: &str) -> Result<Box<dyn Read>, usize> {
    let new_read_target: Box<dyn Read> = if path == "TTY:" {
        Box::new(io::stdin())
    } else {
        path = path.trim_start_matches("TeXfonts:");
        path = path.trim_start_matches("TeXformats:");
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                return Err(1);
            }
        };
        Box::new(io::BufReader::new(file))
    };
    Ok(new_read_target)
}

fn builtin_open_file_for_write(path: &str) -> Result<Box<dyn Write>, usize> {
    let write_target: Box<dyn Write> = if path == "TTY:" {
        Box::new(io::stdout())
    } else {
        let file = match std::fs::File::create(path) {
            Ok(f) => f,
            Err(_) => {
                return Err(1);
            }
        };
        Box::new(file)
    };
    Ok(write_target)
}

use crate::pascal::char_repr;
use crate::section_0019::text_char;
pub(crate) use pascal_io::{
    break_in, buffer_variable, buffer_variable_assign, close, eof, eoln, erstat, get, put, r#break,
    read_ln, read_onearg, reset, rewrite, write, write_binary, write_ln, write_ln_noargs,
    FileState, FromBlob, PascalFile, ReadLine, ToBlob,
};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    io::{self, Read, Write},
};
