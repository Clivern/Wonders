// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

pub fn first<T>(array: &[T]) -> Option<&T> {
    array.get(0)
}

pub fn last<T>(array: &[T]) -> Option<&T> {
    array.last()
}

fn each<T, F>(array: &[T], mut f: F)
where
    F: FnMut(&T),
{
    for item in array.iter() {
        f(item);
    }
}

#[test]
fn test_first() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 0] = [];
    let arr3 = ["a", "b", "c", "d", "e", "f"];

    assert_eq!(first(&arr1), Some(&1));
    assert_eq!(first(&arr2), None);
    assert_eq!(first(&arr3), Some(&"a"));
}

#[test]
fn test_last() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 0] = [];
    let arr3 = ["a", "b", "c", "d", "e", "f"];

    assert_eq!(last(&arr1), Some(&5));
    assert_eq!(last(&arr2), None);
    assert_eq!(last(&arr3), Some(&"f"));
}

