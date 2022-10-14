use trust::{
    CheatingAgent, CooperatingAgent, CopycatAgent, DetectiveAgent, Game, GrudgerAgent, RoundOutcome,
};

fn test_game<'a>(mut game: Game, expected_outcomes: impl IntoIterator<Item = &'a RoundOutcome>) {
    let mut left_score = 0;
    let mut right_score = 0;

    for (i, expected) in expected_outcomes.into_iter().enumerate() {
        let outcome = game.play_round();
        let expected = expected.to_owned();
        assert_eq!(
            expected,
            outcome,
            "move #{}: expected {:?}, got {:?}",
            i + 1,
            expected,
            outcome,
        );

        match outcome {
            RoundOutcome::BothCooperated => {
                left_score += 2;
                right_score += 2;
            }
            RoundOutcome::LeftCheated => {
                left_score += 3;
                right_score -= 1;
            }
            RoundOutcome::RightCheated => {
                left_score -= 1;
                right_score += 3;
            }
            RoundOutcome::BothCheated => (),
        }

        assert_eq!(left_score, game.left_score());
        assert_eq!(right_score, game.right_score());
    }
}

#[test]
fn cooperators() {
    let game = Game::new(
        Box::new(CooperatingAgent::default()),
        Box::new(CooperatingAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 12]);
}

#[test]
fn cheaters() {
    let game = Game::new(
        Box::new(CheatingAgent::default()),
        Box::new(CheatingAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCheated; 8]);
}

#[test]
fn grudgers() {
    let game = Game::new(
        Box::new(GrudgerAgent::default()),
        Box::new(GrudgerAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 15]);
}

#[test]
fn copycats() {
    let game = Game::new(
        Box::new(CopycatAgent::default()),
        Box::new(CopycatAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 14]);
}

#[test]
fn detectives() {
    let game = Game::new(
        Box::new(DetectiveAgent::default()),
        Box::new(DetectiveAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 1].iter())
            .chain([RoundOutcome::BothCooperated; 12].iter()),
    );
}

#[test]
fn cooperator_cheater() {
    let game = Game::new(
        Box::new(CooperatingAgent::default()),
        Box::new(CheatingAgent::default()),
    );
    test_game(game, &[RoundOutcome::RightCheated; 18]);
}

#[test]
fn cooperator_grudger() {
    let game = Game::new(
        Box::new(CooperatingAgent::default()),
        Box::new(GrudgerAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 16]);
}

#[test]
fn cooperator_copycat() {
    let game = Game::new(
        Box::new(CooperatingAgent::default()),
        Box::new(CopycatAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 11]);
}

#[test]
fn cooperator_detective() {
    let game = Game::new(
        Box::new(CooperatingAgent::default()),
        Box::new(DetectiveAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::RightCheated; 1].iter())
            .chain([RoundOutcome::BothCooperated; 2].iter())
            .chain([RoundOutcome::RightCheated; 8].iter()),
    );
}

#[test]
fn cheater_grudger() {
    let game = Game::new(
        Box::new(CheatingAgent::default()),
        Box::new(GrudgerAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::LeftCheated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 10].iter()),
    );
}

#[test]
fn cheater_copycat() {
    let game = Game::new(
        Box::new(CheatingAgent::default()),
        Box::new(CopycatAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::LeftCheated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 7].iter()),
    );
}

#[test]
fn cheater_detective() {
    let game = Game::new(
        Box::new(CheatingAgent::default()),
        Box::new(DetectiveAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::LeftCheated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 1].iter())
            .chain([RoundOutcome::LeftCheated; 2].iter())
            .chain([RoundOutcome::BothCheated; 8].iter()),
    );
}

#[test]
fn grudger_copycat() {
    let game = Game::new(
        Box::new(GrudgerAgent::default()),
        Box::new(CopycatAgent::default()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 17]);
}

#[test]
fn grudger_detective() {
    let game = Game::new(
        Box::new(GrudgerAgent::default()),
        Box::new(DetectiveAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::RightCheated; 1].iter())
            .chain([RoundOutcome::LeftCheated; 2].iter())
            .chain([RoundOutcome::BothCheated; 8].iter()),
    );
}

#[test]
fn copycat_detective() {
    let game = Game::new(
        Box::new(CopycatAgent::default()),
        Box::new(DetectiveAgent::default()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::RightCheated; 1].iter())
            .chain([RoundOutcome::LeftCheated; 1].iter())
            .chain([RoundOutcome::BothCooperated; 11].iter()),
    );
}
