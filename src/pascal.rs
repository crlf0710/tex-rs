pub type integer = i32;
pub type real = f32;
pub type word = u32;

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
            ELEMENT: Default + Copy {
            fn default() -> Self {
                $name([Default::default(); <$len_typenum as typenum::Unsigned>::$typenum_const as usize])
            }
        }

        impl<ELEMENT> core::ops::Index<$index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $index_type) -> &ELEMENT {
                &(self.0)[idx.get() as usize]
            }
        }

        impl<ELEMENT> core::ops::Index<$base_index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $base_index_type) -> &ELEMENT {
                &(self.0)[idx as usize]
            }
        }
    };
}

macro_rules! define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length {
    ($v:vis $name:ident[$index_type:path] => $base_index_type:path; $typenum_const:ident; $start_typenum:path; $length_typenum:path) => {
        $v struct $name<ELEMENT>
        ([ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_const as usize]);

        impl<ELEMENT> $name<ELEMENT> {
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
            ELEMENT: Default + Copy {
            fn default() -> Self {
                $name([Default::default(); <$length_typenum as typenum::Unsigned>::$typenum_const as usize])
            }
        }

        impl<ELEMENT> core::ops::Index<$index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $index_type) -> &ELEMENT {
                &(self.0)[idx.get() as usize + <$start_typenum as typenum::Unsigned>::$typenum_const as usize]
            }
        }

        impl<ELEMENT> core::ops::Index<$base_index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $base_index_type) -> &ELEMENT {
                &(self.0)[idx as usize + <$start_typenum as typenum::Unsigned>::$typenum_const as usize]
            }
        }
    };
}

pub(crate) struct IoTarget {
    input_target: Box<dyn Read>,
    output_target: Box<dyn Write>,
}

#[allow(unused_variables)]
pub(crate) trait PascalFile {
    fn io_target(&self) -> &IoTarget;
    fn io_target_mut(&mut self) -> &mut IoTarget;
    fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<()> {
        self.io_target_mut().output_target.write_fmt(args)
    }
}

impl Default for IoTarget {
    fn default() -> Self {
        IoTarget {
            input_target: Box::new(io::empty()),
            output_target: Box::new(io::sink()),
        }
    }
}

// TODO: Implement this.
pub struct packed_file_of_text_char(IoTarget);

impl Default for packed_file_of_text_char {
    fn default() -> Self {
        packed_file_of_text_char(IoTarget::default())
    }
}

impl PascalFile for packed_file_of_text_char {
    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
}

// TODO: Implement this.
pub struct packed_file_of<T>(IoTarget, PhantomData<T>);

impl<T> Default for packed_file_of<T> {
    fn default() -> Self {
        packed_file_of(IoTarget::default(), PhantomData)
    }
}

impl<T> PascalFile for packed_file_of<T> {
    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
}

// TODO: Implement this.
pub struct file_of<T>(IoTarget, PhantomData<T>);

impl<T> Default for file_of<T> {
    fn default() -> Self {
        file_of(IoTarget::default(), PhantomData)
    }
}

impl<T> PascalFile for file_of<T> {
    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
}

#[allow(unused_variables)]
pub(crate) fn write<F: PascalFile, T: Display>(file: &mut F, val: T) {
    write!(file, "{}", val).unwrap();
}

#[allow(unused_variables)]
pub(crate) fn write_ln<F: PascalFile, T: Display>(file: &mut F, val: T) {
    writeln!(file, "{}", val).unwrap();
}

#[allow(unused_variables)]
pub(crate) fn write_ln_noargs<F: PascalFile>(file: &mut F) {
    writeln!(file, "").unwrap();
}

#[allow(unused_variables)]
pub(crate) fn reset<F: PascalFile>(file: &mut F, path: &str, options: &str) {
    let new_input_target: Box<dyn Read> = if path == "TTY:" {
        Box::new(io::stdin())
    } else {
        unimplemented!()
    };
    file.io_target_mut().input_target = new_input_target;
}

#[allow(unused_variables)]
pub(crate) fn rewrite<F: PascalFile>(file: &mut F, path: &str, options: &str) {
    let new_output_target: Box<dyn Write> = if path == "TTY:" {
        Box::new(io::stdout())
    } else {
        unimplemented!()
    };
    file.io_target_mut().output_target = new_output_target;
}

pub(crate) fn r#break<F: PascalFile>(file: &mut F) {
    file.io_target_mut().output_target.flush().unwrap();
}

use core::fmt::{self, Display};
use core::marker::PhantomData;
use std::io::{self, Read, Write};
use typenum::Unsigned;
