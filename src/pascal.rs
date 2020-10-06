pub type integer = i32;
pub type real = f32;

// TODO: Implement this.
pub struct ranged_unsigned_integer<B, MIN, MAX>(PhantomData<B>, PhantomData<MIN>, PhantomData<MAX>);
// TODO: Implement this.
pub struct packed_file_of_text_char();
// TODO: Implement this.
pub struct packed_file_of<T>(PhantomData<T>);
// TODO: Implement this.
pub struct file_of<T>(PhantomData<T>);

use core::marker::PhantomData;
