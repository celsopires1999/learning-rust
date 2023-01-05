use super::*;

#[test]
fn it_should_create_an_empty_hand() {
    let hand = Hand::new();
    assert_eq!(hand.cards.len(), 0);
}

#[test]
fn it_should_add_a_card() {
    let mut hand = Hand::new();
    hand.add(Card::Ace);
    assert_eq!(hand.cards[0], Card::Ace);
}

#[test]
fn it_should_not_be_loosing_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    assert!(!hand.is_loosing_hand());
}

#[test]
fn it_should_be_loosing_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Five);
    hand.add(Card::Ace);
    assert!(hand.is_loosing_hand());
}

#[test]
fn it_should_be_hand_value_11() {
    let mut hand = Hand::new();
    hand.add(Card::Five);
    hand.add(Card::Six);
    assert_eq!(hand.value(), 11);
}

#[test]
fn it_should_be_hand_value_22() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    hand.add(Card::Ace);
    assert_eq!(hand.value(), 22);
}

#[test]
fn test_all_cards() {
    let mut hand = Hand::new();
    hand.add(Card::Ace);
    assert_eq!(hand.value(), 11);

    let mut hand = Hand::new();
    hand.add(Card::Two);
    assert_eq!(hand.value(), 2);

    let mut hand = Hand::new();
    hand.add(Card::Three);
    assert_eq!(hand.value(), 3);

    let mut hand = Hand::new();
    hand.add(Card::Four);
    assert_eq!(hand.value(), 4);

    let mut hand = Hand::new();
    hand.add(Card::Five);
    assert_eq!(hand.value(), 5);

    let mut hand = Hand::new();
    hand.add(Card::Six);
    assert_eq!(hand.value(), 6);

    let mut hand = Hand::new();
    hand.add(Card::Seven);
    assert_eq!(hand.value(), 7);

    let mut hand = Hand::new();
    hand.add(Card::Eight);
    assert_eq!(hand.value(), 8);

    let mut hand = Hand::new();
    hand.add(Card::Nine);
    assert_eq!(hand.value(), 9);

    let mut hand = Hand::new();
    hand.add(Card::Jack);
    assert_eq!(hand.value(), 10);

    let mut hand = Hand::new();
    hand.add(Card::Queen);
    assert_eq!(hand.value(), 10);

    let mut hand = Hand::new();
    hand.add(Card::King);
    assert_eq!(hand.value(), 10);
}
