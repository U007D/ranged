use crate::consts::*;
use std::panic;

#[cfg(test)]
mod unit_tests;

pub struct Ranged<const START: i32, const END: i32> {
    value: i32,
}

impl<const START: i32, const END: i32> Ranged<START, END> {
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        // TODO: Upgrade to compile-time range bounds validity checking once supported/discovered
        match START <= END {
            true => match START <= value && value < END {
                true => Some(Self { value }),
                false => None,
            },
            false => panic!(msg::ERR_INVALID_RANGE_BOUNDS),
        }
    }
}

//impl<const START: i64, const END: i64> Ranged<START, END> {
//    #[must_use]
//    pub const fn from_i64(value: i64) -> Option<Self> {
//        // TODO: Upgrade to compile-time range bounds validity checking once supported/discovered
//        match START <= END {
//            true => match START <= value && value < END {
//                true => Some(Self { value }),
//                false => None,
//            },
//            false => panic!(msg::ERR_INVALID_RANGE_BOUNDS),
//        }
//    }
//}
//
//macro_rules! impl_ranged_for_type {
//    ( $($typ: ty)+ ) => {
//        $(
//        impl<const START: $typ, const END: $typ> Ranged<START, END> {
//            #[must_use]
//            pub const fn from_$typ(value: $typ) -> Option<Self> {
//                // TODO: Upgrade to compile-time range bounds validity checking once supported/discovered
//                match START <= END {
//                    true => match START <= value && value < END {
//                        true => Some(Self { value }),
//                        false => None,
//                    },
//                    false => panic!(msg::ERR_INVALID_RANGE_BOUNDS),
//                }
//            }
//        })+
//    };
//}
