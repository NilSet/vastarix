//#![warn(rust_2018_idioms)]

pub mod primitives {
    use gc_derive::{Finalize, Trace};

    #[derive(Trace, Finalize)]
    pub struct Undefined;
    #[derive(Trace, Finalize)]
    pub struct Null;
    pub type Boolean = bool;
    #[derive(Trace, Finalize, Hash, Eq, PartialEq)]
    pub struct String {
        pub data: Vec<u16>,
    }
    #[derive(Trace, Finalize, Hash, Eq, PartialEq)]
    pub struct Symbol {
        pub description: Option<String>,
    }
    pub type Number = f64;
}

pub mod object {
    use crate::primitives::*;
    use gc::{custom_trace, Gc, GcCell, Trace};
    use gc_derive::{Finalize, Trace};
    use indexmap::IndexMap;

    pub trait Callable: Trace {}

    #[derive(Trace, Finalize, Hash, Eq, PartialEq)]
    pub enum PropertyKey {
        String(String),
        Symbol(Box<Symbol>),
    }
    #[derive(Trace, Finalize)]
    pub enum Value {
        Undefined(Undefined),
        Null(Null),
        Boolean(Boolean),
        String(String),
        Symbol(Box<Symbol>),
        Number(Number),
        Object(Box<Object>),
    }

    #[derive(Trace, Finalize)]
    pub struct DataProperty {
        pub value: Value,
        pub writable: bool,
        pub enumerable: bool,
        pub configurable: bool,
    }
    #[derive(Trace, Finalize)]
    pub struct AccessorProperty {
        pub get: Option<Gc<GcCell<Callable>>>,
        pub set: Option<Gc<GcCell<Callable>>>,
        pub enumerable: bool,
        pub configurable: bool,
    }
    #[derive(Trace, Finalize)]
    pub enum Property {
        DataProperty(DataProperty),
        AccessorProperty(AccessorProperty),
    }

    #[derive(Trace, Finalize)]
    pub enum Object {
        OrdinaryObject(OrdinaryObject),
        ArrayObject(ArrayObject),
    }

    macro_rules! impl_object_method {
        ($method:ident(&self $(,$argn:ident: $argt:ty)*) -> $ret:ty) => {
            fn $method(&self $(,$argn: $argt)*) -> $ret {
                match self {
                    Object::OrdinaryObject(object) => object.$method($($argn),*),
                    Object::ArrayObject(object) => object.$method($($argn),*),
                }
            }
        };
        ($method:ident(&mut self $(,$argn:ident: $argt:ty)*) -> $ret:ty) => {
            fn $method(&mut self $(,$argn: $argt)*) -> $ret {
                match self {
                    Object::OrdinaryObject(object) => object.$method($($argn),*),
                    Object::ArrayObject(object) => object.$method($($argn),*),
                }
            }
        };
    }

    #[derive(Finalize)]
    pub struct OrdinaryObject {
        pub own_properties: IndexMap<PropertyKey, Property>,
        pub extensible: Boolean,
        pub prototype: Option<Gc<GcCell<Object>>>,
    }
    unsafe impl Trace for OrdinaryObject {
        custom_trace!(this, {
            for (key, value) in &this.own_properties {
                mark(key);
                mark(value);
            }
            mark(&this.prototype);
        });
    }

    #[derive(Trace, Finalize)]
    pub struct ArrayObject {
        pub inner: OrdinaryObject,
    }

    pub trait ObjectInternalMethods {
        fn get_prototype_of(&self) -> Option<Box<Object>>;
        fn set_prototype_of(&mut self, proto: Option<Box<Object>>) -> Boolean;
        fn is_extensible(&self) -> Boolean;
        fn prevent_extensions(&mut self) -> Boolean;
        fn get_own_property(&self, key: PropertyKey) -> Option<Property>;
    }

    impl ObjectInternalMethods for OrdinaryObject {
        fn get_prototype_of(&self) -> Option<Box<Object>> {
            None
        }
        fn set_prototype_of(&mut self, _proto: Option<Box<Object>>) -> Boolean {
            false
        }
        fn is_extensible(&self) -> Boolean {
            self.extensible
        }
        fn prevent_extensions(&mut self) -> Boolean {
            self.extensible = false;
            true
        }
        fn get_own_property(&self, _key: PropertyKey) -> Option<Property> {
            None
        }
    }
    impl ObjectInternalMethods for ArrayObject {
        fn get_prototype_of(&self) -> Option<Box<Object>> {
            self.inner.get_prototype_of()
        }
        fn set_prototype_of(&mut self, proto: Option<Box<Object>>) -> Boolean {
            self.inner.set_prototype_of(proto)
        }
        fn is_extensible(&self) -> Boolean {
            self.inner.is_extensible()
        }
        fn prevent_extensions(&mut self) -> Boolean {
            self.inner.prevent_extensions()
        }
        fn get_own_property(&self, key: PropertyKey) -> Option<Property> {
            self.inner.get_own_property(key)
        }
    }
    impl ObjectInternalMethods for Object {
        impl_object_method!(get_prototype_of(&self) -> Option<Box<Object>>);
        impl_object_method!(set_prototype_of(&mut self, proto: Option<Box<Object>>) -> Boolean);
        impl_object_method!(is_extensible(&self) -> Boolean);
        impl_object_method!(prevent_extensions(&mut self) -> Boolean);
        impl_object_method!(get_own_property(&self, key: PropertyKey) -> Option<Property>);
    }
}
