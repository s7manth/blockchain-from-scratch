//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use super::StateMachine;

/// The keys on the ATM keypad
#[allow(dead_code)]
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
#[allow(dead_code)]
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM
#[derive(Debug, PartialEq, Eq, Clone)]
enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}

/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}

// TODO: find a more *rusty* way to do this exercise
impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
    type State = Self;
    type Transition = Action;

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        // you could be in three different positions:
        // waiting => you just entered the atm booth
        // authenticated => then you just enter the amount to withdraw
        // authenticating => after you swipe card, you now know the hash,
        // and are waiting for the user to enter the pin
        let mut state = starting_state.clone();

        match state.expected_pin_hash {
            Auth::Waiting => match t {
                Action::PressKey(_key) => {
                    // why are you pressing keys without swiping the card?
                },
                Action::SwipeCard(hash) => {
                    state.expected_pin_hash = Auth::Authenticating(*hash);
                    state.keystroke_register = Vec::new();
                }
            },
            Auth::Authenticating(hash) => match t {
                Action::PressKey(key) => {
                    // yeah now you can enter your pin
                    // check whether one presses enter to wait for withdrawal access
                    match key {
                        Key::Enter => {
                            let attempt = crate::hash(&state.keystroke_register);
                            if attempt == hash {
                                state.expected_pin_hash = Auth::Authenticated;
                            } else {
                                state.expected_pin_hash = Auth::Waiting;
                            }
                            state.keystroke_register = Vec::new();
                        },
                        Key::One | Key::Two | Key::Three | Key::Four => {
                            state.keystroke_register.push(*key)
                        }
                    }
                },
                Action::SwipeCard(h) => {
                    state.expected_pin_hash = Auth::Authenticating(*h);
                    if *h != hash {
                        state.keystroke_register = Vec::new();
                    }
                }
            },
            Auth::Authenticated => match t {
                Action::PressKey(key) => {
                    match key {
                        Key::Enter => {
                            let attempt = state.keystroke_register.iter().fold(0, |acc, x| {
                                let digit = match x {
                                    Key::One => 1,
                                    Key::Two => 2,
                                    Key::Three => 3,
                                    Key::Four => 4,
                                    _ => 0 // it won't be possible since this is bound to get intercepted earlier
                                };

                                acc * 10 + digit
                            });
                            // println!("INFO amount requested {:?}", attempt);
                            if attempt <= state.cash_inside {
                                state.cash_inside -= attempt;
                            }
                            state.expected_pin_hash = Auth::Waiting;
                            state.keystroke_register = Vec::new();
                        },
                        Key::One | Key::Two | Key::Three | Key::Four => {
                            state.keystroke_register.push(*key)
                        }
                    }
                },
                Action::SwipeCard(hash) => {
                    state.expected_pin_hash = Auth::Authenticating(*hash);
                    state.keystroke_register = Vec::new();
                }
            },
        }
        state
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}
