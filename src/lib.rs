/// An marker for FFI safe object with objective-c
pub unsafe trait ObjcObject {
    fn as_id(&self) -> &objc::runtime::Object;
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object;
}
unsafe impl ObjcObject for objc::runtime::Object {
    #[inline(always)]
    fn as_id(&self) -> &objc::runtime::Object {
        self
    }

    #[inline(always)]
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        self
    }
}
unsafe impl<T> ObjcObject for &'_ mut T
where
    T: ObjcObject,
{
    #[inline(always)]
    fn as_id(&self) -> &objc::runtime::Object {
        T::as_id(self)
    }

    #[inline(always)]
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        T::as_id_mut(self)
    }
}

#[macro_export]
macro_rules! DefineObjcObjectWrapper {
    ($(#[$a: meta])* $v: vis $tyname: ident : $super: ty) => {
        #[repr(transparent)]
        $(#[$a])*
        $v struct $tyname(objc::runtime::Object);
        $crate::DefineObjcObjectWrapper!(__ext_struct $tyname : $super);
    };
    ($(#[$a: meta])* $v: vis $tyname: ident) => {
        #[repr(transparent)]
        $(#[$a])*
        $v struct $tyname(objc::runtime::Object);
        $crate::DefineObjcObjectWrapper!(__ext_struct $tyname);
    };

    { $(#[$a: meta])* $v: vis $tyname: ident : $super: ty; } => {
        #[repr(transparent)]
        $(#[$a])*
        $v struct $tyname(objc::runtime::Object);
        $crate::DefineObjcObjectWrapper!(__ext_struct $tyname : $super);
    };
    { $(#[$a: meta])* $v: vis $tyname: ident; } => {
        #[repr(transparent)]
        $(#[$a])*
        $v struct $tyname(objc::runtime::Object);
        $crate::DefineObjcObjectWrapper!(__ext_struct $tyname);
    };

    (__ext_struct $tyname: ident) => {
        unsafe impl $crate::ObjcObject for $tyname {
            #[inline(always)]
            fn as_id(&self) -> &objc::runtime::Object { &self.0 }
            #[inline(always)]
            fn as_id_mut(&mut self) -> &mut objc::runtime::Object { &mut self.0 }
        }

        unsafe impl objc::Encode for &'_ $tyname {
            #[inline(always)]
            fn encode() -> objc::Encoding {
                <&objc::runtime::Object as objc::Encode>::encode()
            }
        }
        unsafe impl objc::Encode for &'_ mut $tyname {
            #[inline(always)]
            fn encode() -> objc::Encoding {
                <&mut objc::runtime::Object as objc::Encode>::encode()
            }
        }

        unsafe impl objc::Message for $tyname {
            #[inline(always)]
            unsafe fn send_message<A, R>(&self, sel: objc::runtime::Sel, args: A)
                        -> Result<R, objc::MessageError>
                        where Self: Sized, A: objc::MessageArguments, R: std::any::Any {
                self.0.send_message::<A, R>(sel, args)
            }
        
            #[inline(always)]
            fn verify_message<A, R>(&self, sel: objc::runtime::Sel) -> Result<(), objc::MessageError>
                        where Self: Sized, A: objc::EncodeArguments, R: objc::Encode {
                self.0.verify_message::<A, R>(sel)
            }
        }
    };
    (__ext_struct $tyname: ident : $super: ty) => {
        $crate::DefineObjcObjectWrapper!(__ext_struct $tyname);
        impl std::ops::Deref for $tyname {
            type Target = $super;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl std::ops::DerefMut for $tyname {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { core::mem::transmute(self) }
            }
        }
    }
}

#[macro_export]
macro_rules! IdObject {
    ($v: vis $name: ident = id < $($p: path),+ >) => {
        #[repr(C)]
        $v struct $name(objc::runtime::Object);
        unsafe impl $crate::ObjcObject for $name {
            fn as_id(&self) -> &objc::runtime::Object { &self.0 }
            fn as_id_mut(&mut self) -> &mut objc::runtime::Object { &mut self.0 }
        }
        unsafe impl objc::Encode for &'_ $name {
            fn encode() -> objc::Encoding {
                <&objc::runtime::Object as objc::Encode>::encode()
            }
        }
        unsafe impl objc::Encode for &'_ mut $name {
            fn encode() -> objc::Encoding {
                <&mut objc::runtime::Object as objc::Encode>::encode()
            }
        }
        $(unsafe impl $p for $name {})+
    }
}
