use anyhow::Result;
use ffi_convert::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreakfastMenu {
    starter: String,
    dishes: u32,
    is_recommend: bool,
}

#[repr(C)]
#[derive(CReprOf, AsRust, CDrop, RawPointerConverter)]
#[target_type(BreakfastMenu)]
pub struct CBreakfastMenu {
    starter: *const libc::c_char,
    dishes: u32,
    is_recommend: bool
}

#[derive(Clone, Debug, PartialEq)]
pub struct LaunchMenu {
    starter: f32
}

#[repr(C)]
#[derive(CReprOf, AsRust, CDrop, RawPointerConverter)]
#[target_type(LaunchMenu)]
pub struct CLaunchMenu {
    starter: f32
}

#[derive(Clone, Debug, PartialEq)]
pub enum Meal {
    Breakfast(BreakfastMenu),
    Launch(LaunchMenu),
    Dinner,

    OnlyUsedInRust
}

#[allow(non_camel_case_types)]
#[derive(Debug, AsRustEnum, CReprOfEnum, CDropEnum)]
#[target_type(Meal)]
pub enum MEAL_TYPE {
    #[default]
    UNIMPLEMENTED = 0,
    #[case(Breakfast)]
    #[pointee(CBreakfastMenu)]
    BREAKFAST = 1,
    #[case(Launch)]
    #[pointee(CLaunchMenu)]
    LAUNCH = 2,
    #[case(Dinner)]
    DINNER = 3,
}

#[derive(Debug)]
pub struct CMeal {
    ty: MEAL_TYPE,
    data: *const libc::c_void
}

impl AsRust<Meal> for CMeal {
    fn as_rust(&self) -> std::result::Result<Meal, AsRustError> {
        Ok(self.ty.as_rust(self.data)?)
    }
}

impl CDrop for CMeal {
    fn do_drop(&mut self) -> std::result::Result<(), CDropError> {
        Ok(())
    }
}

impl CReprOf<Meal> for CMeal {
    fn c_repr_of(input: Meal) -> Result<Self, CReprOfError> {
        let (ty, data) = MEAL_TYPE::c_repr_of(input)?;
        Ok(CMeal {
            ty,
            data
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_round_trip_rust_c_rust;
    use super::*;

    generate_round_trip_rust_c_rust!(round_trip_meal_breakfast, Meal, CMeal, {
        Meal::Breakfast(BreakfastMenu {
            starter: "sausage".to_string(),
            dishes: 10,
            is_recommend: true,
        })
    });

    generate_round_trip_rust_c_rust!(round_trip_meal_launch, Meal, CMeal, {
        Meal::Launch(LaunchMenu {
            starter: 1.1
        })
    });

    generate_round_trip_rust_c_rust!(round_trip_meal_dinner, Meal, CMeal, {
        Meal::Dinner
    });
}