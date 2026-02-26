pub mod normalization;
pub mod categories;
pub mod scoring;

use normalization::normalize_input;
use categories::{
    detect_instruction_override,
    detect_system_prompt_extraction,
    detect_tool_escalation,
    detect_privilege_escalation,
    detect_encoding_evasion,
};
use scoring::{compute_score, ScanResult};

pub fn scan_input(raw_input: &str) -> ScanResult {
    let normalized = normalize_input(raw_input);

    let mut results = Vec::new();

    results.push(detect_instruction_override(&normalized));
    results.push(detect_system_prompt_extraction(&normalized));
    results.push(detect_tool_escalation(&normalized));
    results.push(detect_privilege_escalation(&normalized));
    results.push(detect_encoding_evasion(&normalized));

    compute_score(results)
}