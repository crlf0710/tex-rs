pub type integer = i32;
pub type real = f32;
pub type word = u32;
pub type boolean = bool;

#[cfg(not(feature = "unicode_support"))]
pub(crate) struct char(u8);

#[cfg(not(feature = "unicode_support"))]
impl char {
    pub(crate) const fn new(v: u8) -> Self {
        char(v)
    }
}

#[cfg(feature = "unicode_support")]
pub(crate) struct char(u32, PhantomData<Rc<()>>);

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

pub(crate) struct IoTarget {
    input_target: Box<dyn Read>,
    output_target: Box<dyn Write>,
}

#[allow(unused_variables)]
pub(crate) trait PascalFile {
    type Unit;

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
    type Unit = crate::section_0019::text_char;

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
    type Unit = T;

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
    type Unit = T;

    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
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
pub(crate) fn caret<F: PascalFile>(file: &mut F) -> F::Unit {
    todo!();
}

#[allow(unused_variables)]
pub(crate) fn eof<F: PascalFile>(file: &mut F) -> bool {
    todo!();
}

#[allow(unused_variables)]
pub(crate) fn eoln<F: PascalFile>(file: &mut F) -> bool {
    todo!();
}

#[allow(unused_variables)]
pub(crate) fn get<F: PascalFile>(file: &mut F) {
    todo!();
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

pub(crate) fn close<F: PascalFile>(file: &mut F) {
    *file.io_target_mut() = IoTarget::default();
}

use core::fmt::{self, Display};
use core::marker::PhantomData;
use std::io::{self, Read, Write};
use std::rc::Rc;
use typenum::Unsigned;
