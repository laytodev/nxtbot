#[macro_export]
macro_rules! global {
    (
        $(
            $vs:vis $name:ident: $fty:ty = $sep:tt $var:tt$([$add:tt])?;
        )*
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            $vs static mut $name: $name = $name {
                offset: $crate::__define_offset3!($sep $var)
            };
            #[allow(non_camel_case_types)]
            $vs struct $name {
                offset: faithe::RuntimeOffset,
            }
            unsafe impl ::core::marker::Sync for $name { }
            impl $name {
                #[inline]
                $vs unsafe fn get(&self) -> $fty {
                    std::ptr::read(self.get_ref() as _)
                }

                #[inline]
                $vs unsafe fn get_ref(&self) -> &$fty {
                    if !self.offset.is_resolved() {
                        faithe::__expect!(self.offset.try_resolve("osclient.exe", faithe::__define_offset2!($($add)?)), "Failed to resolve global's address");
                    }

                    (self.offset.address() as *const $fty).as_ref().unwrap()
                }

                #[inline]
                $vs unsafe fn get_mut(&mut self) -> &mut $fty {
                    if !self.offset.is_resolved() {
                        faithe::__expect!(self.offset.try_resolve("osclient.exe", faithe::__define_offset2!($($add)?)), "Failed to resolve global's address");
                    }

                    (self.offset.address() as *mut $fty).as_mut().unwrap()
                }

                #[inline]
                $vs unsafe fn address(&self) -> usize {
                    if !self.offset.is_resolved() {
                        faithe::__expect!(self.offset.try_resolve("osclient.exe", faithe::__define_offset2!($($add)?)), "Failed to resolve global's address");
                    }
                    (self.offset.address())
                }
            }
        )*
    };
}