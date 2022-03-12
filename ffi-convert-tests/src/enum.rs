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
pub struct BaguetteData {
    description: String,
    from_brittany: bool,
}

#[repr(C)]
#[derive(CReprOf, AsRust, CDrop, RawPointerConverter)]
#[target_type(BaguetteData)]
pub struct CBaguetteData {
    description: *const libc::c_char,
    from_brittany: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NightSnakeMenu {
    CocaCola,
    Baguette(BaguetteData),
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, AsRustEnum, CReprOfEnum, CDropEnum)]
#[target_type(NightSnakeMenu)]
pub enum NIGHT_SNAKE_MENU_TYPE {
    #[case(CocaCola)]
    COCA_COLA = 1,
    #[case(Baguette)]
    #[pointee(CBaguetteData)]
    BAGUETTE = 2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Meal {
    Breakfast(BreakfastMenu),
    Launch(LaunchMenu),
    Dinner,
    NightSnack(NightSnakeMenu),

    OnlyUsedInRust
}

#[repr(C)]
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
    #[case(NightSnack)]
    #[pointee(CEnum::<NIGHT_SNAKE_MENU_TYPE>)]
    NIGHT_SNACK = 4,
}

#[cfg(test)]
mod tests {
    use crate::generate_round_trip_rust_c_rust;
    use super::*;

    generate_round_trip_rust_c_rust!(round_trip_meal_breakfast, Meal, CEnum<MEAL_TYPE>, {
        Meal::Breakfast(BreakfastMenu {
            starter: "sausage".to_string(),
            dishes: 10,
            is_recommend: true,
        })
    });

    generate_round_trip_rust_c_rust!(round_trip_meal_launch, Meal, CEnum<MEAL_TYPE>, {
        Meal::Launch(LaunchMenu {
            starter: 1.1
        })
    });

    generate_round_trip_rust_c_rust!(round_trip_meal_dinner, Meal, CEnum<MEAL_TYPE>, {
        Meal::Dinner
    });

    generate_round_trip_rust_c_rust!(round_trip_meal_night_snacke, Meal, CEnum<MEAL_TYPE>, {
        Meal::NightSnack(NightSnakeMenu::Baguette(BaguetteData {
            description: "very long".to_string(),
            from_brittany: true,
        }))
    });
}