use super::categories::CategoryResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub total_score: f32,
    pub severity: String,
    pub exit_code: i32,
    pub categories: Vec<CategoryResult>,
}

pub fn compute_score(categories: Vec<CategoryResult>) -> ScanResult {
    let mut total = 0.0;
    let mut critical_trigger_count = 0;

    for cat in &categories {
        if cat.triggered {
            total += cat.weight;

            if cat.name == "System Prompt Extraction"
                || cat.name == "Tool Invocation Escalation"
            {
                critical_trigger_count += 1;
            }
        }
    }

    if total > 1.0 {
        total = 1.0;
    }

    if critical_trigger_count >= 2 {
        total = 1.0;
    }

    let severity = match total {
        s if s < 0.20 => "Low",
        s if s < 0.50 => "Moderate",
        s if s < 0.80 => "High",
        _ => "Critical",
    }
    .to_string();

    let exit_code = match severity.as_str() {
        "Low" => 0,
        "Moderate" => 1,
        "High" => 2,
        "Critical" => 3,
        _ => 0,
    };

    ScanResult {
        total_score: total,
        severity,
        exit_code,
        categories,
    }
}