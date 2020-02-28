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

use crate::{
    storage,
    storage::{
        KeyPtr,
        PullForward,
        PushForward,
        StorageSize,
    },
};

impl<T> core::ops::Index<u32> for storage::Vec2<T>
where
    T: StorageSize + PullForward,
{
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        self.get(index)
            .expect("index out of bounds")
    }
}

impl<T> core::ops::IndexMut<u32> for storage::Vec2<T>
where
    T: StorageSize + PullForward,
{
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        self.get_mut(index)
            .expect("index out of bounds")
    }
}

impl<T> StorageSize for storage::Vec2<T>
where
    T: StorageSize,
{
    const SIZE: u64 =
        <u32 as StorageSize>::SIZE + <storage::LazyChunk<T> as StorageSize>::SIZE;
}

impl<T> PullForward for storage::Vec2<T>
where
    T: StorageSize,
{
    fn pull_forward(ptr: &mut KeyPtr) -> Self {
        Self {
            len: PullForward::pull_forward(ptr),
            elems: PullForward::pull_forward(ptr),
        }
    }
}

impl<T> PushForward for storage::Vec2<T>
where
    storage::LazyChunk<T>: PushForward,
{
    fn push_forward(&self, ptr: &mut KeyPtr) {
        PushForward::push_forward(&self.len(), ptr);
        PushForward::push_forward(&self.elems, ptr);
    }
}