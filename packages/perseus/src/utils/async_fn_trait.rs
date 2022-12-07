use futures::Future;
use std::pin::Pin;

/// A generic return type for asynchronous functions that we need to store in a
/// struct.
pub type AsyncFnReturn<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

/// Creates traits that prevent users from having to pin their functions' return
/// types. We can't make a generic one until desugared function types are stabilized (https://github.com/rust-lang/rust/issues/29625).
#[macro_export]
#[doc(hidden)]
macro_rules! make_async_trait {
    (
        $vis:vis $name:ident
        // This is capable of supporting HRTBs, though that's no longer needed. Left in for future cases.
        $(< $( $g_name:ident $( : $g_restr_1:tt $( + $g_restr_extra:tt )* $( - for<$g_lt:lifetime> $g_restr_hrtb:tt<$g_lt_1:lifetime> )* )? $(,)? )+ >)?,
        $return_ty:ty
        $(, $arg_name:ident: $arg:ty)*
    ) => {
        // These traits should be purely internal, the user is likely to shoot themselves in the foot
        #[doc(hidden)]
        pub trait $name$( <$( $g_name $( : $g_restr_1 $( + $g_restr_extra )* $( + for<$g_lt> $g_restr_hrtb<$g_lt_1> )* )?, )*> )? {
            fn call(
                &self,
                // Each given argument is repeated
                $(
                    $arg_name: $arg,
                )*
            ) -> AsyncFnReturn<$return_ty>;
        }
        impl<T, F, $($( $g_name $( : $g_restr_1 $( + $g_restr_extra )* $( + for<$g_lt> $g_restr_hrtb<$g_lt_1> )* )?, )*)?> $name$( <$( $g_name, )*> )? for T
        where
            T: Fn(
                $(
                    $arg,
                )*
            ) -> F,
            F: Future<Output = $return_ty> + Send + Sync + 'static,
        {
            fn call(
                &self,
                $(
                    $arg_name: $arg,
                )*
            ) -> AsyncFnReturn<$return_ty> {
                Box::pin(self(
                    $(
                        $arg_name,
                    )*
                ))
            }
        }
    };
}
