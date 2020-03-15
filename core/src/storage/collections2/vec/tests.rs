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

use super::Vec as StorageVec;

#[test]
fn new_vec_works() {
    // `StorageVec::new`
    let vec = <StorageVec<i32>>::new();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.get(0), None);
    assert!(vec.iter().next().is_none());
    // `StorageVec::default`
    let default = <StorageVec<i32> as Default>::default();
    assert!(default.is_empty());
    assert_eq!(default.len(), 0);
    assert_eq!(vec.get(0), None);
    assert!(default.iter().next().is_none());
    // `StorageVec::new` and `StorageVec::default` should be equal.
    assert_eq!(vec, default);
}

#[test]
fn from_iterator_works() {
    let some_primes = [1, 2, 3, 5, 7, 11, 13];
    assert_eq!(some_primes.iter().copied().collect::<StorageVec<_>>(), {
        let mut vec = StorageVec::new();
        for prime in &some_primes {
            vec.push(*prime)
        }
        vec
    });
}

#[test]
fn from_empty_iterator_works() {
    assert_eq!(
        [].iter().copied().collect::<StorageVec<i32>>(),
        StorageVec::new(),
    );
}

#[test]
fn push_pop_first_last_works() {
    /// Asserts conditions are met for the given storage vector.
    fn assert_vec<F, L>(vec: &StorageVec<u8>, len: u32, first: F, last: L)
    where
        F: Into<Option<u8>>,
        L: Into<Option<u8>>,
    {
        assert_eq!(vec.is_empty(), len == 0);
        assert_eq!(vec.len(), len);
        assert_eq!(vec.first().copied(), first.into());
        assert_eq!(vec.last().copied(), last.into());
    }

    let mut vec = StorageVec::new();
    assert_vec(&vec, 0, None, None);

    // Sequence of `push`
    vec.push(b'a');
    assert_vec(&vec, 1, b'a', b'a');
    vec.push(b'b');
    assert_vec(&vec, 2, b'a', b'b');
    vec.push(b'c');
    assert_vec(&vec, 3, b'a', b'c');
    vec.push(b'd');
    assert_vec(&vec, 4, b'a', b'd');

    // Sequence of `pop`
    assert_eq!(vec.pop(), Some(b'd'));
    assert_vec(&vec, 3, b'a', b'c');
    assert_eq!(vec.pop(), Some(b'c'));
    assert_vec(&vec, 2, b'a', b'b');
    assert_eq!(vec.pop(), Some(b'b'));
    assert_vec(&vec, 1, b'a', b'a');
    assert_eq!(vec.pop(), Some(b'a'));
    assert_vec(&vec, 0, None, None);

    // Pop from empty vector.
    assert_eq!(vec.pop(), None);
    assert_vec(&vec, 0, None, None);
}

#[test]
fn pop_drop_works() {
    let elems = [b'a', b'b', b'c', b'd'];
    let mut vec = vec_from_slice(&elems);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &elems[0..3]);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &elems[0..2]);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &elems[0..1]);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &[]);
    assert_eq!(vec.pop_drop(), None);
    assert_eq_slice(&vec, &[]);
}

#[test]
fn get_works() {
    let elems = [b'a', b'b', b'c', b'd'];
    let mut vec = vec_from_slice(&elems);
    for (n, mut expected) in elems.iter().copied().enumerate() {
        let n = n as u32;
        assert_eq!(vec.get(n), Some(&expected));
        assert_eq!(vec.get_mut(n), Some(&mut expected));
    }
    let len = vec.len();
    assert_eq!(vec.get(len), None);
    assert_eq!(vec.get_mut(len), None);
}

#[test]
fn iter_next_works() {
    let elems = [b'a', b'b', b'c', b'd'];
    let vec = vec_from_slice(&elems);
    let mut iter = vec.iter();
    assert_eq!(iter.next(), Some(&b'a'));
    assert_eq!(iter.next(), Some(&b'b'));
    assert_eq!(iter.next(), Some(&b'c'));
    assert_eq!(iter.next(), Some(&b'd'));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_next_back_works() {
    let elems = [b'a', b'b', b'c', b'd'];
    let vec = vec_from_slice(&elems);
    let mut iter = vec.iter().rev();
    assert_eq!(iter.next(), Some(&b'd'));
    assert_eq!(iter.next(), Some(&b'c'));
    assert_eq!(iter.next(), Some(&b'b'));
    assert_eq!(iter.next(), Some(&b'a'));
    assert_eq!(iter.next(), None);
}

/// Asserts that the the given ordered storage vector elements are equal to the
/// ordered elements of the given slice.
fn assert_eq_slice(vec: &StorageVec<u8>, slice: &[u8]) {
    assert_eq!(vec.len() as usize, slice.len());
    assert!(vec.iter().zip(slice.iter()).all(|(lhs, rhs)| *lhs == *rhs))
}

/// Creates a storage vector from the given slice.
fn vec_from_slice(slice: &[u8]) -> StorageVec<u8> {
    slice.iter().copied().collect::<StorageVec<u8>>()
}

#[test]
fn swap_works() {
    let elems = [b'a', b'b', b'c', b'd'];
    let mut vec = vec_from_slice(&elems);

    // Swap at same position is a no-op.
    for index in 0..elems.len() as u32 {
        vec.swap(index, index);
        assert_eq_slice(&vec, &elems);
    }

    // Swap first and second
    vec.swap(0, 1);
    assert_eq_slice(&vec, &[b'b', b'a', b'c', b'd']);
    // Swap third and last
    vec.swap(2, 3);
    assert_eq_slice(&vec, &[b'b', b'a', b'd', b'c']);
    // Swap first and last
    vec.swap(0, 3);
    assert_eq_slice(&vec, &[b'c', b'a', b'd', b'b']);
}

#[test]
#[should_panic]
fn swap_one_invalid_index() {
    let mut vec = vec_from_slice(&[b'a', b'b', b'c', b'd']);
    vec.swap(0, vec.len());
}

#[test]
#[should_panic]
fn swap_both_invalid_indices() {
    let mut vec = vec_from_slice(&[b'a', b'b', b'c', b'd']);
    vec.swap(vec.len(), vec.len());
}

#[test]
fn swap_remove_works() {
    let mut vec = vec_from_slice(&[b'a', b'b', b'c', b'd']);

    // Swap remove first element.
    assert_eq!(vec.swap_remove(0), Some(b'a'));
    assert_eq_slice(&vec, &[b'd', b'b', b'c']);
    // Swap remove middle element.
    assert_eq!(vec.swap_remove(1), Some(b'b'));
    assert_eq_slice(&vec, &[b'd', b'c']);
    // Swap remove last element.
    assert_eq!(vec.swap_remove(1), Some(b'c'));
    assert_eq_slice(&vec, &[b'd']);
    // Swap remove only element.
    assert_eq!(vec.swap_remove(0), Some(b'd'));
    assert_eq_slice(&vec, &[]);
    // Swap remove from empty vector.
    assert_eq!(vec.swap_remove(0), None);
    assert_eq_slice(&vec, &[]);
}

#[test]
fn swap_remove_drop_works() {
    let mut vec = vec_from_slice(&[b'a', b'b', b'c', b'd']);

    // Swap remove first element.
    assert_eq!(vec.swap_remove_drop(0), Some(()));
    assert_eq_slice(&vec, &[b'd', b'b', b'c']);
    // Swap remove middle element.
    assert_eq!(vec.swap_remove_drop(1), Some(()));
    assert_eq_slice(&vec, &[b'd', b'c']);
    // Swap remove last element.
    assert_eq!(vec.swap_remove_drop(1), Some(()));
    assert_eq_slice(&vec, &[b'd']);
    // Swap remove only element.
    assert_eq!(vec.swap_remove_drop(0), Some(()));
    assert_eq_slice(&vec, &[]);
    // Swap remove from empty vector.
    assert_eq!(vec.swap_remove_drop(0), None);
    assert_eq_slice(&vec, &[]);
}
