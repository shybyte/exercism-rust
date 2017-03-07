use std::collections::HashMap;

#[derive(Debug)]
struct TeamStat {
    win: u32,
    drawn: u32,
    loss: u32,
}

fn matches_played(stat: &TeamStat) -> u32 {
    stat.win + stat.drawn + stat.loss
}

fn points(stat: &TeamStat) -> u32 {
    stat.win * 3 + stat.drawn
}

#[derive(Copy, Clone)]
enum MatchResult {
    WIN,
    DRAWN,
    LOSS
}

fn invert_match_result(match_result: MatchResult) -> MatchResult {
    match match_result {
        MatchResult::WIN => MatchResult::LOSS,
        MatchResult::DRAWN => MatchResult::DRAWN,
        MatchResult::LOSS => MatchResult::WIN
    }
}

fn update_stat(stats: &mut HashMap<String, TeamStat>, team: String, match_result: MatchResult) {
    let stat = stats.entry(team).or_insert_with(|| TeamStat { win: 0, drawn: 0, loss: 0 });
    match match_result {
        MatchResult::WIN => stat.win += 1,
        MatchResult::DRAWN => stat.drawn += 1,
        MatchResult::LOSS => stat.loss += 1
    }
}

pub fn tally(input: &str) -> String {
    // read stats
    let mut stats: HashMap<String, TeamStat> = HashMap::new();
    for line in input.lines() {
        let columns: Vec<_> = line.split(";").collect();
        if columns.len() != 3 { continue; }
        let (team1, team2, match_result_string) = (columns[0].into(), columns[1].into(), columns[2]);
        let match_result = match match_result_string {
            "win" => MatchResult::WIN,
            "loss" => MatchResult::LOSS,
            "draw" => MatchResult::DRAWN,
            _ => continue
        };
        update_stat(&mut stats, team1, match_result);
        update_stat(&mut stats, team2, invert_match_result(match_result));
    }

    // sort stats
    let mut stats_sorted: Vec<_> = stats.iter().collect();
    stats_sorted.sort_by_key(|&(team, _)| team);
    stats_sorted.sort_by_key(|&(_, s)| 0 - points(s) as i32);

    // build output string
    let mut output = "Team                           | MP |  W |  D |  L |  P".to_string();
    for &(team, stat) in stats_sorted.iter() {
        output.push_str(&format!(
            "\n{:31}| {:2} | {:2} | {:2} | {:2} | {:2}",
            team, matches_played(stat), stat.win, stat.drawn, stat.loss, points(stat)
        ));
    };

    output
}
