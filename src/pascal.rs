pub type integer = i32;
pub type real = f32;
pub type word = u32;
pub type boolean = bool;

#[cfg(not(feature = "unicode_support"))]
#[derive(Copy, Clone)]
pub(crate) struct char(pub(crate) u8);

#[cfg(not(feature = "unicode_support"))]
impl char {
    pub(crate) const fn new(v: u8) -> Self {
        char(v)
    }
}

#[cfg(feature = "unicode_support")]
#[derive(Copy, Clone)]
pub(crate) struct char(pub(crate) u32, PhantomData<Rc<()>>);

#[cfg(feature = "unicode_support")]
impl char {
    pub(crate) const fn new(v: u32) -> Self {
        char(v, PhantomData)
    }
}

macro_rules! define_ranged_unsigned_integer {
    ($v:vis $name:ident => $base_type:path; $typenum_const:ident) => {
        // TODO: Implement this.
        $v struct $name<MIN, MAX>($base_type, PhantomData<(MIN, MAX)>);

        impl<MIN, MAX> $name<MIN, MAX> where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            $v fn new(val: $base_type) -> Self {
                //use static_assertions::const_assert;
                debug_assert!(val >= <MIN as typenum::Unsigned>::$typenum_const);
                debug_assert!(val <= <MAX as typenum::Unsigned>::$typenum_const);
                //TODO: Add more checks here.
                $name(val, PhantomData)
            }

            $v fn get(self) -> $base_type {
                self.0
            }
        }

        impl<MIN, MAX> Clone for $name<MIN, MAX> {
            fn clone(&self) -> Self {
                $name(self.0.clone(), PhantomData)
            }
        }

        impl<MIN, MAX> Copy for $name<MIN, MAX> {}

        impl<MIN, MAX> Default for $name<MIN, MAX> where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            fn default() -> Self {
                let val = <MIN as typenum::Unsigned>::$typenum_const;
                debug_assert!(val <= <MAX as typenum::Unsigned>::$typenum_const);
                $name(val, PhantomData)
            }
        }

        impl<MIN, MAX> PartialEq<$name<MIN, MAX>> for $name<MIN, MAX> {
            fn eq(&self, rhs: &Self) -> bool {
                self.0.eq(&rhs.0)
            }
        }

        impl<MIN, MAX> PartialEq<$base_type> for $name<MIN, MAX> {
            fn eq(&self, rhs: &$base_type) -> bool {
                self.0.eq(rhs)
            }
        }

        impl<MIN, MAX> PartialOrd<$name<MIN, MAX>> for $name<MIN, MAX> {
            fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&rhs.0)
            }
        }

        impl<MIN, MAX> core::ops::AddAssign<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            fn add_assign(&mut self, rhs: $base_type) {
                let result = (self.0).checked_add(rhs).expect("overflowed");
                if result > <MAX as typenum::Unsigned>::$typenum_const {
                    panic!("overflowed");
                }
                self.0 = result;
            }
        }

        impl<MIN, MAX> core::ops::SubAssign<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            fn sub_assign(&mut self, rhs: $base_type) {
                let result = (self.0).checked_sub(rhs).expect("underflowed");
                if result < <MIN as typenum::Unsigned>::$typenum_const {
                    panic!("underflowed");
                }
                self.0 = result;
            }
        }

        impl<MIN, MAX> core::ops::Add<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            type Output = Self;

            fn add(mut self, rhs: $base_type) -> Self {
                self += rhs;
                self
            }
        }

        impl<MIN, MAX> core::ops::Sub<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            type Output = Self;

            fn sub(mut self, rhs: $base_type) -> Self {
                self -= rhs;
                self
            }
        }
    };
}

define_ranged_unsigned_integer!(pub u8_from_m_to_n => u8; U8);
define_ranged_unsigned_integer!(pub u16_from_m_to_n => u16; U16);
define_ranged_unsigned_integer!(pub u32_from_m_to_n => u32; U32);

pub type u8_from_0_to_n<N> = u8_from_m_to_n<typenum::U0, N>;
pub type u16_from_0_to_n<N> = u16_from_m_to_n<typenum::U0, N>;
pub type u32_from_0_to_n<N> = u32_from_m_to_n<typenum::U0, N>;

/* // Rustc is not ready to accept this:
macro_rules! define_array_keyed_with_ranged_unsigned_integer_from_0 {
    ($v:vis $name:ident => $base_type:path; $typenum_const:ident) => {
        // TODO: Implement this.
        $v struct $name<ELEMENT, N: typenum::Unsigned>
        ([ELEMENT; <N as typenum::Unsigned>::$typenum_const as usize], PhantomData<N>);

        impl<ELEMENT, N: typenum::Unsigned> $name<ELEMENT, N> {
        }

        impl<ELEMENT, N: typenum::Unsigned> Clone for $name<ELEMENT, N>
        where
            [ELEMENT; <N as typenum::Unsigned>::$typenum_const as usize]: Clone {
            fn clone(&self) -> Self {
                $name(self.0.clone(), PhantomData)
            }
        }

        impl<ELEMENT, N: typenum::Unsigned> Copy for $name<ELEMENT, N>
        where
            [ELEMENT; <N as typenum::Unsigned>::$typenum_const as usize]: Copy {}
    };
}

define_array_keyed_with_ranged_unsigned_integer_from_0!(pub array_keyed_with_u8_from_0_to_n => u8; U8);
define_array_keyed_with_ranged_unsigned_integer_from_0!(pub array_keyed_with_u16_from_0_to_n => u16; U16);
define_array_keyed_with_ranged_unsigned_integer_from_0!(pub array_keyed_with_u32_from_0_to_n => u32; U32);

*/
macro_rules! make_default_array {
    ($elem_ty:ty; $elem_cnt:expr) => {{
        use core::mem::{self, MaybeUninit};
        let mut data: [MaybeUninit<$elem_ty>; $elem_cnt] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut data[..] {
            *elem = MaybeUninit::new(Default::default());
        }
        let data = MaybeUninit::new(data);
        unsafe { mem::transmute_copy::<_, [$elem_ty; $elem_cnt]>(&data) }
    }};
}

macro_rules! make_copied_array {
    ($elem_ty:ty; $elem_val:expr; $elem_cnt:expr) => {{
        use core::mem::{self, MaybeUninit};
        let mut data: [MaybeUninit<$elem_ty>; $elem_cnt] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut data[..] {
            *elem = MaybeUninit::new($elem_val);
        }
        let data = MaybeUninit::new(data);
        unsafe { mem::transmute_copy::<_, [$elem_ty; $elem_cnt]>(&data) }
    }};
}

macro_rules! define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length {
    ($v:vis $name:ident[$index_type:path] => $base_index_type:path; $typenum_const:ident; $len_typenum:path) => {
        $v struct $name<ELEMENT>
        ([ELEMENT; <$len_typenum as typenum::Unsigned>::$typenum_const as usize]);

        impl<ELEMENT> $name<ELEMENT> {
        }

        impl<ELEMENT> Clone for $name<ELEMENT>
        where
            [ELEMENT; <$len_typenum as typenum::Unsigned>::$typenum_const as usize]: Clone {
            fn clone(&self) -> Self {
                $name(self.0.clone())
            }
        }

        impl<ELEMENT> Copy for $name<ELEMENT>
        where
            [ELEMENT; <$len_typenum as typenum::Unsigned>::$typenum_const as usize]: Copy {}

        impl<ELEMENT> Default for $name<ELEMENT>
        where
            ELEMENT: Default {
            fn default() -> Self {
                $name(make_default_array!(ELEMENT;<$len_typenum as typenum::Unsigned>::$typenum_const as usize ))
            }
        }

        impl<ELEMENT> core::ops::Index<$index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $index_type) -> &ELEMENT {
                &(self.0)[idx.get() as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<$index_type> for $name<ELEMENT>
        {
            fn index_mut(&mut self, idx: $index_type) -> &mut ELEMENT {
                &mut (self.0)[idx.get() as usize]
            }
        }

        impl<ELEMENT> core::ops::Index<$base_index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $base_index_type) -> &ELEMENT {
                &(self.0)[idx as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<$base_index_type> for $name<ELEMENT>
        {
            fn index_mut(&mut self, idx: $base_index_type) -> &mut ELEMENT {
                &mut (self.0)[idx as usize]
            }
        }
    };
}

macro_rules! define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length {
    ($v:vis $name:ident[$index_type:path] => $base_index_type:path; $typenum_const:ident; $start_typenum:path; $length_typenum:path) => {
        $v struct $name<ELEMENT>
        ([ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_const as usize]);

        impl<ELEMENT> $name<ELEMENT> {
            $v fn iter(&self) -> core::slice::Iter<'_, ELEMENT> {
                (self.0)[..].iter()
            }
        }

        impl<ELEMENT> $name<ELEMENT>
        where
            ELEMENT: Copy {
            $v fn from_copied(val: ELEMENT) -> Self {
                $name(make_copied_array!(ELEMENT; val; <$length_typenum as typenum::Unsigned>::$typenum_const as usize ))
            }
        }

        impl<ELEMENT> Clone for $name<ELEMENT>
        where
            [ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_const as usize]: Clone {
            fn clone(&self) -> Self {
                $name(self.0.clone())
            }
        }

        impl<ELEMENT> Copy for $name<ELEMENT>
        where
            [ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_const as usize]: Copy {}

        impl<ELEMENT> Default for $name<ELEMENT>
        where
            ELEMENT: Default {
            fn default() -> Self {
                $name(make_default_array!(ELEMENT;<$length_typenum as typenum::Unsigned>::$typenum_const as usize ))
            }
        }

        impl<ELEMENT> core::ops::Index<$index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $index_type) -> &ELEMENT {
                &(self.0)[idx.get() as usize - <$start_typenum as typenum::Unsigned>::$typenum_const as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<$index_type> for $name<ELEMENT>
        {
            fn index_mut(&mut self, idx: $index_type) -> &mut ELEMENT {
                &mut (self.0)[idx.get() as usize - <$start_typenum as typenum::Unsigned>::$typenum_const as usize]
            }
        }

        impl<ELEMENT> core::ops::Index<$base_index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $base_index_type) -> &ELEMENT {
                &(self.0)[idx as usize - <$start_typenum as typenum::Unsigned>::$typenum_const as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<$base_index_type> for $name<ELEMENT>
        {
            fn index_mut(&mut self, idx: $base_index_type) -> &mut ELEMENT {
                &mut (self.0)[idx as usize - <$start_typenum as typenum::Unsigned>::$typenum_const as usize]
            }
        }
    };
}

pub(crate) trait ReadLine {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
}

impl ReadLine for io::Stdin {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        io::Stdin::read_line(self, buf)
    }
}

impl<R: io::Read> ReadLine for io::BufReader<R> {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        <Self as io::BufRead>::read_line(self, buf)
    }
}

pub(crate) enum LineBufferState<T> {
    UnknownState {
        initial_line: bool,
    },
    AfterReadLine {
        line_buffer: Vec<T>,
        line_position: usize,
        line_no_more: bool,
    },
    Eof,
}

pub(crate) enum BlockBufferState<T> {
    UnknownState,
    AfterReadBlock {
        bytes_buffer: Box<[u8]>,
        bytes_avail_length: usize,
        bytes_position: usize,
        bytes_caret: Option<(T, usize)>,
    },
    Eof,
}

pub(crate) enum FileState<T> {
    Undefined,
    GenerationMode {
        write_caret: Option<T>,
        write_target: Box<dyn Write>,
    },
    LineInspectionMode {
        read_line_buffer: LineBufferState<T>,
        read_target: Box<dyn ReadLine>,
    },
    BlockInspectionMode {
        read_block_buffer: BlockBufferState<T>,
        read_target: Box<dyn Read>,
    },
}

impl<T> Default for FileState<T> {
    fn default() -> Self {
        FileState::Undefined
    }
}

impl<T> FileState<T> {
    fn discard_caret_and_get_write_target(&mut self) -> &mut dyn Write {
        match self {
            FileState::GenerationMode {
                write_caret,
                write_target,
            } => {
                *write_caret = None;
                write_target.as_mut()
            }
            _ => {
                panic!("file not in generation mode!");
            }
        }
    }

    fn refill<F>(&mut self)
    where
        F: PascalFile<Unit = T>,
    {
        match self {
            FileState::LineInspectionMode {
                read_line_buffer,
                read_target,
            } => match read_line_buffer {
                LineBufferState::UnknownState { initial_line } => {
                    let initial_line = *initial_line;
                    let mut buf = String::new();
                    read_target.read_line(&mut buf).expect("read line failure");
                    if initial_line && buf.is_empty() {
                        *read_line_buffer = LineBufferState::Eof;
                        return;
                    }
                    let mut line_chars = vec![];
                    F::convert_line_string_crlf_to_lf(&mut buf);
                    F::convert_line_string_to_units(&buf, &mut line_chars);
                    let no_more = match line_chars.last() {
                        Some(c) => !F::is_eoln_unit(c),
                        None => true,
                    };
                    if no_more {
                        line_chars.push(F::eoln_unit());
                    }
                    *read_line_buffer = LineBufferState::AfterReadLine {
                        line_buffer: line_chars,
                        line_position: 0,
                        line_no_more: no_more,
                    };
                    return;
                }
                _ => unreachable!(),
            },
            FileState::BlockInspectionMode { .. } => {
                todo!();
            }
            _ => {
                panic!("file not in inspection mode!");
            }
        }
    }
}

#[allow(unused_variables)]
pub(crate) trait PascalFile {
    type Unit;

    fn is_text_file() -> bool;

    fn is_eoln_unit(unit: &Self::Unit) -> bool;

    fn eoln_unit() -> Self::Unit;

    fn convert_line_string_crlf_to_lf(input: &mut String);

    fn convert_line_string_to_units(input: &str, units: &mut Vec<Self::Unit>);

    fn file_state(&self) -> &FileState<Self::Unit>;

    fn file_state_mut(&mut self) -> &mut FileState<Self::Unit>;

    fn put_unit_to_target(&mut self, val: Self::Unit) {
        todo!();
    }
}

pub(crate) struct file_of_text_char(FileState<text_char>);

impl Default for file_of_text_char {
    fn default() -> Self {
        file_of_text_char(FileState::default())
    }
}

impl PascalFile for file_of_text_char {
    type Unit = text_char;

    fn is_text_file() -> bool {
        true
    }

    fn is_eoln_unit(unit: &Self::Unit) -> bool {
        unit.0 == b'\n' as _
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
                        todo!();
                    }
                }
            }
        }
    }

    fn file_state(&self) -> &FileState<text_char> {
        &self.0
    }

    fn file_state_mut(&mut self) -> &mut FileState<text_char> {
        &mut self.0
    }
}

pub(crate) type packed_file_of_text_char = file_of_text_char;

pub(crate) struct file_of<T>(FileState<T>);

impl<T> Default for file_of<T> {
    fn default() -> Self {
        file_of(FileState::default())
    }
}

impl<T> PascalFile for file_of<T> {
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

    fn file_state(&self) -> &FileState<T> {
        &self.0
    }

    fn file_state_mut(&mut self) -> &mut FileState<T> {
        &mut self.0
    }
}

pub(crate) type packed_file_of<T> = file_of<T>;

#[allow(unused_variables)]
pub(crate) fn rewrite<F: PascalFile, P: Into<String>>(file: &mut F, path: P, options: &str) {
    let path = path.into();
    let new_write_target: Box<dyn Write> = if path == "TTY:" {
        Box::new(io::stdout())
    } else {
        unimplemented!()
    };
    *file.file_state_mut() = FileState::GenerationMode {
        write_target: new_write_target,
        write_caret: None,
    };
}

#[allow(unused_variables)]
pub(crate) fn put<F: PascalFile>(file: &mut F) {
    match file.file_state_mut() {
        FileState::GenerationMode {
            write_target,
            write_caret,
        } => {
            let caret_value = write_caret
                .take()
                .expect("file buffer variable value is undefined!");
            file.put_unit_to_target(caret_value);
        }
        _ => {
            panic!("file not in generation mode!");
        }
    }
}

#[allow(unused_variables)]
pub(crate) fn reset<F: PascalFile, P: Into<String>>(file: &mut F, path: P, options: &str) {
    let path = path.into();
    if F::is_text_file() {
        let new_read_target: Box<dyn ReadLine> = if path == "TTY:" {
            Box::new(io::stdin())
        } else if path == crate::section_0011::pool_name {
            unimplemented!();
        } else {
            unimplemented!()
        };
        *file.file_state_mut() = FileState::LineInspectionMode {
            read_target: new_read_target,
            read_line_buffer: LineBufferState::UnknownState { initial_line: true },
        };
    } else {
        let new_read_target: Box<dyn Read> = if path == "TTY:" {
            Box::new(io::stdin())
        } else {
            unimplemented!()
        };
        *file.file_state_mut() = FileState::BlockInspectionMode {
            read_target: new_read_target,
            read_block_buffer: BlockBufferState::UnknownState,
        };
    }
}

#[allow(unused_variables)]
pub(crate) fn get<F: PascalFile>(file: &mut F) {
    loop {
        match file.file_state_mut() {
            FileState::LineInspectionMode {
                read_line_buffer, ..
            } => match read_line_buffer {
                LineBufferState::Eof => {
                    panic!("file eof reached");
                }
                LineBufferState::UnknownState { .. } => {
                    file.file_state_mut().refill::<F>();
                    continue;
                }
                LineBufferState::AfterReadLine {
                    line_buffer,
                    line_position,
                    line_no_more,
                } => {
                    if *line_position + 1 < line_buffer.len() {
                        *line_position += 1;
                    } else {
                        if *line_no_more {
                            *read_line_buffer = LineBufferState::Eof
                        } else {
                            *read_line_buffer = LineBufferState::UnknownState {
                                initial_line: false,
                            };
                        }
                    }
                }
            },
            FileState::BlockInspectionMode { .. } => {
                todo!();
            }
            _ => {
                panic!("file not in inspection mode");
            }
        }
        return;
    }
}

#[allow(unused_variables)]
pub(crate) fn buffer_variable<F: PascalFile>(file: &mut F) -> F::Unit
where
    F::Unit: Clone,
{
    loop {
        match file.file_state() {
            FileState::LineInspectionMode {
                read_line_buffer,
                read_target,
            } => match read_line_buffer {
                LineBufferState::Eof => {
                    panic!("file eof reached");
                }
                LineBufferState::UnknownState { .. } => {
                    file.file_state_mut().refill::<F>();
                    continue;
                }
                LineBufferState::AfterReadLine {
                    line_buffer,
                    line_position,
                    ..
                } => {
                    return line_buffer[*line_position].clone();
                }
            },
            FileState::BlockInspectionMode {
                read_block_buffer,
                read_target,
            } => {
                todo!();
            }
            _ => panic!("file not in inspection mode"),
        }
    }
}

pub(crate) fn eof<F: PascalFile>(file: &mut F) -> bool {
    loop {
        match file.file_state() {
            FileState::LineInspectionMode {
                read_line_buffer, ..
            } => match read_line_buffer {
                LineBufferState::Eof => {
                    return true;
                }
                LineBufferState::UnknownState { .. } => {
                    file.file_state_mut().refill::<F>();
                    continue;
                }
                LineBufferState::AfterReadLine { .. } => {
                    return false;
                }
            },
            FileState::BlockInspectionMode { .. } => {
                todo!();
            }
            FileState::GenerationMode { .. } => {
                return true;
            }
            _ => panic!("file not in any mode"),
        }
    }
}

#[allow(unused_variables)]
pub(crate) fn eoln<F: PascalFile>(file: &mut F) -> bool {
    loop {
        match file.file_state() {
            FileState::LineInspectionMode {
                read_line_buffer, ..
            } => match read_line_buffer {
                LineBufferState::Eof => {
                    panic!("file eof reached");
                }
                LineBufferState::UnknownState { .. } => {
                    file.file_state_mut().refill::<F>();
                    continue;
                }
                LineBufferState::AfterReadLine {
                    line_buffer,
                    line_position,
                    ..
                } => {
                    return F::is_eoln_unit(&line_buffer[*line_position]);
                }
            },
            FileState::BlockInspectionMode { .. } => panic!("file is not text file"),
            _ => panic!("file not in inspection mode"),
        }
    }
}

pub(crate) fn write<F: PascalFile, T: Display>(file: &mut F, val: T) {
    let write_target = file.file_state_mut().discard_caret_and_get_write_target();
    write!(write_target, "{}", val).unwrap();
}

pub(crate) fn write_ln<F: PascalFile, T: Display>(file: &mut F, val: T) {
    let write_target = file.file_state_mut().discard_caret_and_get_write_target();
    writeln!(write_target, "{}", val).unwrap();
}

pub(crate) fn write_ln_noargs<F: PascalFile>(file: &mut F) {
    let write_target = file.file_state_mut().discard_caret_and_get_write_target();
    writeln!(write_target, "").unwrap();
}

pub(crate) fn r#break<F: PascalFile>(file: &mut F) {
    let write_target = file.file_state_mut().discard_caret_and_get_write_target();
    write_target.flush().unwrap();
}

#[allow(unused_variables)]
pub(crate) fn erstat<F: PascalFile>(file: &mut F) -> integer {
    todo!();
}

pub(crate) fn close<F: PascalFile>(file: &mut F) {
    *file.file_state_mut() = FileState::default();
}

use crate::section_0019::text_char;
use core::fmt::{self, Display};
use core::marker::PhantomData;
use std::io::{self, Read, Write};
use std::rc::Rc;
use typenum::Unsigned;
