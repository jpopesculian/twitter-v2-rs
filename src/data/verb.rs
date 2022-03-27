use serde::{Deserialize, Serialize};

macro_rules! verb {
    ($vis:vis struct $class:ident { $verb:ident }) => {
        #[derive(Deserialize, Serialize, Clone, Copy, Debug)]
        $vis struct $class {
            pub $verb: bool
        }

        impl From<$class> for bool {
            fn from(verb: $class) -> bool {
                verb.$verb
            }
        }
    };
}

verb! { pub struct Deleted { deleted } }
verb! { pub struct Retweeted { retweeted } }
verb! { pub struct Liked { liked } }
