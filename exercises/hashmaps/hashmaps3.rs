// hashmaps3.rs
//
// 给出了一个足球比赛的比分列表（每行一个）。每行的格式为：
// "<队伍1名称>,<队伍2名称>,<队伍1进球>,<队伍2进球>"
// 示例：England,France,4,2（英格兰进4球，法国进2球）。
//
// 你需要建立一个比分表，包含队伍名称、该队进球数和失球数。
// 构建比分表的一种方法是使用哈希表（Hashmap）。解决方案已经部分
// 使用哈希表实现，请完成它以通过测试。
//
// 让我通过测试！
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        // Update team 1's score
        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        }); // or_insert() will insert a new entry if the key does not exist
        team_1.goals_scored += team_1_score; // 上面相当于初始化与获取，下面这两行是更新
        team_1.goals_conceded += team_2_score;
        // Update team 2's score
        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
