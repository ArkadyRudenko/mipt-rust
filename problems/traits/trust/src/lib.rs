#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub trait Player {
    fn is_put_coin(&self) -> bool;
    fn update_behaviour(&mut self, is_cooperative_of_adversary: bool);
}

pub struct Game {
    left_player: Box<dyn Player>,
    right_player: Box<dyn Player>,
    left_coins: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Player>, right: Box<dyn Player>) -> Self {
        Self {
            left_player: left,
            right_player: right,
            left_coins: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_coins
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left = self.left_player.is_put_coin();
        let right = self.right_player.is_put_coin();

        self.right_player.update_behaviour(left);
        self.left_player.update_behaviour(right);
        return if left && right {
            self.left_coins += 2;
            self.right_score += 2;
            RoundOutcome::BothCooperated
        } else if !left && !right {
            RoundOutcome::BothCheated
        } else if !left && right {
            self.left_coins += 3;
            self.right_score -= 1;
            RoundOutcome::LeftCheated
        } else {
            self.left_coins -= 1;
            self.right_score += 3;
            RoundOutcome::RightCheated
        };
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Player for CheatingAgent {
    fn is_put_coin(&self) -> bool {
        false
    }

    fn update_behaviour(&mut self, is_cooperative_of_adversary: bool) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Player for CooperatingAgent {
    fn is_put_coin(&self) -> bool {
        true
    }

    fn update_behaviour(&mut self, is_cooperative_of_adversary: bool) {}
}
////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    is_cooperative: bool,
}

impl Default for GrudgerAgent {
    fn default() -> Self {
        Self {
            is_cooperative: true,
        }
    }
}

impl Player for GrudgerAgent {
    fn is_put_coin(&self) -> bool {
        self.is_cooperative
    }

    fn update_behaviour(&mut self, is_cooperative_of_adversary: bool) {
        if !is_cooperative_of_adversary {
            self.is_cooperative = false;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    is_cooperative: bool,
}

impl Default for CopycatAgent {
    fn default() -> Self {
        Self {
            is_cooperative: true,
        }
    }
}

impl Player for CopycatAgent {
    fn is_put_coin(&self) -> bool {
        self.is_cooperative
    }

    fn update_behaviour(&mut self, is_cooperative_of_adversary: bool) {
        self.is_cooperative = is_cooperative_of_adversary
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    is_cooperative: bool,
    iter: usize,
    adver_is_cheat: bool,
    is_ready: bool,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        Self {
            is_cooperative: true,
            iter: 0,
            adver_is_cheat: false,
            is_ready: false,
        }
    }
}

struct Behavior {
    steps: Vec<bool>,
    step_index: usize,
    is_fixed_behavior: bool,
}

impl Behavior {
    fn new() -> Self {
        Self {
            steps: vec![true, false, true, true],
            step_index: 0,
            is_fixed_behavior: true,
        }
    }

    fn get_step(&self) -> bool {
        self.steps[self.step_index]
    }

    fn update_behaviour(&mut self, adver_step: bool) {
        self.step_index += 1;
    }
}

impl Player for DetectiveAgent {
    fn is_put_coin(&self) -> bool {
        self.is_cooperative
    }

    fn update_behaviour(&mut self, is_cooperative_of_adversary: bool) {
        self.iter += 1;
        if !is_cooperative_of_adversary && !self.is_ready {
            self.is_ready = true;
            self.adver_is_cheat = true;
        }
        if self.iter < 4 {
            if self.iter == 1 {
                self.is_cooperative = false;
            } else if self.iter == 2 {
                self.is_cooperative = true;
            }
        } else {
            if self.adver_is_cheat {
                self.is_cooperative = is_cooperative_of_adversary;
            } else {
                self.is_cooperative = false;
            }
        }
    }
}
