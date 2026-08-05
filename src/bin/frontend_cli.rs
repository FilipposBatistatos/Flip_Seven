use flip_seven::{ Action, Deck, Hand, Game, StepOutcome, Player, ControlMode, Strategy, ExpectedValue, Threshold };

use clap::Parser;
use std::time::{ SystemTime, UNIX_EPOCH };

use std::io::{ self, Write };

fn run(human_players: u8, bot_specs: &[String], target: u32, seed: u64)
{
    let mut players: Vec<Player> = bot_specs
        .iter()
        .map(|spec| Player::new(spec, strategy_from_spec(spec), ControlMode::Automatic))
        .collect();

    for id in 1..=human_players
    {
        players.push(Player::new(&format!("Human {}", id), Box::new(ExpectedValue), ControlMode::Advisory));
    }

    let mut game = Game::new(players, seed);
    let mut round : usize = 0;

    loop
    {
        round += 1;
        game.start_round(round);
        println!("\n=== Round {} ===", round);

        loop
        {
            match game.step()
            {
                StepOutcome::RoundOver => break,
                StepOutcome::NeedsInput { player_index, recommendation } =>
                {
                    print!("{}", game.display());
                    // println!(
                    //     "Your turn — suggestion: {:?} (Hit: {:.2} vs Stay: {:.2})",
                    //     recommendation.action, &format!("{}", recommendation.hit), &format!("{}", recommendation.stay)
                    //     );

                    let stats_str = match (recommendation.hit, recommendation.stay)
                    {
                        (Some(h), Some(s))  => format!("(Hit: {:.2} vs Stay: {:.2})", h, s),
                        (Some(h), None)     => format!("(Hit: {:.2}", h),
                        (None, Some(s))     => format!("(Stay: {:.2})", s),
                        (None, None)        => String::new(),
                    };

                    println!(
                        "Your turn - suggestion: {:?}{}",
                        recommendation.action,
                        stats_str
                    );

                    let action = read_action();
                    if game.apply(player_index, action)
                    {
                        // Game finished
                        break;
                    }
                }
            }
        }

        print!("{}", game.display());

        if game.players.iter().any(|p| p.cumulative_score >= target)
        {
            println!("Game over — target of {} reached.", target);
            break;
        }
    }
}

fn strategy_from_spec(spec: &str) -> Box<dyn Strategy>
{
    if spec == "expected_value"
    {
        Box::new(ExpectedValue)
    }
    else if let Some(cutoff) = spec.strip_prefix("threshold")
    {
        let cutoff: i32 = cutoff.parse().expect("threshold spec must end in a number, e.g. threshold25");
        Box::new(Threshold(cutoff))
    }
    else
    {
        panic!("unknown strategy spec: {}", spec);
    }
}

fn read_action() -> Action
{
    loop
    {
        print!("(h)it or (s)tay? ");
        io::stdout().flush().ok();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err()
        {
            continue;
        }

        let input = line.trim().to_lowercase();
        let mut parts = input.split_whitespace();

        let command = match parts.next()
        {
            Some(cmd) => cmd,
            None => continue,
        };

        match command
        {
            "h" | "hit" => 
            {
                let provided_card = parts
                    .next()
                    .and_then(|val| val.parse::<usize>().ok());

                return Action::Hit(provided_card);
            }
            "s" | "stay" => return Action::Stay,
            _ => println!("Didn't understand that — type 'h' or 's'."),
        }
    }
}

#[derive(Parser)]
#[command(name = "flip7")]
struct Cli
{
    #[arg(long)]
    human_players: Option<u8>,

    #[arg(long)]
    bots: Option<String>,

    #[arg(long, default_value_t = 200)]
    target: u32,

    #[arg(long)]
    seed: Option<u64>,
}

fn main()
{
    let cli = Cli::parse();
    let human_players: u8 = cli.human_players.unwrap_or(0);
    let bot_specs: Vec<String> = cli
        .bots
        .as_deref()
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let seed = cli.seed.unwrap_or_else(|| {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
    });

    run(human_players, &bot_specs, cli.target, seed);
}

