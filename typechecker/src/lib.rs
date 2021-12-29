use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

// This is just a temporary poc writeup. I want to write this before
// i read into type checking algorithms/structures (maybe in "crafting algorithms").
//
// Maybe the name `Type` is a bit suboptimal.

pub type TypeId = u64;

// Maybe supporting the smaller float types would make sense.
// I really don't know which primitive numeric sizes to support

// NOTE Questions to check and handle:
// 1. equal?:      type T = U, type U = V => T == U && U == V && T == V
//     - Equal types have the same size
//     - Equal types are field wise safely transmutable to each other
// 2. same?:       type T = U => only T is the same as T
// 3. compatible?  type T = pub U => T <-> U => !(T <-> V where V <-> U) 
//     - compatible types are assignable to each other
//     - compatible types are always equal to each other
// So: `type T = pub U;` `type V = U`: T == U, V == U, T == V, T <-> U, !(V <-> U), !(T <-> V)
// and `type T = pub U;` `type V = pub U`: T == U, V == U, T == V, T <-> U, V <-> U, !(T <-> V)

pub struct TypeHandle {
    pub info: TypeInfo,
    /// Only same for equal type
    pub inner_id: TypeId,
}

pub struct TypeInfo {
    /// Only same for same types, globally unique for every type name and every anonymous type
    pub id: TypeId,
}

pub enum PrimitiveIntSize {
    S8,
    S16,
    S32,
    S64,
    S128,
    Ptr,
}

pub enum Primitive {
    Char,
    F32,
    F64,
    F128,
    Integer(PrimitiveIntSize),
    UnsignedInteger(PrimitiveIntSize),
}

pub enum Type {
    Map(TypeMap),
    Tuple(Vec<Type>),
    Sum(HashSet<Type>),
    // Enum later is more complicated
    Union(HashMap<String, Type>),
    // TODO move this text to some other text file
    // NOTE: There has to be a difference between ref and ptr because ref will be overloadable.
    // NOTE: Allocator info is done by holding an Allocator
    // Pointer types are handled like this:
    // &/View and &mut/ViewMut = overloadable template, different for many types
    // @/Ptr and @mut/PtrMut = Pointer reference
    // A C pointer would be @mut T
    // Another important pointer is the RawBox<T> which is the same as Box but
    // does not drop its content and just deallocates it.
    // RawBox<T> = {ptr: @mut T, alloc: Allocator}
    // Box<T> = RawBox; owned pointer which drops its content.
    // Vec could be: type Vec<T> = {ptr: @mut [T;?], cap: USize, len: USize, alloc: Allocator};
    Ptr(Box<Type>),
    PtrMut(Box<Type>),
}

pub struct TypeMap(HashMap<String, Type, ahash::RandomState>);

impl TypeMap {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_map(HashMap::with_capacity_and_hasher(
            capacity,
            Default::default(),
        ))
    }

    pub fn from_map(me: HashMap<String, Type, ahash::RandomState>) -> Self {
        Self(me)
    }

    pub fn into_inner(self) -> HashMap<String, Type, ahash::RandomState> {
        self.0
    }

    pub fn is_same(&self, other: &Self) -> bool {
        let m1 = &self.0;
        let m2 = &other.0;

        if m1.len() != m2.len() {
            return false;
        }

        for (k, v) in m1 {
            if let Some(t) = m2.get(k) {
                if !v.is_same(t) {
                    //field k inside m2 but has a different type than field k in m1
                    return false;
                }
            } else {
                // field k is not inside m2
                return false;
            }
        }

        true
    }
}

impl PartialEq for TypeMap {
    fn eq(&self, other: &Self) -> bool {
        self.is_same(other)
    }
}

// Maybe it makes sense to hide the types inside extra structs

impl Type {
    pub fn is_same(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Map(m1), Type::Map(m2)) => m1.is_same(&m2),
            _ => false,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.is_same(other)
    }
}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
