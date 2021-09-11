pub type integer = i32;
pub type real = f32;
pub type word = u32;
pub type boolean = bool;

pub(crate) trait IsOddOrEven {
    fn is_odd(&self) -> boolean;
    fn is_even(&self) -> boolean {
        !self.is_odd()
    }
}

pub(crate) macro impl_is_even_or_odd_for_primitive($t:ty) {
    impl crate::pascal::IsOddOrEven for $t {
        fn is_odd(&self) -> crate::pascal::boolean {
            self % 2 != 0
        }
    }
}

impl_is_even_or_odd_for_primitive!(u8);
impl_is_even_or_odd_for_primitive!(u16);
impl_is_even_or_odd_for_primitive!(u32);
impl_is_even_or_odd_for_primitive!(i8);
impl_is_even_or_odd_for_primitive!(i16);
impl_is_even_or_odd_for_primitive!(i32);

#[cfg(not(feature = "unicode_support"))]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Debug)]
pub(crate) struct char(pub(crate) u8);

#[cfg(not(feature = "unicode_support"))]
type char_repr = u8;

#[cfg(not(feature = "unicode_support"))]
impl core::fmt::Debug for char {}

#[cfg(not(feature = "unicode_support"))]
impl char {
    pub(crate) const MAX: char = char(255);

    pub(crate) const fn new(v: u8) -> Self {
        char(v)
    }
}

#[cfg(feature = "unicode_support")]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub(crate) struct char(pub(crate) u32, PhantomData<Rc<()>>);

#[cfg(feature = "unicode_support")]
impl char {
    pub(crate) const MAX: char = char(0x007F_FFFF, PhantomData);
    pub(crate) const fn new(v: u32) -> Self {
        char(v, PhantomData)
    }
}

#[cfg(feature = "unicode_support")]
type char_repr = u32;

#[cfg(feature = "unicode_support")]
impl core::fmt::Debug for char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(*self <= char::MAX);
        f.debug_list()
            .entries(crate::unicode_support::chars_from_generalized_char(*self))
            .finish()?;
        Ok(())
    }
}

#[cfg(feature = "unicode_support")]
impl core::fmt::Display for char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(*self <= char::MAX);
        for ch in crate::unicode_support::chars_from_generalized_char(*self) {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

macro_rules! define_ranged_unsigned_integer {
    ($v:vis $name:ident => $base_type:path; $typenum_const:ident) => {
        // TODO: Implement this.
        $v struct $name<MIN, MAX>($base_type, PhantomData<(MIN, MAX)>);

        impl<MIN, MAX> $name<MIN, MAX> where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            $v fn new(val: $base_type) -> Self {
                //use static_assertions::const_assert;
                assert!(val >= <MIN as typenum::Unsigned>::$typenum_const);
                assert!(val <= <MAX as typenum::Unsigned>::$typenum_const);
                //TODO: Add more checks here.
                $name(val, PhantomData)
            }

            $v fn get(self) -> $base_type {
                self.0
            }
        }

        impl<MIN, MAX> $name<MIN, MAX> {
            $v const unsafe fn new_unchecked(val: $base_type) -> Self {
                $name(val, PhantomData)
            }
        }

        impl<MIN, MAX> From<$base_type> for $name<MIN, MAX> where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            fn from(v: $base_type) -> Self {
                $name::new(v)
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

        impl<MIN, MAX> core::fmt::Debug for $name<MIN, MAX> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)?;
                Ok(())
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

        impl<MIN, MAX> PartialOrd<$base_type> for $name<MIN, MAX> {
            fn partial_cmp(&self, rhs: &$base_type) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(rhs)
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

        impl<MIN, MAX> core::ops::Add<$name<MIN, MAX>> for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            type Output = Self;

            fn add(mut self, rhs: $name<MIN, MAX>) -> Self {
                self += rhs.get();
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

        impl<MIN, MAX> core::ops::Sub<$name<MIN, MAX>> for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            type Output = Self;

            fn sub(mut self, rhs: $name<MIN, MAX>) -> Self {
                self -= rhs.get();
                self
            }
        }

        impl<MIN, MAX> crate::pascal::IsOddOrEven for $name<MIN, MAX>
        where MIN: typenum::Unsigned, MAX: typenum::Unsigned {
            fn is_odd(&self) -> crate::pascal::boolean {
                self.get().is_odd()
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

macro_rules! define_ranged_signed_integer {
    ($v:vis $name:ident => $base_type:path; $typenum_const:ident) => {
        // TODO: Implement this.
        $v struct $name<MIN, MAX>($base_type, PhantomData<(MIN, MAX)>);

        impl<MIN, MAX> $name<MIN, MAX> where MIN: typenum::Integer, MAX: typenum::Integer {
            $v fn new(val: $base_type) -> Self {
                //use static_assertions::const_assert;
                assert!(val >= <MIN as typenum::Integer>::$typenum_const);
                assert!(val <= <MAX as typenum::Integer>::$typenum_const);
                //TODO: Add more checks here.
                $name(val, PhantomData)
            }

            $v fn get(self) -> $base_type {
                self.0
            }
        }

        impl<MIN, MAX> From<$base_type> for $name<MIN, MAX> where MIN: typenum::Integer, MAX: typenum::Integer {
            fn from(v: $base_type) -> Self {
                $name::new(v)
            }
        }

        impl<MIN, MAX> Clone for $name<MIN, MAX> {
            fn clone(&self) -> Self {
                $name(self.0.clone(), PhantomData)
            }
        }

        impl<MIN, MAX> Copy for $name<MIN, MAX> {}

        impl<MIN, MAX> Default for $name<MIN, MAX> where MIN: typenum::Integer, MAX: typenum::Integer {
            fn default() -> Self {
                let val;
                if 0 >= <MIN as typenum::Integer>::$typenum_const &&
                    0 <= <MAX as typenum::Integer>::$typenum_const {
                    val = 0;
                } else {
                    val = <MIN as typenum::Integer>::$typenum_const;
                }
                debug_assert!(val >= <MIN as typenum::Integer>::$typenum_const);
                debug_assert!(val <= <MAX as typenum::Integer>::$typenum_const);
                $name(val, PhantomData)
            }
        }

        impl<MIN, MAX> core::fmt::Debug for $name<MIN, MAX> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)?;
                Ok(())
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

        impl<MIN, MAX> PartialOrd<$base_type> for $name<MIN, MAX> {
            fn partial_cmp(&self, rhs: &$base_type) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(rhs)
            }
        }

        impl<MIN, MAX> core::ops::Neg for $name<MIN, MAX>
        where MIN: typenum::Integer, MAX: typenum::Integer {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new(-self.0)
            }
        }

        impl<MIN, MAX> core::ops::AddAssign<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Integer, MAX: typenum::Integer {
            fn add_assign(&mut self, rhs: $base_type) {
                let result = (self.0).checked_add(rhs).expect("overflowed");
                if result > <MAX as typenum::Integer>::$typenum_const {
                    panic!("overflowed");
                }
                self.0 = result;
            }
        }

        impl<MIN, MAX> core::ops::SubAssign<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Integer, MAX: typenum::Integer {
            fn sub_assign(&mut self, rhs: $base_type) {
                let result = (self.0).checked_sub(rhs).expect("underflowed");
                if result < <MIN as typenum::Integer>::$typenum_const {
                    panic!("underflowed");
                }
                self.0 = result;
            }
        }

        impl<MIN, MAX> core::ops::Add<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Integer, MAX: typenum::Integer {
            type Output = Self;

            fn add(mut self, rhs: $base_type) -> Self {
                self += rhs;
                self
            }
        }

        impl<MIN, MAX> core::ops::Sub<$base_type> for $name<MIN, MAX>
        where MIN: typenum::Integer, MAX: typenum::Integer {
            type Output = Self;

            fn sub(mut self, rhs: $base_type) -> Self {
                self -= rhs;
                self
            }
        }
    };
}

define_ranged_signed_integer!(pub i8_from_m_to_n => i8; I8);
define_ranged_signed_integer!(pub i16_from_m_to_n => i16; I16);
define_ranged_signed_integer!(pub i32_from_m_to_n => i32; I32);

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
pub(crate) macro make_default_array($elem_ty:ty; $elem_cnt:expr) {{
    use core::mem::{self, MaybeUninit};
    let mut data: [MaybeUninit<$elem_ty>; $elem_cnt] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for elem in &mut data[..] {
        *elem = MaybeUninit::new(Default::default());
    }
    let data = MaybeUninit::new(data);
    unsafe { mem::transmute_copy::<_, [$elem_ty; $elem_cnt]>(&data) }
}}

pub(crate) macro make_copied_array($elem_ty:ty; $elem_val:expr; $elem_cnt:expr) {{
    use core::mem::{self, MaybeUninit};
    let mut data: [MaybeUninit<$elem_ty>; $elem_cnt] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for elem in &mut data[..] {
        *elem = MaybeUninit::new($elem_val);
    }
    let data = MaybeUninit::new(data);
    unsafe { mem::transmute_copy::<_, [$elem_ty; $elem_cnt]>(&data) }
}}

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
                $name(crate::pascal::make_default_array!(ELEMENT;<$len_typenum as typenum::Unsigned>::$typenum_const as usize ))
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

        impl<ELEMENT> core::ops::Index<core::ops::Range<$index_type>> for $name<ELEMENT>
        {
            type Output = [ELEMENT];
            fn index(&self, range: core::ops::Range<$index_type>) -> &[ELEMENT] {
                &(self.0)[range.start.get() as usize..range.end.get() as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<core::ops::Range<$index_type>> for $name<ELEMENT>
        {
            fn index_mut(&mut self, range: core::ops::Range<$index_type>) -> &mut [ELEMENT] {
                &mut (self.0)[range.start.get() as usize..range.end.get() as usize]
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
                $name(crate::pascal::make_copied_array!(ELEMENT; val; <$length_typenum as typenum::Unsigned>::$typenum_const as usize ))
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
                $name(crate::pascal::make_default_array!(ELEMENT;<$length_typenum as typenum::Unsigned>::$typenum_const as usize ))
            }
        }

        impl<ELEMENT: core::fmt::Debug> core::fmt::Debug for $name<ELEMENT> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let start_k = <$start_typenum as typenum::Unsigned>::$typenum_const as usize;
                let mut debug_map = f.debug_map();
                debug_map.entries((start_k..).zip(self.0.iter()));
                debug_map.finish()?;
                Ok(())
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

        impl<ELEMENT> core::ops::Index<core::ops::Range<$index_type>> for $name<ELEMENT>
        {
            type Output = [ELEMENT];
            fn index(&self, range: core::ops::Range<$index_type>) -> &[ELEMENT] {
                let start = range.start.get() as usize -
                    <$start_typenum as typenum::Unsigned>::$typenum_const as usize;
                let end = range.end.get() as usize -
                    <$start_typenum as typenum::Unsigned>::$typenum_const as usize;
                &(self.0)[start..end]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<core::ops::Range<$index_type>> for $name<ELEMENT>
        {
            fn index_mut(&mut self, range: core::ops::Range<$index_type>) -> &mut [ELEMENT] {
                let start = range.start.get() as usize -
                    <$start_typenum as typenum::Unsigned>::$typenum_const as usize;
                let end = range.end.get() as usize -
                    <$start_typenum as typenum::Unsigned>::$typenum_const as usize;
                &mut (self.0)[start..end]
            }
        }
    };
}

macro_rules! define_array_keyed_with_ranged_signed_integer_with_fixed_start_and_length {
    ($v:vis $name:ident[$index_type:path] => $base_index_type:path; $typenum_start_const:ident;
        $typenum_length_const:ident; $start_typenum:path; $length_typenum:path) => {

        $v struct $name<ELEMENT>
        ([ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_length_const as usize]);

        impl<ELEMENT> $name<ELEMENT> {
            $v fn iter(&self) -> core::slice::Iter<'_, ELEMENT> {
                (self.0)[..].iter()
            }
        }

        impl<ELEMENT> $name<ELEMENT>
        where
            ELEMENT: Copy {
            $v fn from_copied(val: ELEMENT) -> Self {
                $name(crate::pascal::make_copied_array!(ELEMENT; val; <$length_typenum as typenum::Unsigned>::$typenum_length_const as usize ))
            }
        }

        impl<ELEMENT> Clone for $name<ELEMENT>
        where
            [ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_length_const as usize]: Clone {
            fn clone(&self) -> Self {
                $name(self.0.clone())
            }
        }

        impl<ELEMENT> Copy for $name<ELEMENT>
        where
            [ELEMENT; <$length_typenum as typenum::Unsigned>::$typenum_length_const as usize]: Copy {}

        impl<ELEMENT> Default for $name<ELEMENT>
        where
            ELEMENT: Default {
            fn default() -> Self {
                $name(crate::pascal::make_default_array!(ELEMENT;<$length_typenum as typenum::Unsigned>::$typenum_length_const as usize ))
            }
        }

        impl<ELEMENT: core::fmt::Debug> core::fmt::Debug for $name<ELEMENT> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let start_k = <$start_typenum as typenum::Integer>::$typenum_start_const as usize;
                let mut debug_map = f.debug_map();
                debug_map.entries((start_k..).zip(self.0.iter()));
                debug_map.finish()?;
                Ok(())
            }
        }

        impl<ELEMENT> core::ops::Index<$index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $index_type) -> &ELEMENT {
                &(self.0)[(idx.get() - <$start_typenum as typenum::Integer>::$typenum_start_const) as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<$index_type> for $name<ELEMENT>
        {
            fn index_mut(&mut self, idx: $index_type) -> &mut ELEMENT {
                &mut (self.0)[(idx.get() - <$start_typenum as typenum::Integer>::$typenum_start_const) as usize]
            }
        }

        impl<ELEMENT> core::ops::Index<$base_index_type> for $name<ELEMENT>
        {
            type Output = ELEMENT;
            fn index(&self, idx: $base_index_type) -> &ELEMENT {
                &(self.0)[(idx as isize - <$start_typenum as typenum::Integer>::$typenum_start_const as isize) as usize]
            }
        }

        impl<ELEMENT> core::ops::IndexMut<$base_index_type> for $name<ELEMENT>
        {
            fn index_mut(&mut self, idx: $base_index_type) -> &mut ELEMENT {
                &mut (self.0)[(idx as isize - <$start_typenum as typenum::Integer>::$typenum_start_const as isize) as usize]
            }
        }
    };
}

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
                        todo!();
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
        open_text_file_for_read(path)
    }

    fn open_binary_file_for_read(path: &str) -> Result<Box<dyn Read>, usize> {
        open_binary_file_for_read(path)
    }

    fn open_file_for_write(path: &str) -> Result<Box<dyn Write>, usize> {
        open_file_for_write(path)
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
        open_text_file_for_read(path)
    }

    fn open_binary_file_for_read(path: &str) -> Result<Box<dyn Read>, usize> {
        open_binary_file_for_read(path)
    }

    fn open_file_for_write(path: &str) -> Result<Box<dyn Write>, usize> {
        open_file_for_write(path)
    }
}

pub(crate) type packed_file_of<T> = file_of<T>;

fn open_text_file_for_read(path: &str) -> Result<(Box<dyn pascal_io::ReadLine>, bool), usize> {
    let mut term_special_handling = false;
    let new_read_target: Box<dyn ReadLine> = if path == "TTY:" {
        term_special_handling = true;
        Box::new(io::stdin())
    } else if path == crate::section_0011::pool_name {
        Box::new(crate::string_pool::pool_file())
    } else {
        let path = path.trim_end_matches(' ');
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

fn open_binary_file_for_read(path: &str) -> Result<Box<dyn Read>, usize> {
    let new_read_target: Box<dyn Read> = if path == "TTY:" {
        Box::new(io::stdin())
    } else {
        let mut path = path.trim_end_matches(' ');
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

fn open_file_for_write(path: &str) -> Result<Box<dyn Write>, usize> {
    let write_target: Box<dyn Write> = if path == "TTY:" {
        Box::new(io::stdout())
    } else {
        let path = path.trim_end_matches(' ');
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

pub(crate) use pascal_io::*;

use crate::section_0019::text_char;
use core::cell::Cell;
use core::fmt::{self, Display};
use core::marker::PhantomData;
use std::io::{self, Read, Write};
use std::rc::Rc;
use typenum::Unsigned;
