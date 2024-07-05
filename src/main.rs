#[derive(Debug, PartialEq, Clone)]
enum Element {
    Fire,
    Water,
    Snow,
}

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Orange,
    Purple,
}

#[derive(Debug, PartialEq, Clone)]
struct Card {
    element: Element,
    value: u8,
    color: Color,
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Vec<Card>,
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
    score: Score,
}

impl GameState {
    fn new(player1_name: &str, player2_name: &str) -> Self {
        GameState {
            player1: Player {
                name: player1_name.to_string(),
                hand: vec![],
            },
            player2: Player {
                name: player2_name.to_string(),
                hand: vec![],
            },
            score: Score {
                player1: vec![],
                player2: vec![],
            },
        }
    }

    fn play_round(&mut self, card1: Card, card2: Card) {
        println!(
            "{} played {:?} and {} played {:?}",
            self.player1.name, card1, self.player2.name, card2
        );

        match GameState::determine_winner(&self.player1, &card1, &self.player2, &card2) {
            Some(player) => {
                println!("{} wins the round!", player.name);
                if &self.player1.name == &player.name {
                    self.score.player1.push(card1.clone());
                } else {
                    self.score.player2.push(card2.clone());
                }
            }
            None => (),
        }

        if let Some(player) = self.check_end_condition() {
            println!("{} wins the game!", player.name);
        }
    }

    fn determine_winner<'a>(
        player1: &'a Player,
        card1: &'a Card,
        player2: &'a Player,
        card2: &'a Card,
    ) -> Option<&'a Player> {
        use Element::*;

        match (card1.element.clone(), card2.element.clone()) {
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

        mismatching_colors && mismatching_elements
    }

    fn check_end_condition(&self) -> Option<&Player> {
        use itertools::Itertools;

        let player1_has_winning_combination = self
            .score
            .player1
            .iter()
            .combinations(3)
            .any(|cards| GameState::end_condition(cards));

        if player1_has_winning_combination {
            return Some(&self.player1);
        }

        let player2_has_winning_combination = self
            .score
            .player2
            .iter()
            .combinations(3)
            .any(|cards| GameState::end_condition(cards));

        if player2_has_winning_combination {
            return Some(&self.player2);
        }

        None
    }
}

fn main() {
    let mut game = GameState::new("Player 1", "Player 2");

    let card1 = Card {
        element: Element::Fire,
        value: 5,
        color: Color::Red,
    };

    let card2 = Card {
        element: Element::Water,
        value: 3,
        color: Color::Blue,
    };

    game.play_round(card1, card2);
}