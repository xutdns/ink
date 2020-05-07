// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::max;
use crate::{
    env::{
        AccountId,
        Hash,
    },
    storage2::traits::{
        KeyPtr,
        PackedLayout,
        SpreadLayout,
    },
};
use ink_prelude::{
    boxed::Box,
    string::String,
};
use ink_primitives::Key;

macro_rules! impl_layout_for_primitive {
    ( $($ty:ty),* $(,)? ) => {
        $(
            impl_always_packed_layout!($ty);
            impl PackedLayout for $ty {
                #[inline(always)]
                fn pull_packed(&mut self, _at: &Key) {}
                #[inline(always)]
                fn push_packed(&self, _at: &Key) {}
                #[inline(always)]
                fn clear_packed(&self, _at: &Key) {}
            }
        )*
    };
}
#[rustfmt::skip]
impl_layout_for_primitive!(
    // We do not include `f32` and `f64` since Wasm contracts currently
    // do not support them. We might add them to this list once we add
    // support for those primitives.
    Key, Hash, AccountId,
    String,
    bool,
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
);

impl<T> SpreadLayout for Option<T>
where
    T: SpreadLayout,
{
    const FOOTPRINT: u64 = 1 + <T as SpreadLayout>::FOOTPRINT;

    fn push_spread(&self, ptr: &mut KeyPtr) {
        match self {
            Some(value) => {
                false.push_spread(ptr);
                <T as SpreadLayout>::push_spread(value, ptr)
            }
            None => {
                true.push_spread(ptr);
            }
        }
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        if let Some(value) = self {
            ptr.advance_by(1);
            <T as SpreadLayout>::clear_spread(value, ptr)
        }
    }

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        if <bool as SpreadLayout>::pull_spread(ptr) {
            return Some(<T as SpreadLayout>::pull_spread(ptr))
        }
        None
    }
}

impl<T> PackedLayout for Option<T>
where
    T: PackedLayout,
{
    #[inline]
    fn push_packed(&self, at: &Key) {
        if let Some(value) = self {
            <T as PackedLayout>::push_packed(value, at)
        }
    }

    #[inline]
    fn clear_packed(&self, at: &Key) {
        if let Some(value) = self {
            <T as PackedLayout>::clear_packed(value, at)
        }
    }

    #[inline]
    fn pull_packed(&mut self, at: &Key) {
        if let Some(value) = self {
            <T as PackedLayout>::pull_packed(value, at)
        }
    }
}

impl<T, E> SpreadLayout for Result<T, E>
where
    T: SpreadLayout,
    E: SpreadLayout,
{
    const FOOTPRINT: u64 = 1 + max(
        <T as SpreadLayout>::FOOTPRINT,
        <E as SpreadLayout>::FOOTPRINT,
    );

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        let is_ok = <bool as SpreadLayout>::pull_spread(ptr);
        if is_ok {
            Ok(<T as SpreadLayout>::pull_spread(ptr))
        } else {
            Err(<E as SpreadLayout>::pull_spread(ptr))
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        match self {
            Ok(value) => {
                true.push_spread(ptr);
                <T as SpreadLayout>::push_spread(value, ptr);
            }
            Err(error) => {
                false.push_spread(ptr);
                <E as SpreadLayout>::push_spread(error, ptr);
            }
        }
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        ptr.advance_by(1);
        match self {
            Ok(value) => {
                <T as SpreadLayout>::clear_spread(value, ptr);
            }
            Err(error) => {
                <E as SpreadLayout>::clear_spread(error, ptr);
            }
        }
    }
}

impl<T, E> PackedLayout for Result<T, E>
where
    T: PackedLayout,
    E: PackedLayout,
{
    #[inline]
    fn push_packed(&self, at: &Key) {
        match self {
            Ok(value) => <T as PackedLayout>::push_packed(value, at),
            Err(error) => <E as PackedLayout>::push_packed(error, at),
        }
    }

    #[inline]
    fn clear_packed(&self, at: &Key) {
        match self {
            Ok(value) => <T as PackedLayout>::clear_packed(value, at),
            Err(error) => <E as PackedLayout>::clear_packed(error, at),
        }
    }

    #[inline]
    fn pull_packed(&mut self, at: &Key) {
        match self {
            Ok(value) => <T as PackedLayout>::pull_packed(value, at),
            Err(error) => <E as PackedLayout>::pull_packed(error, at),
        }
    }
}

impl<T> SpreadLayout for Box<T>
where
    T: SpreadLayout,
{
    const FOOTPRINT: u64 = <T as SpreadLayout>::FOOTPRINT;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Box::new(<T as SpreadLayout>::pull_spread(ptr))
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        <T as SpreadLayout>::push_spread(&*self, ptr)
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        <T as SpreadLayout>::clear_spread(&*self, ptr)
    }
}

impl<T> PackedLayout for Box<T>
where
    T: PackedLayout,
{
    #[inline]
    fn push_packed(&self, at: &Key) {
        <T as PackedLayout>::push_packed(&*self, at)
    }

    #[inline]
    fn clear_packed(&self, at: &Key) {
        <T as PackedLayout>::clear_packed(&*self, at)
    }

    #[inline]
    fn pull_packed(&mut self, at: &Key) {
        <T as PackedLayout>::pull_packed(&mut *self, at)
    }
}