#[macro_export]
macro_rules! function {
    (
        $(
            $vs:vis $name:ident: $(extern $($cc:literal)?)? fn($($arg_id:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? = $sep:tt $var:tt$([$add:tt])?;
        )*
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            $vs static $name: $name = $name {
                offset: $crate::__define_offset3!($sep $var)
            };
            #[allow(non_camel_case_types)]
            $vs struct $name {
                offset: faithe::RuntimeOffset,
            }
            unsafe impl ::core::marker::Sync for $name { }
            impl $name {
                #[inline]
                $vs fn call(&self, $($arg_id:$arg_ty),*) $(-> $ret_ty)? {
                    unsafe {
                        if !self.offset.is_resolved() {
                            faithe::__expect!(self.offset.try_resolve("osclient.exe", faithe::__define_offset2!($($add)?)), "Failed to resolve function's address");
                        }
                        ::core::mem::transmute::<_, $(extern $($cc)?)? fn($($arg_ty),*) $(-> $ret_ty)?>(self.offset.address())($($arg_id),*)
                    }
                }

                #[inline]
                $vs fn address(&self) -> usize {
                    unsafe {
                        if !self.offset.is_resolved() {
                            faithe::__expect!(self.offset.try_resolve("osclient.exe", faithe::__define_offset2!($($add)?)), "Failed to resolve function's address");
                        }
                        self.offset.address()
                    }
                }
            }
        )*
    };
}