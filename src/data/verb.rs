use serde::{Deserialize, Serialize};

macro_rules! verb {
    ($vis:vis struct $class:ident { $verb:ident }) => {
        #[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $class {
            pub $verb: bool
        }

        impl $class {
            pub fn as_bool(&self) -> bool {
                self.$verb
            }
        }

        impl From<$class> for bool {
            fn from($verb: $class) -> bool {
                $verb.$verb
            }
        }

        impl From<bool> for $class {
            fn from($verb: bool) -> $class {
                $class { $verb }
            }
        }
    };
}

verb! { pub struct Deleted { deleted } }
verb! { pub struct Retweeted { retweeted } }
verb! { pub struct Liked { liked } }
verb! { pub struct Bookmarked { bookmarked } }
verb! { pub struct Hidden { hidden } }
verb! { pub struct Following { following } }
verb! { pub struct Blocking { blocking } }
verb! { pub struct Muting { muting } }
