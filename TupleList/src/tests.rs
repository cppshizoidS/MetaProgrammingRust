use tuple::{TupleList, Tuple, AsTupleOfRefs, TupleCons, NonEmptyTuple};

#[cfg(feature = "std")]
#[test]
fn all_features() {
    trait NumberOrString {
        type OtherType;

        fn into_other(self) -> Self::OtherType;
        fn format(&self) -> String;
        fn plus_one(&mut self);
    }

    impl NumberOrString for i32 {
        type OtherType = String;

        fn into_other(self) -> Self::OtherType { self.to_string() }
        fn format(&self) -> String { self.to_string() }
        fn plus_one(&mut self) { *self += 1; }
    }

    impl NumberOrString for String {
        type OtherType = i32;

        fn into_other(self) -> Self::OtherType { self.parse().unwrap() }
        fn format(&self) -> String { self.clone() }
        fn plus_one(&mut self) { self.push('1'); }
    }

    trait NumberOrStringTupleListValue: TupleList {
        type OtherType: TupleList;

        fn into_other(self) -> Self::OtherType;
    }
    trait NumberOrStringRef {
        fn format_ref(self) -> String; 
    }
    trait NumberOrStringMutRef {
        fn plus_one_ref(self); 
    }

    impl NumberOrStringTupleListValue for () {
        type OtherType = ();

        fn into_other(self) -> Self::OtherType { () }
    }
    impl NumberOrStringRef for () {
        fn format_ref(self) -> String { String::new() }
    }
    impl NumberOrStringMutRef for () {
        fn plus_one_ref(self) {}
    }

    impl<Head, Tail> NumberOrStringTupleListValue for (Head, Tail) where
        Head: NumberOrString,
        Tail: NumberOrStringTupleListValue,
        Self: TupleList,
        (Head::OtherType, Tail::OtherType): TupleList,
    {
        type OtherType = (Head::OtherType, Tail::OtherType);

        fn into_other(self) -> Self::OtherType {
            (self.0.into_other(), self.1.into_other())
        }
    }

    impl<Head, Tail> NumberOrStringRef for (&Head, Tail) where
        Head: NumberOrString,
        Tail: NumberOrStringRef + TupleList,
        Self: TupleList,
    {
        fn format_ref(self) -> String {
            format!("{} {}", self.0.format(), self.1.format_ref())
        }
    }

    impl<Head, Tail> NumberOrStringMutRef for (&mut Head, Tail) where
        Head: NumberOrString,
        Tail: NumberOrStringMutRef + TupleList,
        Self: TupleList,
    {
        fn plus_one_ref(self) {
            self.0.plus_one();
            self.1.plus_one_ref();
        }
    }

    impl<'a, T, RT> NumberOrStringRef for &'a T where
        T: AsTupleOfRefs<'a, TupleOfRefs=RT>,
        RT: Tuple + 'a,
        RT::TupleList: NumberOrStringRef + TupleList,
    {
        fn format_ref(self) -> String {
            self.as_tuple_of_refs().into_tuple_list().format_ref()
        }
    }

    impl<'a, T, RT> NumberOrStringMutRef for &'a mut T where
        T: AsTupleOfRefs<'a, TupleOfMutRefs=RT>,
        RT: Tuple + 'a,
        RT::TupleList: NumberOrStringMutRef + TupleList,
    {
        fn plus_one_ref(self) {
            self.as_tuple_of_mut_refs().into_tuple_list().plus_one_ref()
        }
    }

    impl<T> NumberOrString for T where
        T: Tuple,
        T::TupleList: NumberOrStringTupleListValue,
        for<'a> &'a T: NumberOrStringRef,
        for<'a> &'a mut T: NumberOrStringMutRef,
    {
        type OtherType = <<<T as Tuple>::TupleList as NumberOrStringTupleListValue>::OtherType as TupleList>::Tuple;

        fn into_other(self) -> Self::OtherType {
            self.into_tuple_list().into_other().into_tuple()
        }
        fn format(&self) -> String {
            self.format_ref()
        }
        fn plus_one(&mut self) {
            self.plus_one_ref()
        }
    }

    let src = (1, String::from("2"), 3, String::from("4"));
    let dst = (String::from("1"), 2, String::from("3"), 4);
    assert_eq!(
        src.into_other(),
        dst,
    );

    let src = (1, String::from("2"), 3, String::from("4"));
    assert_eq!(
        src.format(),
        "1 2 3 4 ",
    );

    let mut src = (1, String::from("2"), 3, String::from("4"));
    src.plus_one();
    assert_eq!(
        src,
        (2, String::from("21"), 4, String::from("41")),
    );
}

#[test]
#[cfg(feature = "std")]
fn value_single_trait() {
    trait NumberOrString {
        type OtherType;

        fn into_other(self) -> Self::OtherType;
    }

    impl NumberOrString for i32 {
        type OtherType = String;

        fn into_other(self) -> Self::OtherType { self.to_string() }
    }

    impl NumberOrString for String {
        type OtherType = i32;

        fn into_other(self) -> Self::OtherType { self.parse().unwrap() }
    }

    impl NumberOrString for () {
        type OtherType = ();

        fn into_other(self) -> Self::OtherType { () }
    }

    impl<Head, Tail> NumberOrString for (Head, Tail) where
        Head: NumberOrString,
        Tail: TupleList + NumberOrString,
        Self: TupleList,
        (Head::OtherType, Tail::OtherType): TupleList,
    {
        type OtherType = (Head::OtherType, Tail::OtherType);

        fn into_other(self) -> Self::OtherType {
            (self.0.into_other(), self.1.into_other())
        }
    }

    #[cfg(feature = "std")]
    fn into_other<T, OTL>(tuple: T) -> OTL::Tuple where
        T: Tuple,
        T::TupleList: NumberOrString<OtherType=OTL>,
        OTL: TupleList,
    {
        tuple.into_tuple_list().into_other().into_tuple()
    }

    let src = (1, String::from("2"), 3, String::from("4"));
    let dst = (String::from("1"), 2, String::from("3"), 4);
    assert_eq!(
        into_other(src),
        dst,
    );
}

#[test]
#[cfg(feature = "std")]
fn swap_string_and_int_dual_traits_recursion() {

    trait SwapStringAndInt     {
        type Other;
        fn swap(self) -> Self::Other;
    }
    impl SwapStringAndInt for i32 {
        type Other = String;
        fn swap(self) -> String { self.to_string() }
    }
    impl SwapStringAndInt for String {
        type Other = i32;
        fn swap(self) -> i32 { self.parse().unwrap() }
    }

    trait SwapStringAndIntTupleList {
        type Other;
        fn swap(self) -> Self::Other;
    }

    impl SwapStringAndIntTupleList for () {
        type Other = ();
        fn swap(self) -> () { () }
    }
    impl<Head, Tail> SwapStringAndIntTupleList for (Head, Tail) where
        Head: SwapStringAndInt,
        Tail: SwapStringAndIntTupleList + TupleList,
        Tail::Other: TupleList,
    {
        type Other = (Head::Other, Tail::Other);
        fn swap(self) -> Self::Other {

            (self.0.swap(), self.1.swap())
        }
    }

    impl<T, OtherTL> SwapStringAndInt for T where
        T: Tuple,
        T::TupleList: SwapStringAndIntTupleList<Other=OtherTL>,
        OtherTL: TupleList,
    {
        type Other = OtherTL::Tuple;
        fn swap(self) -> Self::Other {

            self.into_tuple_list().swap().into_tuple()
        }
    }

    let original = (4, String::from("2"), 7, String::from("13"));

    let swapped = original.swap();
    assert_eq!(
        swapped,
        (String::from("4"), 2, String::from("7"), 13),
    );

    let nested_tuple = ((1, String::from("2"), 3), 4);
    let nested_tuple_swapped = nested_tuple.swap();
    assert_eq!(
        nested_tuple_swapped,
        ((String::from("1"), 2, String::from("3")), String::from("4")),
    );
}

#[test]
#[cfg(feature = "std")]
fn swap_string_and_int_tuple() {
    trait SwapStringAndInt {
        type Other;
        fn swap(self) -> Self::Other;
    }
    impl SwapStringAndInt for i32 {
        type Other = String;
        fn swap(self) -> String { self.to_string() }
    }
    impl SwapStringAndInt for String {
        type Other = i32;
        fn swap(self) -> i32 { self.parse().unwrap() }
    }

    impl SwapStringAndInt for () {
        type Other = ();
        fn swap(self) {}
    }

    impl<Head, Tail, TailOther, T> SwapStringAndInt for T where
        T: NonEmptyTuple<Head=Head, Tail=Tail>,
        Head: SwapStringAndInt,
        Tail: Tuple + SwapStringAndInt<Other=TailOther>,
        TailOther: TupleCons<Head::Other>,
    {
        type Other = TailOther::ConsResult;
        fn swap(self) -> Self::Other {
            let (head, tail) = self.uncons();
            return TupleCons::cons(head.swap(), tail.swap());
        }
    }

    let original = (4, String::from("2"), 7, String::from("13"));

    let swapped = original.swap();
    assert_eq!(
        swapped,
        (String::from("4"), 2, String::from("7"), 13),
    );

    let nested = ((1, String::from("2")), 3);
    let nested_swapped = nested.swap();
    assert_eq!(
        nested_swapped,
        ((String::from("1"), 2), String::from("3")),
    );
}

#[test]
#[cfg(feature = "std")]
fn custom_display_tuple() {

    trait CustomDisplay {
        fn fmt(self) -> String;
    }
    impl CustomDisplay for i32  { fn fmt(self) -> String { self.to_string() } }
    impl CustomDisplay for bool { fn fmt(self) -> String { self.to_string() } }
    impl CustomDisplay for &str { fn fmt(self) -> String { self.to_string() } }

    impl CustomDisplay for () {
        fn fmt(self) -> String { String::new() }
    }

    impl<Head, Tail, T> CustomDisplay for T where
        T: NonEmptyTuple<Head=Head, Tail=Tail>,
        Head: CustomDisplay,
        Tail: CustomDisplay + Tuple,
    {
        fn fmt(self) -> String {
            let (head, tail) = self.uncons();
            return format!("{} {}", head.fmt(), tail.fmt());
        }
    }

    let tuple = (2, false, "abc");
    assert_eq!(
        tuple.fmt(),
        "2 false abc ",
    );

    let recursive_tuple = (2, false, "abc", (3, true, "def"));
    assert_eq!(
        recursive_tuple.fmt(),
        "2 false abc 3 true def  ",
    );
}

#[test]
#[cfg(feature = "std")]
fn plus_one_tuple() {
    trait PlusOne<'a> {
        fn plus_one(&'a mut self);
    }

    impl<'a> PlusOne<'a> for i32    { fn plus_one(&'a mut self) { *self += 1; } }
    impl<'a> PlusOne<'a> for bool   { fn plus_one(&'a mut self) { *self = !*self; } }
    impl<'a> PlusOne<'a> for String { fn plus_one(&'a mut self) { self.push('1'); } }

    trait PlusOneTuple: Tuple {
        fn plus_one(self);
    }

    impl PlusOneTuple for () {
        fn plus_one(self) {}
    }

    impl<'a, Head, Tail, T> PlusOneTuple for T where 
        Head: PlusOne<'a> + 'a,
        Tail: PlusOneTuple + 'a,
        T: NonEmptyTuple<Head=&'a mut Head, Tail=Tail> + 'a
    {
        fn plus_one(self) {
            let (head, tail) = self.uncons();
            head.plus_one();
            tail.plus_one();
        }
    }

    impl<'a, T, RT> PlusOne<'a> for T where
        T: NonEmptyTuple + AsTupleOfRefs<'a, TupleOfMutRefs=RT>,
        RT: PlusOneTuple + 'a,
    {
        fn plus_one(&'a mut self) {
            self.as_tuple_of_mut_refs().plus_one()
        }
    }

    let mut tuple = (2, false, String::from("abc"));
    tuple.plus_one();
    let (a, b, c) = tuple;
    assert_eq!(a, 3);
    assert_eq!(b, true);
    assert_eq!(&c, "abc1");
}

#[test]
#[cfg(feature = "std")]
fn plus_one_tuple_list_trait_with_lifetime() {

    trait PlusOne<'a> {
        fn plus_one(&'a mut self);
    }

    impl<'a> PlusOne<'a> for i32    { fn plus_one(&'a mut self) { *self += 1; } }
    impl<'a> PlusOne<'a> for bool   { fn plus_one(&'a mut self) { *self = !*self; } }
    impl<'a> PlusOne<'a> for String { fn plus_one(&'a mut self) { self.push('1'); } }

    trait PlusOneTupleList: TupleList {
        fn plus_one(self);
    }

    impl PlusOneTupleList for () {
        fn plus_one(self) {}
    }

    impl<'a, Head, Tail> PlusOneTupleList for (&'a mut Head, Tail) where 
        Head: PlusOne<'a> + 'a,
        Tail: PlusOneTupleList + 'a,
        Self: TupleList,
    {
        fn plus_one(self) {
            self.0.plus_one();
            self.1.plus_one();
        }
    }

    impl<'a, T, RT> PlusOne<'a> for T where
        T: AsTupleOfRefs<'a, TupleOfMutRefs=RT>, // tuple argument which can be converted into tuple of references
        RT: Tuple + 'a,                          // tuple of references which can be converted into tuple list
        RT::TupleList: PlusOneTupleList,         
    {
        fn plus_one(&'a mut self) {

            self.as_tuple_of_mut_refs().into_tuple_list().plus_one();
        }
    }

    let mut tuple = (2, false, String::from("abc"));
    tuple.plus_one();
    let (a, b, c) = tuple;
    assert_eq!(a, 3);
    assert_eq!(b, true);
    assert_eq!(&c, "abc1");
}

#[test]
#[cfg(feature = "std")]
fn plus_one_tuple_list_trait_without_lifetime() {

    trait PlusOne {
        fn plus_one(&mut self);
    }
    impl PlusOne for i32    { fn plus_one(&mut self) { *self += 1; } }
    impl PlusOne for bool   { fn plus_one(&mut self) { *self = !*self; } }
    impl PlusOne for String { fn plus_one(&mut self) { self.push('1'); } }

    trait PlusOneTupleList: TupleList {
        fn plus_one(self);
    }

    impl PlusOneTupleList for () {
        fn plus_one(self) {}
    }

    impl<'a, Head, Tail> PlusOneTupleList for (&'a mut Head, Tail) where
        Head: PlusOne + 'a,
        Tail: PlusOneTupleList + 'a,
        Self: TupleList,
    {
        fn plus_one(self) {
            self.0.plus_one();
            self.1.plus_one();
        }
    }

    fn plus_one<'a, T, RT>(tuple: &'a mut T) where
        T: AsTupleOfRefs<'a, TupleOfMutRefs=RT>, // tuple argument which can be converted into tuple of references
        RT: Tuple + 'a,                          // tuple of references which can be converted into tuple list
        RT::TupleList: PlusOneTupleList,         
    {

        tuple.as_tuple_of_mut_refs().into_tuple_list().plus_one();
    }

    let mut tuple = (2, false, String::from("abc"));
    plus_one(&mut tuple);
    let (a, b, c) = tuple;
    assert_eq!(a, 3);
    assert_eq!(b, true);
    assert_eq!(&c, "abc1");
}

#[test]
fn empty() {
    assert_eq!(().into_tuple_list(), ());
    assert_eq!((),                 ().into_tuple());
}

#[test]
fn single() {
    assert_eq!((false,).into_tuple_list(), (false, ()));
    assert_eq!((false,),                   (false, ()).into_tuple());
}

#[test]
fn double() {
    assert_eq!((false, 1).into_tuple_list(), (false, (1, ())));
    assert_eq!((false, 1),                   (false, (1, ())).into_tuple());
}

#[test]
fn triple() {
    assert_eq!((false, 1, "abc").into_tuple_list(), (false, (1, ("abc", ()))));
    assert_eq!((false, 1, "abc"),                   (false, (1, ("abc", ()))).into_tuple());
}

#[test]
#[cfg(feature = "std")]
fn complex_types() {
    use std::collections::HashMap;
    let t : tuple_list_type!(i32, &str, HashMap<i32, i32>) = (1, ("abc", (HashMap::new(), ())));
    let tuple_list!(a, b, c) = t;
    assert_eq!(a, 1);
    assert_eq!(b, "abc");
    assert_eq!(c, HashMap::new());
}

#[test]
fn complex_values() {
    let s = "abc";
    let t = tuple_list!(s.len(), s, 2 + 3);
    let tuple_list!(a, b, c) = t;
    assert_eq!(a, 3);
    assert_eq!(b, "abc");
    assert_eq!(c, 5);
}

#[test]
fn trailing_comma() {
    { 
        let _a = tuple_list!();
        let _b = tuple_list!(0);
        let _c = tuple_list!(0,);
        let _d = tuple_list!(0,false);
        let _e = tuple_list!(0,false,);
    }
    { 
        let _a : tuple_list_type!() = Default::default();
        let _b : tuple_list_type!(i32) = Default::default();
        let _c : tuple_list_type!(i32,) = Default::default();
        let _d : tuple_list_type!(i32,bool) = Default::default();
        let _e : tuple_list_type!(i32,bool,) = Default::default();
    }
}

#[test]
#[cfg(feature = "std")]
fn traits() {

    let list : tuple_list_type!(bool, i32, String) = tuple_list!(false, 1, String::from("abc"));
    assert_eq!(list.clone(), list); 

    fn consume(_: tuple_list_type!(i32, bool)) {}
    let copy : tuple_list_type!(i32, bool) = tuple_list!(5, false);
    consume(copy);
    consume(copy);

    assert_eq!(format!("{:?}", tuple_list!(1, false, "abc")), "(1, (false, (\"abc\", ())))");

    let default: tuple_list_type!(i32, bool, String) = Default::default();
    assert_eq!(default, tuple_list!(0, false, String::new()));

    use std::hash::Hash;
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    ().hash(&mut hasher);
    tuple_list!(false).hash(&mut hasher);
    tuple_list!(false, String::from("abc")).hash(&mut hasher);

    assert!(tuple_list!(false) < tuple_list!(true));
    assert!(tuple_list!(1, 2) < tuple_list!(2, 3));
    assert!(tuple_list!(5, 3) > tuple_list!(2, 3));
    assert_eq!(tuple_list!(String::from("foo"), false), tuple_list!(String::from("foo"), false));
    assert_ne!(tuple_list!(String::from("foo"), false), tuple_list!(String::from("foo"), true));
}

#[test]
fn tuple_list_size() {
    assert_eq!(0, <tuple_list_type!() as TupleList>::TUPLE_LIST_SIZE);
    assert_eq!(1, <tuple_list_type!(i32) as TupleList>::TUPLE_LIST_SIZE);
    #[cfg(feature = "std")]
    assert_eq!(2, <tuple_list_type!(i32, String) as TupleList>::TUPLE_LIST_SIZE);
    #[cfg(feature = "std")]
    assert_eq!(3, <tuple_list_type!(i32, String, bool) as TupleList>::TUPLE_LIST_SIZE);
}