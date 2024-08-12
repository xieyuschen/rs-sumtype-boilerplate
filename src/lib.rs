use paste::paste;
use strum_macros::Display;

/// ### create_sumtype
///
/// create_sumtype constructs a sumtype with the name as the first argument passes,
/// which holds the following types as sumtype field.
/// The type constructor is named with the pattern of appending sumtype name and type name.
/// For example,
/// ```ignore
/// use rs_sumtype_boilerplate::create_sumtype;
/// struct A;
/// struct B;
/// create_sumtype!(MySum, A, B);
/// ```
/// will produce the following structure:
/// ```ignore
/// enum MySum {
///     MySumA(A),
///     MySumB(B),
/// }
/// ```
// todo: think about how to support the custom display
#[macro_export]
macro_rules! create_sumtype {
    ($enum_name:ident, $($types: ident),*) => {
        paste! {
            #[derive(Display)]
            enum $enum_name {
                $(
                    [<$enum_name $types>]($types),
                )*
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pattern_match() {
        struct A;
        struct B;
        create_sumtype!(MySum, A, B);

        let sum_a: MySum = MySum::MySumA(A {});
        let mut sum_b: MySum = MySum::MySumB(B {});

        match sum_a {
            MySum::MySumA(_) => (),
            _ => assert!(false, "shouldn't reach"),
        };
        match sum_b {
            MySum::MySumB(_) => (), // Handles MySumB and returns
            _ => assert!(false, "shouldn't reach"),
        };
        sum_b = sum_a;
        match sum_b {
            MySum::MySumA(_) => (),
            _ => assert!(false, "shouldn't reach"),
        };
    }
    #[test]
    fn test_empty_sum_type() {
        // todo: is it possible to remove the comma here?
        create_sumtype!(EmptySum,);
    }
}
