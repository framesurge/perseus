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
        // Because of `Into<GeneratorResult<T>>`, this is capable of supporting generics on trait bounds to two levels
        $(< $( $g_name:ident $( : $g_restr_1:tt $( < $g_restr_1_g:ident $( < $g_restr_1_g_1:ident > )? > )? $( + $g_restr_extra:tt )* )? $(,)? )+ >)?,
        $return_ty:ty
        $(, $arg_name:ident: $arg:ty)*
    ) => {
        // These traits should be purely internal, the user is likely to shoot themselves in the foot
        #[doc(hidden)]
        pub trait $name$( <$( $g_name $( : $g_restr_1 $( < $g_restr_1_g $( < $g_restr_1_g_1 > )? > )? $( + $g_restr_extra )* )?, )*> )? {
            fn call(
                &self,
                // Each given argument is repeated
                $(
                    $arg_name: $arg,
                )*
            ) -> AsyncFnReturn<$return_ty>;
        }
        impl<T, F, $($( $g_name $( : $g_restr_1 $( < $g_restr_1_g $( < $g_restr_1_g_1 > )? > )? $( + $g_restr_extra )* )?, )*)?> $name$( <$( $g_name, )*> )? for T
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
