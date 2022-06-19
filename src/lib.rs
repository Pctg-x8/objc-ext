
/// An marker for FFI safe object with objective-c
pub unsafe trait ObjcObject {
    fn as_id(&self) -> &objc::runtime::Object;
    fn as_id_mut(&mut self) -> &mut objc::runtime::Object;
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
    
    (ext_struct $tyname: ty) => {
        unsafe impl $crate::ObjcObject for $tyname {
            fn as_id(&self) -> &objc::runtime::Object { &self.0 }
            fn as_id_mut(&mut self) -> &mut objc::runtime::Object { &mut self.0 }
        }
    };
    (ext_struct $tyname: ty : $super: ty) => {
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
        $(unsafe impl $p for $name {})+
    }
}
