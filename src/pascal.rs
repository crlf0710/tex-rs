pub type integer = i32;
pub type real = f32;

// TODO: Implement this.
pub struct ranged_unsigned_integer<B, MIN, MAX>(PhantomData<B>, PhantomData<MIN>, PhantomData<MAX>);

pub(crate) trait PascalFile {}

// TODO: Implement this.
pub struct packed_file_of_text_char();

impl Default for packed_file_of_text_char {
    fn default() -> Self {
        packed_file_of_text_char()
    }
}

impl PascalFile for packed_file_of_text_char {}

// TODO: Implement this.
pub struct packed_file_of<T>(PhantomData<T>);

impl<T> Default for packed_file_of<T> {
    fn default() -> Self {
        packed_file_of(PhantomData)
    }
}

impl<T> PascalFile for packed_file_of<T> {}

// TODO: Implement this.
pub struct file_of<T>(PhantomData<T>);

impl<T> Default for file_of<T> {
    fn default() -> Self {
        file_of(PhantomData)
    }
}

impl<T> PascalFile for file_of<T> {}

#[allow(unused_variables)]
pub(crate) fn write<F: PascalFile, T: Display>(file: &F, val: T) {
    unimplemented!()
}

#[allow(unused_variables)]
pub(crate) fn write_ln<F: PascalFile, T: Display>(file: &F, val: T) {
    unimplemented!()
}

#[allow(unused_variables)]
pub(crate) fn write_ln_noargs<F: PascalFile>(file: &F) {
    unimplemented!()
}

#[allow(unused_variables)]
pub(crate) fn reset<F: PascalFile>(file: &mut F, path: &str, options: &str) {
    unimplemented!()
}

#[allow(unused_variables)]
pub(crate) fn rewrite<F: PascalFile>(file: &mut F, path: &str, options: &str) {
    unimplemented!()
}

use core::fmt::Display;
use core::marker::PhantomData;
