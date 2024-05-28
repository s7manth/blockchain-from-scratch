//! When you wear clothes they get dirty. When you wash them they get wet. When you dry them, they're
//! ready to be worn again. Or course washing and wearing clothes takes its toll on the clothes, and
//! eventually they get tattered.

use super::StateMachine;

/// This state machine models the typical life cycle of clothes as they make their way through the laundry
/// cycle several times before ultimately becoming tattered.
pub struct ClothesMachine;

/// Models a piece of clothing throughout its lifecycle.
#[derive(PartialEq, Eq, Debug)]
pub enum ClothesState {
    /// Clean clothes ready to be worn. With some given life left.
    Clean(u64),
    /// Dirty clothes. With some given life left.
    Dirty(u64),
    /// Wet clothes. With some given life left. The clothes are assumed to be wet because
    /// they were just washed. And washing clothes is the only modeled way to get them wet.
    Wet(u64),
    /// Tattered clothes beyond their useful life. These clothes will always be tattered no matter
    /// what is done with them.
    Tattered,
}

/// Something you can do with clothes
#[allow(dead_code)]
pub enum ClothesAction {
    /// Wearing clothes decreases their life by 1 and makes them dirty.
    Wear,
    /// Washing clothes decreases their life by 1, and makes them wet.
    Wash,
    /// This operation models a tumble drier. Drying clothes decreases their life by 1.
    /// If the clothes were clean or wet to begin with they will be clean after drying.
    /// If they were dirty to begin with, they will still be dirty after drying.
    Dry,
}

// fn tatter_helper(life: u64, next_state_if_untattered: ClothesState) -> ClothesState {
//     match life {
//         0 => ClothesState::Tattered,
//         more_to_go => match next_state_if_untattered {
//             ClothesState::Clean(_) => ClothesState::Clean(more_to_go),
//             ClothesState::Dirty(_) => ClothesState::Dirty(more_to_go),
//             ClothesState::Wet(_) => ClothesState::Wet(more_to_go),
//             ClothesState::Tattered => ClothesState::Tattered
//         }
//     }
// }

impl StateMachine for ClothesMachine {
    type State = ClothesState;
    type Transition = ClothesAction;

    fn next_state(starting_state: &ClothesState, t: &ClothesAction) -> ClothesState {
        // match t {
        //     ClothesAction::Dry => match starting_state {
        //         ClothesState::Clean(life) => tatter_helper(life - 1, ClothesState::Clean(0)),
        //         ClothesState::Wet(life) => tatter_helper(life - 1, ClothesState::Clean(0)),
        //         ClothesState::Dirty(life) => tatter_helper(life - 1, ClothesState::Dirty(0)),
        //         ClothesState::Tattered => ClothesState::Tattered
        //     }
        //     ClothesAction::Wash => match starting_state {
        //         ClothesState::Clean(life) => tatter_helper(life - 1, ClothesState::Wet(0)),
        //         ClothesState::Wet(life) => tatter_helper(life - 1, ClothesState::Wet(0)),
        //         ClothesState::Dirty(life) => tatter_helper(life - 1, ClothesState::Wet(0)),
        //         ClothesState::Tattered => ClothesState::Tattered
        //     }
        //     ClothesAction::Wear => match starting_state {
        //         ClothesState::Clean(life) => tatter_helper(life - 1, ClothesState::Dirty(0)),
        //         ClothesState::Wet(life) => tatter_helper(life - 1, ClothesState::Dirty(0)),
        //         ClothesState::Dirty(life) => tatter_helper(life - 1, ClothesState::Dirty(0)),
        //         ClothesState::Tattered => ClothesState::Tattered
        //     }
        // }

        // alternate implementation by matching on state instead of action
        match starting_state {
            ClothesState::Clean(life) | ClothesState::Dirty(life) | ClothesState::Wet(life) => match life {
                1 => ClothesState::Tattered,
                more_to_go => match t {
                    ClothesAction::Wash => ClothesState::Wet(more_to_go - 1),
                    ClothesAction::Wear => ClothesState::Dirty(more_to_go - 1),
                    ClothesAction::Dry => match starting_state {
                        ClothesState::Dirty(_) => ClothesState::Dirty(more_to_go - 1),
                        _ => ClothesState::Clean(more_to_go - 1),
                    }
                }
            },
            ClothesState::Tattered => ClothesState::Tattered,
        }
    }
}

#[test]
fn sm_2_wear_clean_clothes() {
    let start = ClothesState::Clean(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Dirty(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wear_dirty_clothes() {
    let start = ClothesState::Dirty(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Dirty(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wear_wet_clothes() {
    let start = ClothesState::Wet(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Dirty(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wear_tattered_clothes() {
    let start = ClothesState::Tattered;
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wear_clean_until_tattered() {
    let start = ClothesState::Clean(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wear_wet_until_tattered() {
    let start = ClothesState::Wet(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wear_dirty_until_tattered() {
    let start = ClothesState::Dirty(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wear);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_clean_clothes() {
    let start = ClothesState::Clean(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Wet(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_dirty_clothes() {
    let start = ClothesState::Dirty(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Wet(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_wet_clothes() {
    let start = ClothesState::Wet(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Wet(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_tattered_clothes() {
    let start = ClothesState::Tattered;
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_clean_until_tattered() {
    let start = ClothesState::Clean(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_wet_until_tattered() {
    let start = ClothesState::Wet(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_wash_dirty_until_tattered() {
    let start = ClothesState::Dirty(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Wash);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_clean_clothes() {
    let start = ClothesState::Clean(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Clean(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_dirty_clothes() {
    let start = ClothesState::Dirty(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Dirty(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_wet_clothes() {
    let start = ClothesState::Wet(4);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Clean(3);
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_tattered_clothes() {
    let start = ClothesState::Tattered;
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_clean_until_tattered() {
    let start = ClothesState::Clean(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_cwet_until_tattered() {
    let start = ClothesState::Wet(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}

#[test]
fn sm_2_dry_dirty_until_tattered() {
    let start = ClothesState::Dirty(1);
    let end = ClothesMachine::next_state(&start, &ClothesAction::Dry);
    let expected = ClothesState::Tattered;
    assert_eq!(end, expected);
}
