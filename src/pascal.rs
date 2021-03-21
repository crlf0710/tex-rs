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

macro_rules! impl_is_even_or_odd_for_primitive {
    ($t:ty) => {
        impl crate::pascal::IsOddOrEven for $t {
            fn is_odd(&self) -> crate::pascal::boolean {
                self % 2 != 0
            }
        }
    };
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
                $name(make_copied_array!(ELEMENT; val; <$length_typenum as typenum::Unsigned>::$typenum_length_const as usize ))
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
                $name(make_default_array!(ELEMENT;<$length_typenum as typenum::Unsigned>::$typenum_length_const as usize ))
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

impl<T: AsRef<[u8]>> ReadLine for io::Cursor<T> {
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
        bytes_block_buffer: Box<[u8]>,
        bytes_avail_length: usize,
        bytes_position: usize,
        bytes_buffer: Option<T>,
    },
    Eof,
}

pub(crate) enum FileState<T> {
    Undefined,
    GenerationMode {
        write_buffer: Option<T>,
        write_target: Box<dyn Write>,
    },
    LineInspectionMode {
        read_line_buffer: LineBufferState<T>,
        read_target: Box<dyn ReadLine>,
        read_flag_extra_eoln_line: bool,
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
    fn discard_buffer_variable_value_and_get_write_target(&mut self) -> &mut dyn Write {
        match self {
            FileState::GenerationMode {
                write_buffer,
                write_target,
            } => {
                *write_buffer = None;
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
                ..
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
            FileState::BlockInspectionMode {
                read_block_buffer,
                read_target,
            } => {
                const IDEAL_BUFSIZE: usize = 512;
                let size_of_t = core::mem::size_of::<T>();
                assert!(size_of_t > 0);
                if matches!(read_block_buffer, BlockBufferState::UnknownState) {
                    let dest_size = ((IDEAL_BUFSIZE / size_of_t) + 1) * size_of_t;
                    *read_block_buffer = BlockBufferState::AfterReadBlock {
                        bytes_block_buffer: vec![0u8; dest_size].into_boxed_slice(),
                        bytes_avail_length: dest_size,
                        bytes_position: dest_size - size_of_t,
                        bytes_buffer: None,
                    }
                }
                match read_block_buffer {
                    BlockBufferState::AfterReadBlock {
                        bytes_block_buffer,
                        bytes_avail_length,
                        bytes_position,
                        bytes_buffer,
                    } => {
                        let bytes_position_end = *bytes_position + size_of_t;
                        let mut remaining_range = bytes_position_end..*bytes_avail_length;
                        if remaining_range.start > 0 {
                            if remaining_range.len() > 0 {
                                bytes_block_buffer.copy_within(remaining_range.clone(), 0);
                                remaining_range = 0..remaining_range.len();
                            } else {
                                remaining_range = 0..0;
                            }
                        }
                        *bytes_avail_length = remaining_range.end;
                        *bytes_position = 0;
                        *bytes_buffer = None;
                        while *bytes_avail_length < size_of_t {
                            let fillable_range = *bytes_avail_length..bytes_block_buffer.len();
                            let newly_read_len = read_target
                                .read(&mut bytes_block_buffer[fillable_range])
                                .expect("read block failure");
                            if newly_read_len == 0 {
                                *read_block_buffer = BlockBufferState::Eof;
                                return;
                            }
                            *bytes_avail_length += newly_read_len;
                        }
                    }
                    _ => unreachable!(),
                }
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

    fn convert_blob_to_unit(input: &[u8]) -> Self::Unit;
    
    fn convert_unit_to_blob(data: Self::Unit, f: &mut dyn for<'a> FnMut(&'a [u8]));

    fn file_state(&self) -> &FileState<Self::Unit>;

    fn file_state_mut(&mut self) -> &mut FileState<Self::Unit>;

    fn error_state(&self) -> usize;

    fn set_error_state(&mut self, error_state: usize);
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

impl_debug_with_literal!(file_of_text_char, "file_of_text_char");

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
}

pub(crate) type packed_file_of_text_char = file_of_text_char;

pub(crate) struct file_of<T> {
    file_state: FileState<T>,
    error_state: usize,
}

impl_debug_with_literal!(file_of[T], "file_of<T>");

impl<T> Default for file_of<T> {
    fn default() -> Self {
        file_of {
            file_state: FileState::default(),
            error_state: usize::default(),
        }
    }
}

pub(crate) trait FromBlob {
    fn from_blob(data: &[u8]) -> Self;
}
pub(crate) trait IntoBlob {
    type BlobType: core::borrow::Borrow<[u8]>;

    fn into_blob(&self) -> Self::BlobType;
}


impl<T: FromBlob + IntoBlob> PascalFile for file_of<T> {
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
        let blob = v.into_blob();
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
}

pub(crate) type packed_file_of<T> = file_of<T>;

#[allow(unused_variables)]
pub(crate) fn rewrite<F: PascalFile, P: Into<String>>(file: &mut F, path: P, options: &str) {
    let path = path.into();
    let new_write_target: Box<dyn Write> = if path == "TTY:" {
        Box::new(io::stdout())
    } else {
        let path = path.trim_end_matches(' ');
        let file = match std::fs::File::create(path) {
            Ok(f) => f,
            Err(e) => {
                *file.file_state_mut() = FileState::Undefined;
                file.set_error_state(1);
                return;
            }
        };
        Box::new(file)
    };
    *file.file_state_mut() = FileState::GenerationMode {
        write_target: new_write_target,
        write_buffer: None,
    };
}

pub(crate) fn buffer_variable_assign<F: PascalFile>(file: &mut F, value: F::Unit) {
    match file.file_state_mut() {
        FileState::GenerationMode {
            write_buffer,
            ..
        } => {
            *write_buffer = Some(value);
        }
        _ => {
            panic!("file not in generation mode!");
        }
    }
}

pub(crate) fn put<F: PascalFile>(file: &mut F) {
    match file.file_state_mut() {
        FileState::GenerationMode {
            write_target,
            write_buffer
        } => {
            let caret_value = write_buffer
                .take()
                .expect("file buffer variable value is undefined!");
            F::convert_unit_to_blob(caret_value, &mut |data| {
                write_target.write_all(data).expect("fail to write data");
            });
        }
        _ => {
            panic!("file not in generation mode!");
        }
    }
}

#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn reset<F: PascalFile + fmt::Debug, P: Into<String> + fmt::Debug>(
    file: &mut F,
    path: P,
    options: &str,
) {
    let path = path.into();
    if F::is_text_file() {
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
                Err(e) => {
                    *file.file_state_mut() = FileState::Undefined;
                    file.set_error_state(1);
                    return;
                }
            };
            Box::new(io::BufReader::new(file))
        };

        if term_special_handling {
            *file.file_state_mut() = FileState::LineInspectionMode {
                read_target: new_read_target,
                read_line_buffer: LineBufferState::AfterReadLine {
                    line_buffer: vec![F::eoln_unit()],
                    line_position: 0,
                    line_no_more: false,
                },
                read_flag_extra_eoln_line: true,
            };
        } else {
            *file.file_state_mut() = FileState::LineInspectionMode {
                read_target: new_read_target,
                read_line_buffer: LineBufferState::UnknownState { initial_line: true },
                read_flag_extra_eoln_line: false,
            };
        }
        file.set_error_state(0);
    } else {
        let new_read_target: Box<dyn Read> = if path == "TTY:" {
            Box::new(io::stdin())
        } else {
            let mut path = path.trim_end_matches(' ');
            path = path.trim_start_matches("TeXfonts:");
            path = path.trim_start_matches("TeXformats:");
            let file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    *file.file_state_mut() = FileState::Undefined;
                    file.set_error_state(1);
                    return;
                }
            };
            Box::new(io::BufReader::new(file))
        };
        *file.file_state_mut() = FileState::BlockInspectionMode {
            read_target: new_read_target,
            read_block_buffer: BlockBufferState::UnknownState,
        };
        file.set_error_state(0);
    }
}

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
                    return;
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
            FileState::BlockInspectionMode {
                read_block_buffer, ..
            } => match read_block_buffer {
                BlockBufferState::Eof => {
                    panic!("file eof reached");
                }
                BlockBufferState::UnknownState => {
                    file.file_state_mut().refill::<F>();
                    return;
                }
                BlockBufferState::AfterReadBlock {
                    bytes_avail_length,
                    bytes_position,
                    bytes_buffer,
                    ..
                } => {
                    let size_of_t = core::mem::size_of::<F::Unit>();
                    assert!(size_of_t > 0);
                    let bytes_position_end = *bytes_position + size_of_t;
                    let new_bytes_position_end = bytes_position_end + size_of_t;
                    if new_bytes_position_end > *bytes_avail_length {
                        file.file_state_mut().refill::<F>();
                        return;
                    }
                    *bytes_buffer = None;
                    *bytes_position = bytes_position_end;
                }
            },
            _ => {
                panic!("file not in inspection mode");
            }
        }
        return;
    }
}

pub(crate) fn buffer_variable<F: PascalFile>(file: &mut F) -> F::Unit
where
    F::Unit: Clone,
{
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
                    ..
                } => {
                    return line_buffer[*line_position].clone();
                }
            },
            FileState::BlockInspectionMode {
                read_block_buffer, ..
            } => match read_block_buffer {
                BlockBufferState::Eof => {
                    panic!("file eof reached");
                }
                BlockBufferState::UnknownState => {
                    file.file_state_mut().refill::<F>();
                    continue;
                }
                BlockBufferState::AfterReadBlock {
                    bytes_block_buffer,
                    bytes_buffer,
                    bytes_position,
                    ..
                } => match bytes_buffer {
                    None => {
                        let size_of_t = core::mem::size_of::<F::Unit>();
                        assert!(size_of_t > 0);
                        let bytes_position_end = *bytes_position + size_of_t;
                        let v = F::convert_blob_to_unit(
                            &bytes_block_buffer[*bytes_position..bytes_position_end],
                        );
                        *bytes_buffer = Some(v.clone());
                        return v;
                    }
                    Some(v) => {
                        return v.clone();
                    }
                },
            },
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
            FileState::BlockInspectionMode {
                read_block_buffer, ..
            } => match read_block_buffer {
                BlockBufferState::Eof => {
                    return true;
                }
                BlockBufferState::UnknownState { .. } => {
                    file.file_state_mut().refill::<F>();
                    continue;
                }
                BlockBufferState::AfterReadBlock { .. } => {
                    return false;
                }
            },
            FileState::GenerationMode { .. } => {
                return true;
            }
            _ => panic!("file not in any mode"),
        }
    }
}

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
    let write_target = file
        .file_state_mut()
        .discard_buffer_variable_value_and_get_write_target();
    write!(write_target, "{}", val).unwrap();
}

pub(crate) fn write_ln<F: PascalFile, T: Display>(file: &mut F, val: T) {
    let write_target = file
        .file_state_mut()
        .discard_buffer_variable_value_and_get_write_target();
    writeln!(write_target, "{}", val).unwrap();
}

pub(crate) fn write_ln_noargs<F: PascalFile>(file: &mut F) {
    let write_target = file
        .file_state_mut()
        .discard_buffer_variable_value_and_get_write_target();
    writeln!(write_target, "").unwrap();
}

pub(crate) fn write_binary<F: PascalFile, T: IntoBlob>(file: &mut F, val: T) {
    use core::borrow::Borrow;
    let write_target = file
        .file_state_mut()
        .discard_buffer_variable_value_and_get_write_target();
    let blob = val.into_blob();
    write_target.write_all(blob.borrow()).unwrap();
}

pub(crate) fn r#break<F: PascalFile>(file: &mut F) {
    let write_target = file
        .file_state_mut()
        .discard_buffer_variable_value_and_get_write_target();
    write_target.flush().unwrap();
}

pub(crate) fn read_onearg<F: PascalFile>(file: &mut F) -> F::Unit
where
    F::Unit: Copy,
{
    let v = buffer_variable(file);
    get(file);
    v
}

pub(crate) fn read_ln<F: PascalFile>(file: &mut F) {
    while !eoln(file) {
        get(file);
    }
    get(file);
}

pub(crate) fn break_in<F: PascalFile>(file: &mut F, _: boolean) {
    // FIXME: this seems nonstandard. Verify if this handling is correct.
    // and not sure what the 2nd argument should do here.
    let file_state = file.file_state_mut();
    match file_state {
        FileState::LineInspectionMode {
            read_line_buffer,
            read_flag_extra_eoln_line,
            ..
        } => match read_line_buffer {
            LineBufferState::AfterReadLine { line_no_more, .. } => {
                if *line_no_more {
                    *read_line_buffer = LineBufferState::Eof
                } else {
                    if *read_flag_extra_eoln_line {
                        *read_line_buffer = LineBufferState::AfterReadLine {
                            line_buffer: vec![F::eoln_unit()],
                            line_position: 0,
                            line_no_more: false,
                        };
                    } else {
                        *read_line_buffer = LineBufferState::UnknownState {
                            initial_line: false,
                        };
                    }
                }
            }
            _ => {}
        },
        _ => panic!("file is not in line-inspection mode"),
    }
}

#[allow(unused_variables)]
pub(crate) fn erstat<F: PascalFile>(file: &mut F) -> usize {
    file.error_state()
}

pub(crate) fn close<F: PascalFile>(file: &mut F) {
    *file.file_state_mut() = FileState::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    fn ASCII_lit_char(c: u8) -> char {
        let val: u8 = c;
        char::new(val as _)
    }

    #[test]
    fn file_input_0001() {
        use std::io::Cursor;
        const DATA: &'static str = "A\nBC\nA";
        let input_data = Cursor::new(DATA);
        let mut input_file = file_of_text_char::default();
        *input_file.file_state_mut() = FileState::LineInspectionMode {
            read_target: Box::new(input_data),
            read_line_buffer: LineBufferState::UnknownState { initial_line: true },
        };
        input_file.set_error_state(0);
        assert_eq!(false, eoln(&mut input_file));
        assert_eq!(ASCII_lit_char(b'A'), buffer_variable(&mut input_file));
        get(&mut input_file);
        assert_eq!(true, eoln(&mut input_file));
        assert_eq!(ASCII_lit_char(b'\n'), buffer_variable(&mut input_file));
        get(&mut input_file);
        assert_eq!(ASCII_lit_char(b'B'), buffer_variable(&mut input_file));
        get(&mut input_file);
        assert_eq!(ASCII_lit_char(b'C'), buffer_variable(&mut input_file));
        get(&mut input_file);
        assert_eq!(true, eoln(&mut input_file));
        assert_eq!(ASCII_lit_char(b'\n'), buffer_variable(&mut input_file));
        get(&mut input_file);
        assert_eq!(false, eoln(&mut input_file));
        assert_eq!(ASCII_lit_char(b'A'), buffer_variable(&mut input_file));
        get(&mut input_file);
        assert_eq!(true, eoln(&mut input_file));
        assert_eq!(ASCII_lit_char(b'\n'), buffer_variable(&mut input_file));
        assert_eq!(false, eof(&mut input_file));
        get(&mut input_file);
        assert_eq!(true, eof(&mut input_file));
    }
}

use crate::section_0019::text_char;
use core::cell::Cell;
use core::fmt::{self, Display};
use core::marker::PhantomData;
use std::io::{self, Read, Write};
use std::rc::Rc;
use typenum::Unsigned;
