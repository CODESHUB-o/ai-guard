# AI Guard

**Behavioral multi-turn injection detection for LLM applications.**

AI Guard models adversarial escalation across user turns instead of scanning a single prompt blob. 
It detects escalation drift, oscillation patterns, threshold gaming attempts, and computes a composite behavioral risk score (0–100).

Designed for:

### Designed For

- LLM SaaS backends
- Prompt-based AI tools
- CI security enforcement
- Developers building AI workflows

---

## Why AI Guard?

Most prompt injection tools scan a single prompt blob.

AI Guard models:

- User-turn escalation behavior
- Drift progression over time
- Oscillation patterns
- Late detection risk
- Threshold gaming attempts
- Composite adversarial risk scoring

It is **behavioral**, not just pattern-based.

---

## Core Capabilities

- Transcript-aware analysis
- User-turn behavioral modeling
- Drift detection
- Oscillation detection
- Threshold gaming detection
- Composite behavioral risk score (0–100)
- CI threshold enforcement

---

## Installation

### Build from Source

```bash
git clone https://github.com/YOUR_USERNAME/ai-guard.git
cd ai-guard
cargo build --release
```

Binary will be located at:

```
target/release/ai_guard
```

---

## Quick Start

Analyze a transcript:

```bash
./target/release/ai_guard multiturn examples/sample.txt --threshold 95
```

If the composite risk score is greater than or equal to the threshold,  
the process exits with code `1` (CI failure).

---

## Example Output

```
AI Guard — Behavioral Risk Analysis
------------------------------------
User Turns           : 4
Peak Severity        : Critical
Final Severity       : Critical
Drift Detected       : true
Partial Suppression  : false
Oscillation Detected : false
Late Detection       : false
Threshold Gaming     : false
Composite Risk Score : 72
Risk Level           : High
CI Threshold         : 60
```

---

## JSON Output (CI Friendly)

```bash
./target/release/ai_guard multiturn transcript.txt --threshold 95 --json
```

Example:

```json
{
  "composite_risk": 72,
  "risk_level": "High"
}
```

---

## GitHub Action Example

```yaml
name: AI Guard Scan

on:
  pull_request:
  push:
    branches:
      - main

jobs:
  ai-guard:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - run: ./target/release/ai_guard multiturn examples/sample.txt --threshold 95
```

This will fail CI if behavioral risk exceeds the defined threshold.

---

## Composite Risk Model (0–100)

Risk score is calculated using:

- Peak severity weight
- Detection delay
- Escalation velocity
- Drift presence
- Suppression patterns
- Oscillation detection
- Threshold gaming behavior

This enables behavioral adversarial modeling rather than static pattern checks.

---

## Use Cases

- Secure LLM SaaS backends
- Validate prompt pipelines in CI
- Detect adversarial multi-turn escalation
- Monitor injection risk during development
- Guard AI tool execution workflows

---

## Roadmap

Upcoming Pro Features:

- Repository-wide batch scanning
- Risk trend comparison across commits
- Historical baseline tracking
- Advanced reporting export
- Custom scoring configuration

---

## License

MIT License

---

## Philosophy

AI Guard focuses on behavioral modeling rather than surface-level string matching.

The goal is to detect adversarial progression patterns — not just isolated keywords.

Security is not static. Behavior matters.
