use crate::scanner::scan_input;

pub struct MultiTurnResult {
    pub total_user_turns: usize,
    pub peak_severity: String,
    pub final_severity: String,
    pub first_high_turn: Option<usize>,

    pub drift_detected: bool,
    pub partial_bypass: bool,

    pub oscillation_detected: bool,
    pub late_detection: bool,
    pub threshold_gaming: bool,

    pub composite_risk_score: u32,
    pub risk_level: String,
}

#[derive(Debug)]
struct Turn {
    role: String,
    content: String,
}

fn severity_rank(sev: &str) -> i32 {
    match sev {
        "Low" => 0,
        "Moderate" => 1,
        "High" => 2,
        "Critical" => 3,
        _ => 0,
    }
}

fn rank_to_severity(rank: i32) -> String {
    match rank {
        0 => "Low".to_string(),
        1 => "Moderate".to_string(),
        2 => "High".to_string(),
        _ => "Critical".to_string(),
    }
}

fn severity_weight(rank: i32) -> u32 {
    match rank {
        0 => 5,
        1 => 15,
        2 => 30,
        _ => 40,
    }
}

fn risk_label(score: u32) -> String {
    match score {
        0..=20 => "Low".to_string(),
        21..=40 => "Moderate".to_string(),
        41..=60 => "Elevated".to_string(),
        61..=80 => "High".to_string(),
        _ => "Critical".to_string(),
    }
}

fn parse_transcript(input: &str) -> Vec<Turn> {
    let mut turns = Vec::new();
    let mut current_role = String::new();
    let mut current_content = String::new();

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("role:") {
            if !current_role.is_empty() {
                turns.push(Turn {
                    role: current_role.clone(),
                    content: current_content.trim().to_string(),
                });
                current_content.clear();
            }
            current_role = trimmed.replace("role:", "").trim().to_string();
        } else if trimmed.starts_with("content:") {
            current_content = trimmed.replace("content:", "").trim().to_string();
        } else if !trimmed.starts_with("[TURN") && !trimmed.starts_with("#") {
            if !trimmed.is_empty() {
                current_content.push(' ');
                current_content.push_str(trimmed);
            }
        }
    }

    if !current_role.is_empty() {
        turns.push(Turn {
            role: current_role,
            content: current_content.trim().to_string(),
        });
    }

    turns
}

pub fn run_multiturn(transcript: &str) -> MultiTurnResult {
    let turns = parse_transcript(transcript);

    let mut cumulative_context = String::new();
    let mut total_user_turns = 0;

    let mut peak_rank = 0;
    let mut final_rank = 0;
    let mut first_high_turn: Option<usize> = None;
    let mut previous_rank = 0;
    let mut drift_detected = false;
    let mut max_jump = 0;

    let mut severity_history: Vec<i32> = Vec::new();

    let mut user_turn_index = 0;

    for turn in turns {
        cumulative_context.push('\n');
        cumulative_context.push_str(&turn.content);

        let result = scan_input(&cumulative_context);

        if turn.role == "user" {
            total_user_turns += 1;
            user_turn_index += 1;

            let current_rank = severity_rank(&result.severity);

            severity_history.push(current_rank);

            if current_rank > peak_rank {
                peak_rank = current_rank;
            }

            if current_rank >= 2 && first_high_turn.is_none() {
                first_high_turn = Some(user_turn_index);
            }

            if current_rank > previous_rank {
                drift_detected = true;
                let jump = current_rank - previous_rank;
                if jump > max_jump {
                    max_jump = jump;
                }
            }

            previous_rank = current_rank;
            final_rank = current_rank;
        }
    }

    // -------------------------
    // Behavioral Anomaly Layer
    // -------------------------

    // Partial suppression
    let partial_bypass = peak_rank > final_rank;

    // Late detection
    let late_detection = if let Some(turn) = first_high_turn {
        turn == total_user_turns
    } else {
        false
    };

    // Oscillation detection (rank decrease after increase)
    let mut oscillation_detected = false;
    for i in 1..severity_history.len() {
        if severity_history[i] < severity_history[i - 1] {
            oscillation_detected = true;
        }
    }

    // Threshold gaming (never reached High, but drift happened)
    let threshold_gaming =
        peak_rank == 1 && drift_detected && first_high_turn.is_none();

    // -------------------------
    // Composite Risk Calculation
    // -------------------------

    let mut composite = severity_weight(peak_rank);

    // Detection delay weight (0–20)
    let delay_weight = if let Some(turn) = first_high_turn {
        ((turn - 1) as f32 / total_user_turns as f32 * 20.0) as u32
    } else {
        20
    };
    composite += delay_weight;

    // Escalation velocity (0–15)
    let velocity_weight = match max_jump {
        0 => 5,
        1 => 8,
        2 => 12,
        _ => 15,
    };
    composite += velocity_weight;

    // Drift presence (0–10)
    if drift_detected {
        composite += 10;
    }

    // Suppression weight (0–15)
    if partial_bypass {
        composite += 15;
    }

    // Oscillation weight (+10)
    if oscillation_detected {
        composite += 10;
    }

    // Late detection weight (+10)
    if late_detection {
        composite += 10;
    }

    // Threshold gaming weight (+10)
    if threshold_gaming {
        composite += 10;
    }

    if composite > 100 {
        composite = 100;
    }

    let risk_level = risk_label(composite);

    MultiTurnResult {
        total_user_turns,
        peak_severity: rank_to_severity(peak_rank),
        final_severity: rank_to_severity(final_rank),
        first_high_turn,
        drift_detected,
        partial_bypass,
        oscillation_detected,
        late_detection,
        threshold_gaming,
        composite_risk_score: composite,
        risk_level,
    }
}