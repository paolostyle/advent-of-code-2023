use std::{cmp::Ordering, collections::hash_map::Entry};

use itertools::Itertools;

const CARD_ORDER: [char; 13] = [
  '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const JOKER_CARD_ORDER: [char; 13] = [
  'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Eq, PartialEq, Hash)]
struct Card<const JOKER: bool>(char);

impl<const JOKER: bool> Ord for Card<{ JOKER }> {
  fn cmp(&self, other: &Self) -> Ordering {
    let order = if JOKER { JOKER_CARD_ORDER } else { CARD_ORDER };
    order
      .iter()
      .position(|&val| val == self.0)
      .cmp(&order.iter().position(|&val| val == other.0))
  }
}

impl<const T: bool> PartialOrd for Card<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug, Eq)]
struct Hand<const T: bool> {
  cards: Vec<Card<T>>,
  counts: Vec<usize>,
  bid: u64,
}

impl<const JOKER: bool> Hand<{ JOKER }> {
  fn new(input: &str) -> Self {
    if let Some((cards_raw, bid)) = input.split_whitespace().collect_tuple() {
      let cards: Vec<_> = cards_raw.chars().map(Card::<JOKER>).collect();
      let counts = Hand::_get_cards_counts(&cards);
      Hand {
        cards,
        counts,
        bid: bid.parse::<u64>().unwrap(),
      }
    } else {
      panic!("invalid hand")
    }
  }

  fn _get_cards_counts(cards: &[Card<JOKER>]) -> Vec<usize> {
    let mut counts = cards.iter().counts();
    let unique_cards = counts.len();
    let joker_card: Card<JOKER> = Card::<JOKER>('J');

    if JOKER && unique_cards > 1 {
      if let Entry::Occupied(entry) = counts.entry(&joker_card) {
        let jokers_count = entry.remove();
        if let Some((_, value)) = counts.iter_mut().max_by_key(|v| *v.1) {
          *value += jokers_count;
        }
      }
    }

    counts.into_values().sorted().rev().collect()
  }
}

impl<const T: bool> PartialEq for Hand<T> {
  fn eq(&self, other: &Self) -> bool {
    self.cards == other.cards
  }
}

impl<const JOKER: bool> Ord for Hand<{ JOKER }> {
  fn cmp(&self, other: &Self) -> Ordering {
    for (self_count, other_count) in self.counts.iter().zip(&other.counts) {
      match self_count.cmp(other_count) {
        Ordering::Equal => continue,
        ordering => return ordering,
      }
    }

    for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
      match self_card.cmp(other_card) {
        Ordering::Equal => continue,
        ordering => return ordering,
      }
    }
    panic!("unresolved");
  }
}

impl<const T: bool> PartialOrd for Hand<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn total_winnings<const T: bool>(input: &[String]) -> u64 {
  input
    .iter()
    .map(|str| Hand::<T>::new(str))
    .sorted()
    .zip(1_u64..)
    .map(|(hand, rank)| hand.bid * rank)
    .sum()
}

fn main() {
  aoc2023::run(7, total_winnings::<false>, total_winnings::<true>);
}
