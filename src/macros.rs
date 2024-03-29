#[macro_export]
macro_rules! izip {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple , rest , of , iterators
    // eg. izip!( @closure ((a, b), c) => (a, b, c) , dd , ee )
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        $crate::izip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    // unary
    ($first:expr $(,)*) => {
        std::iter::IntoIterator::into_iter($first)
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        $crate::izip!($first)
            .zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        $crate::izip!($first)
            $(
                .zip($rest)
            )*
            .map(
                $crate::izip!(@closure a => (a) $( , $rest )*)
            )
    };
}

// https://github.com/mattgathu/cute
// Modified cute c!
#[macro_export]
macro_rules! v {
     ($e:expr $(, for $p:pat in $iter:expr)+) => {{
        let mut r = vec![];
        v!(@loop r.push($e) $(, for $p in $iter)*);
        r
    }};
     ($e:expr $(, for $i:ident in $iter:expr)+) => {{
        let mut r = vec![];
        v!(@loop r.push($e) $(, for $i in $iter)*);
        r
    }};
    (@loop $e:expr, for $i:ident in $iter:expr $(, for $j:ident in $rest:expr)*) => {
        v!(@loop for $i in $iter { $e } $(, for $j in $rest)*);
    };
    (@loop $e:expr, for $i:pat in $iter:expr $(, for $j:pat in $rest:expr)*) => {
        v!(@loop for $i in $iter { $e } $(, for $j in $rest)*);
    };

   (@loop $e:expr) => {
        $e
    };


    ($exp:expr, for $i:ident in $iter:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i in $iter {
                if $cond {
                    r.push($exp.clone());
                }
            }
            r
        }
    );


    ($exp:expr, for $i:ident in $iter:expr, for $i2:ident in $iter2:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i2 in $iter2 {
                for $i in $iter {
                    if $cond{
                        r.push($exp);
                    }
                }
            }
            r
        }
    );

    ($exp:expr, for $i:ident in $iter:expr, for $i2:ident in $iter2:expr, for $i3:ident in $iter3:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i in $iter {
                for $i2 in $iter2 {
                    for $i3 in $iter3 {
                        if $cond {
                            r.push($exp);
                        }
                    }
                }
            }
            r
        }
    );

    ////////////
    ($exp:expr, for $i:pat in $iter:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i in $iter {
                if $cond {
                    r.push($exp.clone());
                }
            }
            r
        }
    );


    ($exp:expr, for $i:pat in $iter:expr, for $i2:ident in $iter2:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i2 in $iter2 {
                for $i in $iter {
                    if $cond{
                        r.push($exp);
                    }
                }
            }
            r
        }
    );

    ($exp:expr, for $i:pat in $iter:expr, for $i2:pat in $iter2:expr, for $i3:pat in $iter3:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i in $iter {
                for $i2 in $iter2 {
                    for $i3 in $iter3 {
                        if $cond {
                            r.push($exp);
                        }
                    }
                }
            }
            r
        }
    );
    /////////////

    ($key:expr => $val:expr, for $p:pat in $iter:expr) => (
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            for $p in $iter {
                map.insert($key, $val);
            }
            map
        }
    );

    ($key:expr => $val:expr, for $p:pat in $iter:expr, if $cond:expr) => (
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            for $p in $iter {
                if $cond {
                    map.insert($key, $val);
                }
            }
            map
        }
    );

    ($key:expr => $val:expr, for $i:ident in $iter:expr) => (
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            for $i in $iter {
                map.insert($key, $val);
            }
            map
        }
    );

    ($key:expr => $val:expr, for $i:ident in $iter:expr, if $cond:expr) => (
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            for $i in $iter {
                if $cond {
                    map.insert($key, $val);
                }
            }
            map
        }
    );
}

#[macro_export]
macro_rules! view {
    ($shape: expr) => {
        View::new(&$shape.to_vec(), None, None, None)
    };
    ($shape: expr, $strides: expr) => {
        View::new(&$shape.to_vec(), Some($strides.to_vec()), None, None)
    };
    ($shape: expr, $strides: expr, $offset: expr) => {
        View::new(
            &$shape.to_vec(),
            Some($strides.to_vec()),
            Some($offset),
            None,
        )
    };
    ($shape: expr, $strides: expr, $offset: expr, $mask: expr) => {
        View::new(
            &$shape.to_vec(),
            Some($strides.to_vec()),
            Some($offset),
            $mask,
        )
    };
}

mod tests {
    use std::collections::HashMap;
    #[test]
    fn simple_comprehension() {
        let squares: Vec<usize> = v![x*x, for x in 0..10];
        assert_eq!(squares, vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
    }

    #[test]
    fn filter_comprehension() {
        let squares = v![x*x, for x in 0..10, if x % 2 == 0];
        assert_eq!(squares, vec![0, 4, 16, 36, 64]);
    }

    #[test]
    fn simple_nested_comprehension() {
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let flat: Vec<usize> = v![x, for x in y, for y in nested];
        assert_eq!(flat, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn filter_nested_comprehension() {
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let even_flat: Vec<usize> = v![x, for x in y, for y in nested, if x % 2 == 0];
        assert_eq!(even_flat, vec![2, 4, 6, 8]);
    }

    #[test]
    fn repeated_nested_comprehension() {
        let n: i32 = 10;
        let triples = v![(x,y, z), for x in 1..n+1, for y in x..n+1, for z in y..n+1, if x.pow(2) + y.pow(2) == z.pow(2)];
        println!("{:?}", triples);
        assert_eq!(triples, vec![(3, 4, 5), (6, 8, 10)]);
    }

    #[test]
    fn iter_nested_comprehension() {
        let x = v![(x, y), for x in 0..2u8, for y in vec!['a', 'b']];
        assert_eq!(x, vec![(0, 'a'), (1, 'a'), (0, 'b'), (1, 'b')]);
    }

    #[test]
    fn vector_to_iter_comprehension() {
        let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
        let output: Vec<i32> = v![x*2, for x in vec.iter()];
        assert_eq!(output, vec![-8, -4, 0, 4, 8]);
    }

    #[test]
    fn filter_comprehension_two() {
        let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
        let output: Vec<i32> = v![x, for x in vec.iter(), if *x >= 0i32];
        assert_eq!(output, vec![0, 2, 4]);
    }

    #[test]
    fn apply_function_comprehension() {
        let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
        let output: Vec<i32> = v![x.abs(), for x in vec.iter()];
        assert_eq!(output, vec![4, 2, 0, 2, 4]);
    }

    #[test]
    fn apply_user_defined_function() {
        fn square(x: i32) -> i32 {
            x * x
        }

        let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
        let squares: Vec<i32> = v![square(x), for x in vec];
        assert_eq!(squares, vec![16, 4, 0, 4, 16]);
    }

    #[test]
    fn hashmap_comprehension() {
        let v = vec!["one", "two", "three"];
        let map = v! {key => key.to_uppercase(), for key in v};
        let mut expected: HashMap<&str, String> = HashMap::new();
        expected.insert("one", String::from("ONE"));
        expected.insert("two", String::from("TWO"));
        expected.insert("three", String::from("THREE"));

        assert_eq!(map, expected);
    }

    #[test]
    fn hashmap_comprehension_two() {
        let v = vec!["one", "two", "three"];
        let map = v! {format!("{}-true", key) => key.to_uppercase(), for key in v};
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(String::from("one-true"), String::from("ONE"));
        expected.insert(String::from("two-true"), String::from("TWO"));
        expected.insert(String::from("three-true"), String::from("THREE"));

        assert_eq!(map, expected);
    }

    #[test]
    fn hashmap_comprehension_three() {
        let v: Vec<(String, i32)> = vec![
            (String::from("one"), 1),
            (String::from("two"), 2),
            (String::from("three"), 3),
        ];
        let map = v! {key => val, for (key, val) in v};

        let mut expected: HashMap<String, i32> = HashMap::new();
        expected.insert(String::from("one"), 1);
        expected.insert(String::from("two"), 2);
        expected.insert(String::from("three"), 3);

        assert_eq!(map, expected);
    }

    #[test]
    fn hashmap_tuple_comprehension() {
        let v: Vec<(&str, i32)> = vec![("one", 1), ("two", 2), ("three", 3)];
        let map = v! {key => val, for (key, val) in v};

        let mut expected: HashMap<&str, i32> = HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);
        expected.insert("three", 3);

        assert_eq!(map, expected);
    }

    #[test]
    fn conditional_hashmap_tuple_comprehension() {
        let v: Vec<(&str, i32)> = vec![("one", 1), ("two", 2), ("three", 3)];
        let map = v! {key => val, for (key, val) in v, if val == 1 || val == 2};

        let mut expected: HashMap<&str, i32> = HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);

        assert_eq!(map, expected);
    }

    #[test]
    fn hashmap_from_iter() {
        let map = v! {*key => key*key, for key in vec![1,2].iter()};
        let mut e: HashMap<i32, i32> = HashMap::new();
        e.insert(1, 1);
        e.insert(2, 4);

        assert_eq!(map, e);
    }

    #[test]
    fn conditional_hashmap_from_iter() {
        let map = v! {*key => key*key, for key in vec![1,2].iter(), if *key % 2 == 0};
        let mut e: HashMap<i32, i32> = HashMap::new();
        e.insert(2, 4);

        assert_eq!(map, e);
    }

    #[test]
    fn hashmap_from_range() {
        let map = v! {key => key*key, for key in 1..3};
        let mut e: HashMap<i32, i32> = HashMap::new();
        e.insert(1, 1);
        e.insert(2, 4);

        assert_eq!(map, e);
    }

    #[test]
    fn conditional_hashmap_from_range() {
        let map = v! {key => key*key, for key in 1..6, if key % 2 == 1};
        let mut e: HashMap<i32, i32> = HashMap::new();
        e.insert(1, 1);
        e.insert(3, 9);
        e.insert(5, 25);

        assert_eq!(map, e);
    }
}

#[macro_export]
macro_rules! approx_eq {
    ($Lhs:expr, $Rhs:expr) => {{
        let lhs = $Lhs.to_vec();
        let rhs = $Rhs.map(|x| num_traits::FromPrimitive::from_f64(x).unwrap());
        lhs.iter().zip(rhs.iter()).for_each(|(l, r)| {
            assert!(
                float_cmp::approx_eq!(f32, *l, *r, ulps = 2),
                "{l} != {r}\n{:?}",
                $Lhs
            )
        });
    }};
    ($Lhs:expr, $Rhs:expr, $Tolerance:expr) => {{
        let lhs = $Lhs.to_vec();
        let rhs = $Rhs.map(|x| num_traits::FromPrimitive::from_f64(x).unwrap());
        lhs.iter().zip(rhs.iter()).for_each(|(l, r)| {
            assert!(
                float_cmp::approx_eq!(f32, *l, *r, ulps = $Tolerance),
                "{l} != {r}\n{:?}",
                $Lhs
            )
        });
    }};
}

pub use super::approx_eq;
pub use super::izip;
pub use super::v;
pub use super::view;
