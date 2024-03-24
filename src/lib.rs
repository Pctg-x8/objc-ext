/// An marker for FFI safe object with objective-c
pub unsafe trait ObjcObject {
    fn as_id(&self) -> &objc::runtime::Object;
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object;
}
unsafe impl ObjcObject for objc::runtime::Object {
    fn as_id(&self) -> &objc::runtime::Object {
        self
    }
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        self
    }
}
unsafe impl<T> ObjcObject for &'_ mut T
where
    T: ObjcObject,
{
    fn as_id(&self) -> &objc::runtime::Object {
        T::as_id(self)
    }
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object {
        T::as_id_mut(self)
    }
}

#[macro_export]
macro_rules! DefineObjcObjectWrapper {
    ($v: vis $tyname: ident) => {
        #[repr(C)]
        $v struct $tyname(objc::runtime::Object);
        $crate::DefineObjcObjectWrapper!(ext_struct $tyname);
    };
    ($v: vis $tyname: ident : $super: ty) => {
        #[repr(C)]
        $v struct $tyname(objc::runtime::Object);
        $crate::DefineObjcObjectWrapper!(ext_struct $tyname : $super);
    };

    (ext_struct $tyname: ident) => {
        unsafe impl $crate::ObjcObject for $tyname {
            fn as_id(&self) -> &objc::runtime::Object { &self.0 }
            fn as_id_mut(&mut self) -> &mut objc::runtime::Object { &mut self.0 }
        }
        unsafe impl objc::Encode for &'_ $tyname {
            fn encode() -> objc::Encoding {
                <&objc::runtime::Object as objc::Encode>::encode()
            }
        }
        unsafe impl objc::Encode for &'_ mut $tyname {
            fn encode() -> objc::Encoding {
                <&mut objc::runtime::Object as objc::Encode>::encode()
            }
        }
    };
    (ext_struct $tyname: ident : $super: ty) => {
        $crate::DefineObjcObjectWrapper!(ext_struct $tyname);
        impl std::ops::Deref for $tyname {
            type Target = $super;
            fn deref(&self) -> &Self::Target {
                unsafe { std::mem::transmute(self) }
            }
        }
        impl std::ops::DerefMut for $tyname {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { std::mem::transmute(self) }
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
