macro_rules! fwd_mod {
    ($mod:ident) => {
        mod $mod;
        pub use $mod::*;
    };
}

fwd_mod!(day01);
