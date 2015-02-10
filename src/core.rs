// A simplified program structure amenable to simple type checking
// and type environment construction.
use syntax::ast;

#[derive(Debug)]
pub struct Name {
    string: String
}

impl Name {
    pub fn from_ident(ident: ast::Ident) -> Name {
        Name {
            string: ident.name.as_str().to_string()
        }
    }
}

#[derive(Debug)]
pub enum Item {
    Fn(Generics, Vec<Predicate>, Block),
    Trait(Generics, Vec<Predicate>, Vec<Item>),
    // Simplify the representation of data types
    Data(Name, Generics, Vec<Predicate>, Vec<Constructor>)
}

impl Item {
    pub fn record_struct(name: Name,
                         generics: Generics,
                         predicates: Vec<Predicate>,
                         fields: Vec<Field>) -> Item {

        Item::Data(name, generics, predicates, vec![Constructor::Record])
    }

    //fn record_struct(name: Name, generics: Generics, predicates: Predicates) -> Item {
        //panic!("NYI")j
    //}
}

// Represents a sequence of statements.
#[derive(Debug)]
enum Block { MkBlock }

#[derive(Debug)]
enum Field { DummyField }

// Generics are *only* types and lifetimes, bounds are completely seperate.
#[derive(Debug)]
pub struct Generics {
    types: Vec<Ty>,
    lifetimes: Vec<Lifetime>
}

impl Generics {
    pub fn empty() -> Generics {
        Generics {
            types: Vec::new(),
            lifetimes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Ty;

#[derive(Debug)]
pub struct Lifetime;

// A unified concept of a constructor
#[derive(Debug)]
enum Constructor {
    Tuple, // Anonymous fields like struct Foo(...) or an enum constructor
    Record, // A struct style record constructor
}

// No associated type projections yet
#[derive(Debug)]
enum Predicate {
    Trait,
    Outlives,
    Equate
}
