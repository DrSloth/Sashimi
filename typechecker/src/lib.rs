use std::collections::{HashMap, HashSet};

// This is just a temporary poc writeup. I want to write this before
// i read into type checking algorithms/structures (maybe in "crafting algorithms"). 
// 
// Maybe the name `Type` is a bit suboptimal.

pub type TypeHash = u64;
pub type TypeId = u64;

// Maybe supporting the smaller float types would make sense. 
// I really don't know which primitive numeric sizes to support

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
    Map(HashMap<String, Type>),
    Tuple(Vec<Type>),
    Sum(HashSet<Type>),
    // Enum later is more complicated
    Union(HashMap<String, Type>),
    // The question is if this makes sense to define here
    // probably this should be ptr and then `type Ref<T> = Ptr<T> + 'a;` or there is 
    // only one type Ptr which has lang support for being dereferenced.
    // There could also be only the Ref type and a 'any lifetime which means that
    // the ptr would be: Ptr<T> = Ref<T> + 'any such that it removes the lifetime from the ref.
    // To differentiate mutability both Ptr and PtrMut would be needed and generally the
    // inherent building blocks should be as simple as possible.
    // I still havent decided if &T is redefinable this would make sense for fat pointers.
    // In Rust &i32 is something else than &str or &dyn Trait.
    // This is probably the only builtin type which has to be named i am against having
    // a *const / *mut syntax
    Ptr(Box<Type>),
    PtrMut(Box<Type>),
}

// Maybe it makes sense to hide the types inside extra structs

impl Type {
    pub fn is_same(&self, other: &Type) -> bool {
        /* match (self, other) {
            (Type::Map(m1), Type::Map(m2)) => {
                if m1.len() != m2.len() {
                    return false;
                }

                for (k,v) in m1 {
                    if !v.is_same(m2.get(k)) {
                        return false;
                    }
                }

                true
            }
            _ => false
        } */
        false
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/* impl std::hash::Hash for Type {
    fn hash(&self) {
        
    }
} */
