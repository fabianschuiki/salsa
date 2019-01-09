use std::borrow::Borrow;
use typed_arena::Arena;

struct DatabaseImpl<'a> {
    arena: &'a Arena<String>,
    runtime: salsa::Runtime<DatabaseImpl<'a>>,
}

impl<'a> salsa::Database for DatabaseImpl<'a> {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }
}

salsa::query_group! {
    trait Database<'a>: salsa::Database + Borrow<DatabaseImpl<'a>> {
        fn uppercase(key: String) -> &'a str {
            type Uppercase;
        }
    }
}

salsa::database_storage! {
    struct DatabaseStorage<'a> for DatabaseImpl<'a> {
        impl Database<'a> {
            fn uppercase() for Uppercase<'a>;
        }
    }
}

fn uppercase<'a>(db: &impl Database<'a>, key: String) -> &'a str {
    db.borrow().arena.alloc(key.to_uppercase())
}

fn main() {
    let arena = Arena::new();
    let db = DatabaseImpl {
        arena: &arena,
        runtime: Default::default(),
    };
    assert_eq!(db.uppercase("Hello World".to_owned()), "HELLO WORLD");
}
