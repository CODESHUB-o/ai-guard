use regex::Regex;
use lazy_static::lazy_static;
use serde::Serialize;
use crate::utils::entropy::calculate_entropy;

#[derive(Debug, Serialize)]
pub struct CategoryResult {
    pub name: String,
    pub triggered: bool,
    pub match_count: usize,
    pub weight: f32,
}

lazy_static! {
    static ref INSTRUCTION_OVERRIDE_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"\bignore (all )?(previous|earlier) instructions\b").unwrap(),
        Regex::new(r"\boverride (system )?instructions\b").unwrap(),
    ];

    static ref SYSTEM_PROMPT_EXTRACTION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"\breveal\b.*\b(system|internal|hidden)\b").unwrap(),
        Regex::new(r"\bshow\b.*\b(system|internal|hidden)\b").unwrap(),
    ];

    static ref TOOL_ESCALATION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"\bexecute\b.*\b(shell|command|terminal)\b").unwrap(),
        Regex::new(r"\baccess\b.*(/etc/|\.env)").unwrap(),
    ];

    static ref PRIVILEGE_ESCALATION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"\bact as (root|admin|system)\b").unwrap(),
        Regex::new(r"\bgrant (me )?(full )?access\b").unwrap(),
    ];

    static ref BASE64_PATTERN: Regex =
        Regex::new(r"\b[a-zA-Z0-9+/]{20,}={0,2}\b").unwrap();

    static ref HEX_PATTERN: Regex =
        Regex::new(r"\b[a-fA-F0-9]{20,}\b").unwrap();
}

// -------- Heuristic Keyword Matching --------

fn keyword_cluster_score(input: &str, keywords: &[&str]) -> usize {
    keywords.iter()
        .filter(|k| input.contains(*k))
        .count()
}

pub fn detect_instruction_override(input: &str) -> CategoryResult {
    let mut match_count = 0;

    for pattern in INSTRUCTION_OVERRIDE_PATTERNS.iter() {
        match_count += pattern.find_iter(input).count();
    }

    // Heuristic intent detection
    let intent_score = keyword_cluster_score(
        input,
        &["ignore", "previous", "instructions"]
    );

    if intent_score >= 2 {
        match_count += 1;
    }

    CategoryResult {
        name: "Instruction Override".to_string(),
        triggered: match_count > 0,
        match_count,
        weight: 0.30,
    }
}

pub fn detect_system_prompt_extraction(input: &str) -> CategoryResult {
    let mut match_count = 0;

    for pattern in SYSTEM_PROMPT_EXTRACTION_PATTERNS.iter() {
        match_count += pattern.find_iter(input).count();
    }

    let intent_score = keyword_cluster_score(
        input,
        &["hidden", "internal", "system", "prompt", "directive", "config"]
    );

    if intent_score >= 2 {
        match_count += 1;
    }

    CategoryResult {
        name: "System Prompt Extraction".to_string(),
        triggered: match_count > 0,
        match_count,
        weight: 0.35,
    }
}

pub fn detect_tool_escalation(input: &str) -> CategoryResult {
    let mut match_count = 0;

    for pattern in TOOL_ESCALATION_PATTERNS.iter() {
        match_count += pattern.find_iter(input).count();
    }

    let intent_score = keyword_cluster_score(
        input,
        &["execute", "run", "terminal", "operation", "environment", "retrieve"]
    );

    if intent_score >= 2 {
        match_count += 1;
    }

    CategoryResult {
        name: "Tool Invocation Escalation".to_string(),
        triggered: match_count > 0,
        match_count,
        weight: 0.40,
    }
}

pub fn detect_privilege_escalation(input: &str) -> CategoryResult {
    let mut match_count = 0;

    for pattern in PRIVILEGE_ESCALATION_PATTERNS.iter() {
        match_count += pattern.find_iter(input).count();
    }

    let intent_score = keyword_cluster_score(
        input,
        &["elevate", "authority", "remove", "safeguards", "admin"]
    );

    if intent_score >= 2 {
        match_count += 1;
    }

    CategoryResult {
        name: "Privilege Escalation".to_string(),
        triggered: match_count > 0,
        match_count,
        weight: 0.25,
    }
}

pub fn detect_encoding_evasion(input: &str) -> CategoryResult {
    let mut match_count = 0;

    match_count += BASE64_PATTERN.find_iter(input).count();
    match_count += HEX_PATTERN.find_iter(input).count();

    let entropy = calculate_entropy(input);
    let high_entropy = entropy > 4.5 && input.len() > 40;

    CategoryResult {
        name: "Encoding Evasion Indicators".to_string(),
        triggered: match_count > 0 || high_entropy,
        match_count,
        weight: 0.20,
    }
}