use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Deserialize)]
struct Foo {
    a: u64,
    b: String,
}

/*
- Serialize trait on data structure:
    how to serialize itself,
    defines a way to call the appropriate Serializer
- Serializer has many methods defined, e.g., serialize_bool
- Serializer defines the data format, e.g., COBL, JSON, TOML
- Example:
    - If we want to serialise a struct, we call the
        serializer.serialize_struct(name_of_struct, len_of_nonskipped_fields)
            -> Result<Self::SerializeStruct, Self::Error>
    - Since struct is a nested data type, so we have to serialise each data field in the struct as well.
    - The return value is a SerilizeStruct (a sub-serializer), and it has the method serialize_field implemented.
    - The trait SerializeStruct also has end() that finish serializing a struct, and skip_field() to skip a key.
*/
// pub trait Serialize {
//     // Required method
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//        where S: Serializer;
// }

impl Serialize for Foo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Foo", 2)?;
        s.serialize_field("a", &self.a)?;
        s.serialize_field("b", &self.b)?;
        s.end()
    }
}

/*
- Deserialize trait on data structure:
    how to deserialize itself from any data format supported
- de lifetime parameter: a reference to the input the deserializer is working with, determines the lifetime of the input,
    e.g., a json string reference lifetime
- visitor makes it easy to deserialize nested structure
    - e.g., deserialize input data format into a sequence, with a visitor provided,
        so it is a hint that the data type is expecting a sequence value
    - e.g., for foo data structure, we need a visitor that visits a foo
*/
// pub trait Deserialize<'de>: Sized {
//     // Required method
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//        where D: Deserializer<'de>;
// }

// fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
// where
//     V: Visitor<'de>

enum Foo2 {
    Bar { u: u64 },
    Baz { s: String },
}

fn main() {}
