#![allow(non_snake_case)] 
#![cfg_attr(not(feature = "std"), no_std)]

pub trait TupleList where Self: Sized {
    type Tuple: Tuple<TupleList=Self>;
    const TUPLE_LIST_SIZE: usize;
    
    fn into_tuple(self) -> Self::Tuple;
}

pub trait Tuple where Self: Sized {
    type TupleList: TupleList<Tuple=Self>;

    fn into_tuple_list(self) -> Self::TupleList;
}

pub trait AsTupleOfRefs<'a>: Tuple {
    type TupleOfRefs: Tuple + 'a;
    type TupleOfMutRefs: Tuple + 'a;

    fn as_tuple_of_refs(&'a self) -> Self::TupleOfRefs;
    fn as_tuple_of_mut_refs(&'a mut self) -> Self::TupleOfMutRefs;
}

pub trait TupleCons<Head>: Tuple {
    type ConsResult: Tuple;
    fn cons(head: Head, tail: Self) -> Self::ConsResult;
}

pub trait NonEmptyTuple: Tuple {
    type Head;
    type Tail: Tuple;

    fn uncons(self) -> (Self::Head, Self::Tail);
    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

#[macro_export]
macro_rules! tuple_list {
    () => ( () );

    ($i:ident)  => ( ($i, ()) );
    ($i:ident,) => ( ($i, ()) );
    ($i:ident, $($e:ident),*)  => ( ($i, $crate::tuple_list!($($e),*)) );
    ($i:ident, $($e:ident),*,) => ( ($i, $crate::tuple_list!($($e),*)) );

    ($i:expr)  => ( ($i, ()) );
    ($i:expr,) => ( ($i, ()) );
    ($i:expr, $($e:expr),*)  => ( ($i, $crate::tuple_list!($($e),*)) );
    ($i:expr, $($e:expr),*,) => ( ($i, $crate::tuple_list!($($e),*)) );
}

#[macro_export]
macro_rules! tuple_list_type {
    () => ( () );

    ($i:ty)  => ( ($i, ()) );
    ($i:ty,) => ( ($i, ()) );
    ($i:ty, $($e:ty),*)  => ( ($i, $crate::tuple_list_type!($($e),*)) );
    ($i:ty, $($e:ty),*,) => ( ($i, $crate::tuple_list_type!($($e),*)) );
}

macro_rules! list_head {
    ($i:ident) => ( $i );
    ($i:ident, $($e:ident),+) => ( $i );
}

macro_rules! list_tail {
    ($i:ident) => ( () );
    ($i:ident, $e:ident) => ( ($e,) );
    ($i:ident, $($e:ident),+) => ( ($($e),*,) );
}

macro_rules! define_tuple_list_traits {
    () => (
        impl TupleList for () {
            type Tuple = ();
            const TUPLE_LIST_SIZE: usize = 0;
            fn into_tuple(self) {}
        }
        impl Tuple for () {
            type TupleList = ();
            fn into_tuple_list(self) -> () { () }
        }
        impl<'a> AsTupleOfRefs<'a> for () {
            type TupleOfRefs = ();
            type TupleOfMutRefs = ();
            fn as_tuple_of_refs(&'a self) {}
            fn as_tuple_of_mut_refs(&'a mut self) {}
        }
    );
    ($($x:ident),*) => (
        impl<$($x),*> TupleList for tuple_list_type!($($x),*) {
            type Tuple = ($($x),*,);
            const TUPLE_LIST_SIZE: usize = <list_tail!($($x),*) as Tuple>::TupleList::TUPLE_LIST_SIZE + 1;
            fn into_tuple(self) -> Self::Tuple {
                let tuple_list!($($x),*) = self;
                return ($($x),*,)
            }
        }
        impl<$($x),*> Tuple for ($($x),*,) {
            type TupleList = tuple_list_type!($($x),*);
            fn into_tuple_list(self) -> Self::TupleList {
                let ($($x),*,) = self;
                return tuple_list!($($x),*);
            }
        }
        impl<'a, $($x: 'a),*> AsTupleOfRefs<'a> for ($($x),*,) {
            type TupleOfRefs = ($(&'a $x),*,);
            type TupleOfMutRefs = ($(&'a mut $x),*,);
            fn as_tuple_of_refs(&'a self) -> Self::TupleOfRefs {
                let ($($x),*,) = self;
                return ($($x),*,);
            }
            fn as_tuple_of_mut_refs(&'a mut self) -> Self::TupleOfMutRefs {
                let ($($x),*,) = self;
                return ($($x),*,);
            }
        }
        impl<$($x),*> NonEmptyTuple for ($($x),*,) {
            type Head = list_head!($($x),*);
            type Tail = list_tail!($($x),*);
            fn uncons(self) -> (Self::Head, Self::Tail) {
                let ($($x),*,) = self;
                return (list_head!($($x),*), list_tail!($($x),*));
            }
            fn head(self) -> Self::Head { self.0 }
            fn tail(self) -> Self::Tail { self.uncons().1 }
        }
        impl<$($x),*> TupleCons<list_head!($($x),*)> for list_tail!($($x),*) {
            type ConsResult = ($($x),*,);
            fn cons(head: list_head!($($x),*), tail: Self) -> Self::ConsResult {
                let list_head!($($x),*) = head;
                let list_tail!($($x),*) = tail;
                return ($($x),*,);
            }
        }
    );
}

define_tuple_list_traits!();
define_tuple_list_traits!(T1);
define_tuple_list_traits!(T1, T2);
define_tuple_list_traits!(T1, T2, T3);
define_tuple_list_traits!(T1, T2, T3, T4);
define_tuple_list_traits!(T1, T2, T3, T4, T5);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6, T7);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6, T7, T8);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
define_tuple_list_traits!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);