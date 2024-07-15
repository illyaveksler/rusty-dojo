use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Element {
    Fire,
    Water,
    Snow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Orange,
    Purple,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Power {
    None,
    ChangeScore(i8), // When this is scored, you card gets +2 or -2 for the next round. Scored means you have to win the point
    ChangeElement(Element, Element), // e.g., (Fire, Water) to change Fire cards to Water
    DiscardOpponentCardByElement(Element), // Discard a specific element card from the opponent
    DiscardOpponentCardByColor(Color), // Discard a specific color card from the opponent when SCORED
    TriggerSideEffect(SideEffect), // Triggers a side effect
}

// When played:

// For this round:
// When this is played, Water cards become Fire for this round. (both players)
// When this is played, Fire cards become Snowball for this round. (both players)
// When this is played, Snowball cards become Water for this round. (both players)
// When this is played, Snowball cards become Fire for this round (both players)
// When this is played, Fire cards become Water for this round. (both players)

// For next round:
// When this card is played, lower values win ties the next round. (both players)


// When scored:

// Discard color:
// When this is scored, discard one Opponent's Red card.
// When this is scored, discard one Opponent's Yellow card.
// When this is scored, discard one Opponent's Green card.
// When this is scored, discard one Opponent's Orange card.
// When this is scored, discard one opponent's Purple card.
// When this is scored, discard one Opponent's Blue card.
// When this is scored, discard all of Opponent's Red cards.
// When this is scored, discard all of Opponent's Yellow cards.
// When this is scored, discard all of Opponent's Green cards.
// When this is scored, discard all of Opponent's Orange cards.
// When this is scored, discard all of opponent's Purple cards.
// When this is scored, discard all of Opponent's Blue cards.

// Restrict element:
// When this is scored, Snow cannot be played next round.
// When this is scored, Water cannot be played next round.
// When this is scored, Fire cannot be played next round.

// Discard element:
// When this is scored, discard one Opponent's Snowball card.
// When this is scored, discard one Opponent's Water card.
// When this is scored, discard one Opponent's Fire card.

// Change value:
// When this is scored, your card gets +2 for the next round
// When this is scored, your Opponent's card get -2 for the next round

#[derive(Debug, PartialEq, Clone, Copy)]
enum SideEffect {
    None,
    RestrictElementNextRound(Element), // When this is scored, Fire cannot be played next round. Scored means you have to win the round
    LowerValueWinsTieNextRound,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Card {
    element: Element,
    value: u8,
    color: Color,
    power: Power,
}

impl Card {
    pub fn effective_value(&self, side_effect: &SideEffect) -> u8 {
        match side_effect {
            SideEffect::LowerValueWinsTieNextRound => self.value,
            _ => self.value,
        }
    }

    pub fn apply_power(&self, player: &mut Player, opponent: &mut Player) -> Option<SideEffect> {
        match self.power {
            Power::ChangeScore(amount) => {
                self.increase_score(player, amount);
                None
            }
            Power::ChangeElement(from, to) => {
                self.transform_element(opponent, from, to);
                None
            }
            Power::DiscardOpponentCardByElement(element) => {
                self.discard_opponent_card_by_element(opponent, element);
                None
            }
            Power::DiscardOpponentCardByColor(color) => {
                self.discard_opponent_card_by_color(opponent, color);
                None
            }
            Power::TriggerSideEffect(side_effect) => Some(side_effect),
            Power::None => None,
        }
    }

    fn increase_score(&self, player: &mut Player, amount: i8) {
        player.score.iter_mut().for_each(|card| card.value = (card.value as i8 + amount) as u8);
    }

    fn transform_element(&self, opponent: &mut Player, from: Element, to: Element) {
        opponent.hand.iter_mut().for_each(|card| {
            if card.element == from {
                card.element = to;
            }
        });
        // game.player2.hand.iter_mut().for_each(|card| {
        //     if card.element == from {
        //         card.element = to;
        //     }
        // });
    }

    fn discard_opponent_card_by_element(&self, opponent: &mut Player, element: Element) {
        if let Some(pos) = opponent.hand.iter().position(|card| card.element == element) {
            opponent.hand.remove(pos);
        }
    }

    fn discard_opponent_card_by_color(&self, opponent: &mut Player, color: Color) {
        if let Some(pos) = opponent.hand.iter().position(|card| card.color == color) {
            opponent.hand.remove(pos);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Player {
    name: String,
    hand: Vec<Card>,
    score: Vec<Card>,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            hand: vec![],
            score: vec![],
        }
    }
}

#[derive(Debug)]
struct Score {
    player1: Vec<Card>,
    player2: Vec<Card>,
}

#[derive(Debug)]
struct GameState {
    player1: Player,
    player2: Player,
    side_effect: SideEffect,
}

impl GameState {
    fn new(player1_name: &str, player2_name: &str) -> Self {
        GameState {
            player1: Player::new(player1_name),
            player2: Player::new(player2_name),
            side_effect: SideEffect::None,
        }
    }

    fn play_round(&mut self, card1: Card, card2: Card) {
        println!(
            "{} played {:?} and {} played {:?}",
            self.player1.name, card1, self.player2.name, card2
        );

        let card1_effective_value = card1.effective_value(&self.side_effect);
        let card2_effective_value = card2.effective_value(&self.side_effect);

        let effective_card1 = Card {
            element: card1.element,
            value: card1_effective_value,
            color: card1.color,
            power: card1.power,
        };

        let effective_card2 = Card {
            element: card2.element,
            value: card2_effective_value,
            color: card2.color,
            power: card2.power,
        };

        // Apply the powers after the initial processing
        if let Some(side_effect) = card1.apply_power(&mut self.player1, &mut self.player2) {
            self.side_effect = side_effect;
        }
        if let Some(side_effect) = card2.apply_power(&mut self.player2, &mut self.player1) {
            self.side_effect = side_effect;
        }

        match Self::determine_winner(
            &self.player1,
            &effective_card1,
            &self.player2,
            &effective_card2,
        ) {
            Some(winner) => {
                println!("{} wins the round!", winner.name);
                if winner.name == self.player1.name {
                    self.player1.score.push(card1);
                } else {
                    self.player2.score.push(card2);
                }
            }
            None => (),
        }

        if let Some(winner) = self.check_end_condition() {
            println!("{} wins the game!", winner.name);
        }

        // Disable side effect at end of each round
        self.side_effect = SideEffect::None;
    }

    fn determine_winner<'a>(
        player1: &'a Player,
        card1: &'a Card,
        player2: &'a Player,
        card2: &'a Card,
    ) -> Option<&'a Player> {
        use Element::*;

        match (card1.element, card2.element) {
            (Fire, Snow) | (Snow, Water) | (Water, Fire) => Some(player1),
            (Snow, Fire) | (Water, Snow) | (Fire, Water) => Some(player2),
            _ => {
                if card1.value == card2.value {
                    None
                } else if card1.value > card2.value {
                    Some(player1)
                } else {
                    Some(player2)
                }
            }
        }
    }

    fn end_condition(cards: Vec<&Card>) -> bool {
        let mismatching_colors = cards[0].color != cards[1].color
            && cards[1].color != cards[2].color
            && cards[0].color != cards[2].color;
        let mismatching_elements = cards[0].element != cards[1].element
            && cards[1].element != cards[2].element
            && cards[0].element != cards[2].element;
        let matching_elements = cards[0].element == cards[1].element
        && cards[1].element == cards[2].element
        && cards[0].element == cards[2].element;

        (mismatching_colors && mismatching_elements) || (mismatching_colors && matching_elements)
    }

    fn check_end_condition(&self) -> Option<&Player> {
        let player1_has_winning_combination = self
            .player1
            .score
            .iter()
            .combinations(3)
            .any(|cards| Self::end_condition(cards));

        if player1_has_winning_combination {
            return Some(&self.player1);
        }

        let player2_has_winning_combination = self
            .player2
            .score
            .iter()
            .combinations(3)
            .any(|cards| Self::end_condition(cards));

        if player2_has_winning_combination {
            return Some(&self.player2);
        }

        None
    }
}

fn main() {
    println!("Hello, world!");
}