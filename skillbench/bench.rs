#!/usr/bin/env rust-script
//! skillbench — measure what the caveman `SKILL.md` does to billed token output
//! and accuracy on established math & CS evals.
//!
//! Arms (identical except the system prompt):
//!   baseline   — neutral solver prompt, no caveman
//!   caveman    — same prompt + the full caveman SKILL.md body (frontmatter stripped)
//!
//! Final-tagless: `Llm` is the DSL, `MockLlm`/`RealLlm` interpret it, and
//! `run_benchmark` is written once against the trait. Metrics come from the API's
//! own `usage`: `output_tokens` (the full billed generation — the caveman headline)
//! and `output_tokens_details.thinking_tokens`. Tasks come from established eval
//! libraries via scripts/fetch_evals.py.
//!
//! Honest limit: these eval splits force a terse final answer (a bare integer,
//! letter, or function body), so there is little prose for caveman to compress —
//! the bench tests that the SKILL.md does NOT cost accuracy or bloat output, not
//! the README's ~75% prose-compression figure (which lives in benchmarks/).
//!
//! Modes: `mock` (no API), `run` (both arms, fresh), `report` (regenerate
//! REPORT.md from saved samples, no API).
//!
//! ```cargo
//! [dependencies]
//! serde_json = "1"
//! ```
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

const MODEL: &str = "claude-opus-4-8";
const EFFORT: &str = "high";
const MAX_TOKENS: u32 = 32000;

const BASE_SYS: &str = "You solve problems precisely. Reason internally as much as you need, then give exactly what is asked for.";

const ARMS: [&str; 2] = ["baseline", "caveman"];

// the caveman SKILL.md body, with YAML frontmatter stripped, appended to BASE_SYS
fn skill_body(root: &std::path::Path) -> String {
    let txt = std::fs::read_to_string(root.join("../skills/caveman/SKILL.md"))
        .expect("../skills/caveman/SKILL.md");
    let body = match txt.strip_prefix("---") {
        Some(rest) => match rest.split_once("\n---") {
            Some((_fm, b)) => b.trim_start_matches(['\r', '\n']).to_string(),
            None => txt.clone(),
        },
        None => txt.clone(),
    };
    body.trim().to_string()
}
fn arm_sys(arm: &str, skill: &str) -> String {
    match arm {
        "caveman" => format!("{BASE_SYS}\n\n{skill}"),
        _ => BASE_SYS.to_string(),
    }
}

// ---- the DSL -------------------------------------------------------------
#[derive(Clone)]
struct Req { system: String, user: String, display: String }
#[derive(Clone)]
struct Resp { input: i64, output: i64, thinking: i64, answer: String, summary: String, stop: String }
trait Llm { fn complete(&self, req: &Req) -> Resp; }

struct MockLlm;
impl Llm for MockLlm {
    fn complete(&self, req: &Req) -> Resp {
        let base = 1200 + (req.user.len() as i64 % 5) * 150;
        // caveman should shrink the visible output without changing the answer
        let output = if req.system.contains("smart caveman") { base * 25 / 100 } else { base };
        let thinking = output * 90 / 100;
        Resp { input: 40, output, thinking, answer: "42".into(), summary: String::new(), stop: "end_turn".into() }
    }
}

struct RealLlm { key: String }
impl RealLlm {
    fn new() -> RealLlm {
        let out = Command::new("rageveil").args(["show", "geosurge.ai/api.anthropic.com/onehr-cellvm/pool"]).output().expect("run rageveil");
        let key = String::from_utf8_lossy(&out.stdout).lines().map(|l| l.trim()).find(|l| !l.is_empty()).expect("no key from rageveil").to_string();
        RealLlm { key }
    }
}
impl Llm for RealLlm {
    fn complete(&self, req: &Req) -> Resp {
        let body = json!({
            "model": MODEL, "max_tokens": MAX_TOKENS,
            "thinking": {"type": "adaptive", "display": req.display},
            "output_config": {"effort": EFFORT},
            "system": req.system,
            "messages": [{"role":"user","content": req.user}],
        }).to_string();
        let v = curl_post(&self.key, &body);
        if v.get("type").and_then(|t| t.as_str()) == Some("error") {
            eprintln!("  API error: {}", v["error"]);
            return Resp { input: 0, output: 0, thinking: 0, answer: String::new(), summary: String::new(), stop: "error".into() };
        }
        let u = &v["usage"];
        Resp {
            input: u["input_tokens"].as_i64().unwrap_or(0),
            output: u["output_tokens"].as_i64().unwrap_or(0),
            thinking: u["output_tokens_details"]["thinking_tokens"].as_i64().unwrap_or(0),
            answer: join_blocks(&v, "text", "text"),
            summary: join_blocks(&v, "thinking", "thinking"),
            stop: v["stop_reason"].as_str().unwrap_or("?").to_string(),
        }
    }
}
fn curl_post(key: &str, body: &str) -> Value {
    let mut child = Command::new("curl")
        .args(["-sS", "--max-time", "600", "https://api.anthropic.com/v1/messages",
               "-H", &format!("x-api-key: {key}"), "-H", "anthropic-version: 2023-06-01",
               "-H", "content-type: application/json", "--data-binary", "@-"])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("spawn curl");
    child.stdin.take().unwrap().write_all(body.as_bytes()).unwrap();
    let out = child.wait_with_output().expect("curl wait");
    serde_json::from_slice(&out.stdout).unwrap_or_else(|e| json!({"type":"error","error":{"message": format!("parse fail: {e}; raw={}", String::from_utf8_lossy(&out.stdout))}}))
}
fn join_blocks(v: &Value, ty: &str, field: &str) -> String {
    v["content"].as_array().map(|a| a.iter().filter(|b| b["type"] == ty).filter_map(|b| b[field].as_str()).collect::<Vec<_>>().join("")).unwrap_or_default()
}

// ---- tasks ---------------------------------------------------------------
#[derive(Clone)]
struct Task { suite: String, domain: String, id: String, kind: String, instruction: String, prompt: String, gold: String, test: String, entry: String }
fn load_tasks(root: &std::path::Path) -> Vec<Task> {
    let txt = std::fs::read_to_string(root.join("data/tasks.json")).expect("data/tasks.json (run scripts/fetch_evals.py)");
    let v: Value = serde_json::from_str(&txt).expect("tasks.json parse");
    let s = |x: &Value, k: &str| x[k].as_str().unwrap_or("").to_string();
    v.as_array().unwrap().iter().map(|t| Task {
        suite: s(t,"suite"), domain: s(t,"domain"), id: s(t,"id"), kind: s(t,"kind"),
        instruction: s(t,"instruction"), prompt: s(t,"prompt"), gold: s(t,"gold"), test: s(t,"test"), entry: s(t,"entry_point"),
    }).collect()
}

// ---- grading -------------------------------------------------------------
fn grade(t: &Task, answer: &str) -> bool {
    match t.kind.as_str() {
        "numeric" => last_number(answer).map_or(false, |a| norm_num(&a) == norm_num(&t.gold)),
        "letter"  => first_choice(answer).map_or(false, |c| c == t.gold),
        "exact"   => { let (a, g) = (norm_math(answer), norm_math(&t.gold)); !g.is_empty() && (a == g || a.contains(&g)) },
        "code"    => grade_code(answer, &t.test, &t.entry),
        "stdio"   => grade_stdio(answer, &t.test),
        _ => false,
    }
}
fn norm_num(s: &str) -> String { let t = s.trim().trim_start_matches('+').replace(',', ""); let t = t.trim_start_matches('0'); if t.is_empty() { "0".into() } else { t.to_string() } }
fn last_number(s: &str) -> Option<String> {
    let mut nums = vec![]; let mut cur = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() || (c == '-' && cur.is_empty()) || c == ',' { cur.push(c); }
        else { if cur.chars().any(|d| d.is_ascii_digit()) { nums.push(cur.clone()); } cur.clear(); }
    }
    if cur.chars().any(|d| d.is_ascii_digit()) { nums.push(cur); }
    nums.pop()
}
fn first_choice(s: &str) -> Option<String> {
    let cs: Vec<char> = s.chars().collect();
    for (i, &c) in cs.iter().enumerate() {
        if ('A'..='J').contains(&c) {
            let prev_ok = i == 0 || !cs[i-1].is_alphabetic();
            let next_ok = i+1 >= cs.len() || !cs[i+1].is_alphabetic();
            if prev_ok && next_ok { return Some(c.to_string()); }
        }
    }
    None
}
fn norm_math(s: &str) -> String {
    let mut x = s.to_lowercase();
    for p in ["\\left","\\right","\\!","\\,","\\;","\\:","\\big","\\bigg","$","{","}"," ","\n","\t"] { x = x.replace(p, ""); }
    x = x.replace("\\dfrac","\\frac").replace("\\pi","pi").replace("\\cdot","*");
    x.trim_matches(|c| c=='.'||c=='('||c==')').to_string()
}
static CTR: AtomicUsize = AtomicUsize::new(0);
fn grade_code(answer: &str, test: &str, entry: &str) -> bool {
    let code = strip_fences(answer);
    if !code.contains("def ") { return false; }
    let program = format!("{code}\n\n{test}\n\ncheck({entry})\n");
    let path = std::env::temp_dir().join(format!("skillbench_he_{}.py", CTR.fetch_add(1, Ordering::SeqCst)));
    if std::fs::write(&path, program).is_err() { return false; }
    let mut child = match Command::new("python3").arg(&path).stdout(Stdio::null()).stderr(Stdio::null()).spawn() { Ok(c) => c, Err(_) => return false };
    let mut ok = false;
    for _ in 0..150 { match child.try_wait() { Ok(Some(st)) => { ok = st.success(); break; } Ok(None) => std::thread::sleep(std::time::Duration::from_millis(100)), Err(_) => break } }
    let _ = child.kill(); let _ = std::fs::remove_file(&path);
    ok
}
fn strip_fences(s: &str) -> String { s.lines().filter(|l| !l.trim_start().starts_with("```")).collect::<Vec<_>>().join("\n") }
fn norm_out(s: &str) -> String { s.lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n").trim_end().to_string() }
fn io_str(v: &Value) -> String {
    match v { Value::String(s) => s.clone(), Value::Array(a) => a.iter().map(io_str).collect::<Vec<_>>().join("\n"), other => other.to_string() }
}
// APPS-style: run candidate program on each stdin, compare normalized stdout. All must pass.
fn grade_stdio(answer: &str, tests_json: &str) -> bool {
    let code = strip_fences(answer);
    if code.trim().is_empty() { return false; }
    let v: Value = match serde_json::from_str(tests_json) { Ok(v) => v, Err(_) => return false };
    let ins = v["inputs"].as_array().cloned().unwrap_or_default();
    let outs = v["outputs"].as_array().cloned().unwrap_or_default();
    if ins.is_empty() { return false; }
    let path = std::env::temp_dir().join(format!("skillbench_apps_{}.py", CTR.fetch_add(1, Ordering::SeqCst)));
    if std::fs::write(&path, &code).is_err() { return false; }
    let mut ok = true;
    for (i, inp) in ins.iter().enumerate() {
        let stdin_s = io_str(inp);
        let exp = norm_out(&outs.get(i).map(io_str).unwrap_or_default());
        let mut child = match Command::new("python3").arg(&path)
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn() { Ok(c) => c, Err(_) => { ok = false; break; } };
        if let Some(mut si) = child.stdin.take() { let s = stdin_s.clone(); std::thread::spawn(move || { let _ = si.write_all(s.as_bytes()); }); }
        let mut finished = false;
        for _ in 0..120 { match child.try_wait() { Ok(Some(_)) => { finished = true; break; } Ok(None) => std::thread::sleep(std::time::Duration::from_millis(50)), Err(_) => break } }
        if !finished { let _ = child.kill(); ok = false; break; }
        let got = child.wait_with_output().map(|o| norm_out(&String::from_utf8_lossy(&o.stdout))).unwrap_or_default();
        if got != exp { ok = false; break; }
    }
    let _ = std::fs::remove_file(&path);
    ok
}

// ---- samples + algorithm -------------------------------------------------
#[derive(Clone)]
struct Sample { suite: String, domain: String, id: String, arm: String, rep: usize, thinking: i64, output: i64, stop: String, correct: bool }
fn run_benchmark<T: Llm>(llm: &T, tasks: &[Task], arms: &[&str], skill: &str, reps: usize) -> Vec<Sample> {
    let mut out = Vec::new();
    for t in tasks {
        for &arm in arms {
            for rep in 1..=reps {
                let user = format!("{}\n\n{}", t.prompt, t.instruction);
                let r = llm.complete(&Req { system: arm_sys(arm, skill), user, display: "omitted".into() });
                let correct = grade(t, &r.answer);
                println!("  {:<13} {:<9} r{} out={:<5} think={:<5} stop={} correct={}", t.id, arm, rep, r.output, r.thinking, r.stop, correct);
                out.push(Sample { suite: t.suite.clone(), domain: t.domain.clone(), id: t.id.clone(), arm: arm.into(), rep, thinking: r.thinking, output: r.output, stop: r.stop, correct });
            }
        }
    }
    out
}

fn median(v: &[i64]) -> f64 { let mut v = v.to_vec(); v.sort(); let n = v.len(); if n == 0 { 0.0 } else if n%2==1 { v[n/2] as f64 } else { (v[n/2-1]+v[n/2]) as f64/2.0 } }
fn mean(v: &[i64]) -> f64 { if v.is_empty() { 0.0 } else { v.iter().sum::<i64>() as f64 / v.len() as f64 } }
fn task_ids(s: &[Sample]) -> Vec<(String, String)> { let mut o: Vec<(String,String)> = vec![]; for x in s { if !o.iter().any(|(i,_)| i==&x.id) { o.push((x.id.clone(), x.suite.clone())); } } o }
fn t_think(s: &[Sample], id: &str, arm: &str) -> Vec<i64> { s.iter().filter(|x| x.id==id && x.arm==arm).map(|x| x.thinking).collect() }
fn t_acc(s: &[Sample], id: &str, arm: &str) -> (usize, usize) { let v: Vec<&Sample> = s.iter().filter(|x| x.id==id && x.arm==arm).collect(); (v.iter().filter(|x| x.correct).count(), v.len()) }
fn arm_out(s: &[Sample], arm: &str) -> Vec<i64> { s.iter().filter(|x| x.arm==arm).map(|x| x.output).collect() }
fn arm_think(s: &[Sample], arm: &str) -> Vec<i64> { s.iter().filter(|x| x.arm==arm).map(|x| x.thinking).collect() }
fn arm_suite_out(s: &[Sample], arm: &str, suite: &str) -> Vec<i64> { s.iter().filter(|x| x.arm==arm && x.suite==suite).map(|x| x.output).collect() }
fn arm_suite_acc(s: &[Sample], arm: &str, suite: &str) -> (usize, usize) { let v: Vec<&Sample> = s.iter().filter(|x| x.arm==arm && x.suite==suite).collect(); (v.iter().filter(|x| x.correct).count(), v.len()) }
fn arm_acc(s: &[Sample], arm: &str) -> (usize, usize) { let v: Vec<&Sample> = s.iter().filter(|x| x.arm==arm).collect(); (v.iter().filter(|x| x.correct).count(), v.len()) }

fn load_samples(root: &std::path::Path) -> Vec<Sample> {
    let txt = std::fs::read_to_string(root.join("data/samples.json")).expect("data/samples.json");
    let v: Value = serde_json::from_str(&txt).unwrap();
    let s = |x: &Value, k: &str| x[k].as_str().unwrap_or("").to_string();
    v.as_array().unwrap().iter().map(|x| {
        Sample { suite: s(x,"suite"), domain: s(x,"domain"), id: s(x,"id"), arm: s(x,"arm"),
            rep: x["rep"].as_u64().unwrap_or(0) as usize, thinking: x["thinking_tokens"].as_i64().unwrap_or(0),
            output: x["output_tokens"].as_i64().unwrap_or(0), stop: s(x,"stop_reason"), correct: x["correct"].as_bool().unwrap_or(false) }
    }).collect()
}
fn save_samples(root: &std::path::Path, s: &[Sample]) {
    let arr: Vec<Value> = s.iter().map(|x| json!({"suite":x.suite,"domain":x.domain,"id":x.id,"arm":x.arm,"rep":x.rep,"thinking_tokens":x.thinking,"output_tokens":x.output,"stop_reason":x.stop,"correct":x.correct})).collect();
    std::fs::write(root.join("data/samples.json"), serde_json::to_string_pretty(&arr).unwrap()).unwrap();
}

fn build_report(samples: &[Sample], reps: usize) -> String {
    let arms: Vec<&str> = ARMS.iter().cloned().filter(|a| samples.iter().any(|s| s.arm == *a)).collect();
    let mut suites: Vec<(String, String)> = vec![];
    for x in samples { if !suites.iter().any(|(s,_)| s==&x.suite) { suites.push((x.suite.clone(), x.domain.clone())); } }
    let base_out: f64 = median(&arm_out(samples, "baseline"));

    let mut r = String::new();
    r.push_str("# skillbench — caveman SKILL.md, head to head vs baseline\n\n");
    r.push_str(&format!("**Model:** `{MODEL}` · **effort:** `{EFFORT}` · **thinking:** adaptive · **reps/cell:** {reps} · **n samples:** {}\n\n", samples.len()));
    r.push_str("## Question\n\nThe caveman skill claims it cuts billed output ~75% while keeping full technical accuracy. **Holding the model, effort, task and answer format fixed, does adding the caveman `SKILL.md` to the system prompt change billed output tokens, and does accuracy survive?**\n\n");
    r.push_str("## Arms\n\n| arm | system prompt |\n|---|---|\n");
    r.push_str("| `baseline` | neutral solver prompt only |\n");
    r.push_str("| `caveman` | same prompt + the full `skills/caveman/SKILL.md` body (YAML frontmatter stripped) |\n\n");
    r.push_str("**Evals:** AIME + Hendrycks MATH level-5 (math); MMLU-Pro computer science + HumanEval (cs). Metric = API `usage.output_tokens` (full billed generation; `thinking_tokens` shown alongside). Only the system prompt differs across arms.\n\n");
    r.push_str("> **Limits:** these splits force a terse final answer (bare integer / letter / function body), so there is little prose for caveman to compress and HumanEval output is code (\"code blocks unchanged\" by the skill's own rule). This bench therefore measures whether the SKILL.md is *safe* — accuracy preserved, output not bloated — not the README's ~75% prose figure, which is measured on explanatory prompts in `benchmarks/`. Raw chain of thought is never returned; we measure its billed footprint, not its content.\n\n");

    r.push_str("## Results — median output tokens, by arm\n\n");
    r.push_str("| arm | median output | median thinking | Δ output vs baseline | accuracy |\n|---|---:|---:|---:|:--:|\n");
    for &a in &arms {
        let med = median(&arm_out(samples, a));
        let medt = median(&arm_think(samples, a));
        let d = if a == "baseline" || base_out == 0.0 { 0.0 } else { (med - base_out) / base_out * 100.0 };
        let (c, n) = arm_acc(samples, a);
        let dcol = if a == "baseline" { "—".to_string() } else { format!("{:+.0}%", d) };
        r.push_str(&format!("| `{}` | {:.0} | {:.0} | {} | {}/{} |\n", a, med, medt, dcol, c, n));
    }
    r.push('\n');

    r.push_str("## Median output tokens by suite\n\n| suite | domain |");
    for &a in &arms { r.push_str(&format!(" {} |", a)); }
    r.push_str("\n|---|---|"); for _ in &arms { r.push_str("--:|"); } r.push('\n');
    for (su, dom) in &suites {
        r.push_str(&format!("| `{}` | {} |", su, dom));
        for &a in &arms { r.push_str(&format!(" {:.0} |", median(&arm_suite_out(samples, a, su)))); }
        r.push('\n');
    }
    r.push('\n');

    r.push_str("## Accuracy by suite\n\n| suite |");
    for &a in &arms { r.push_str(&format!(" {} |", a)); }
    r.push_str("\n|---|"); for _ in &arms { r.push_str(":--:|"); } r.push('\n');
    for (su, _) in &suites {
        r.push_str(&format!("| `{}` |", su));
        for &a in &arms { let (c,n) = arm_suite_acc(samples, a, su); r.push_str(&format!(" {}/{} |", c, n)); }
        r.push('\n');
    }
    r.push('\n');

    // poison-thinking check: on tasks where baseline genuinely reasons, does caveman
    // cut billed thinking, and does accuracy survive? (cut thinking + dropped accuracy = poison)
    let active: Vec<(String, String)> = task_ids(samples).into_iter()
        .filter(|(id,_)| mean(&t_think(samples, id, "baseline")) >= 20.0).collect();
    r.push_str("## Does caveman poison thinking?\n\n");
    if active.is_empty() {
        r.push_str("No task had baseline mean thinking ≥ 20 — nothing reasoned hard enough to test. Add harder items.\n\n");
    } else {
        r.push_str(&format!("Restricted to the **{} tasks the baseline actually reasons through** (mean baseline thinking ≥ 20). If caveman cuts thinking here *and* loses accuracy, the terseness instruction is bleeding into private reasoning.\n\n", active.len()));
        r.push_str("| task | suite | base think | cave think | Δ think | base acc | cave acc |\n|---|---|--:|--:|--:|:--:|:--:|\n");
        let (mut bsum, mut csum) = (0.0_f64, 0.0_f64);
        let (mut bc, mut bn, mut cc, mut cn) = (0usize, 0usize, 0usize, 0usize);
        for (id, su) in &active {
            let bt = mean(&t_think(samples, id, "baseline"));
            let ct = mean(&t_think(samples, id, "caveman"));
            let d = if bt > 0.0 { (ct - bt) / bt * 100.0 } else { 0.0 };
            let (ba, bnn) = t_acc(samples, id, "baseline");
            let (ca, cnn) = t_acc(samples, id, "caveman");
            bsum += bt; csum += ct; bc += ba; bn += bnn; cc += ca; cn += cnn;
            r.push_str(&format!("| `{}` | {} | {:.0} | {:.0} | {:+.0}% | {}/{} | {}/{} |\n", id, su, bt, ct, d, ba, bnn, ca, cnn));
        }
        let dsum = if bsum > 0.0 { (csum - bsum) / bsum * 100.0 } else { 0.0 };
        r.push_str(&format!("| **{} active** | | **{:.0}** | **{:.0}** | **{:+.0}%** | **{}/{}** | **{}/{}** |\n\n", active.len(), bsum, csum, dsum, bc, bn, cc, cn));
        let acc_drop = (bc as f64 / bn.max(1) as f64) - (cc as f64 / cn.max(1) as f64);
        let verdict = if dsum <= -15.0 && acc_drop > 0.10 {
            "**Poison signal:** caveman cut billed thinking on reasoning-active tasks AND lost accuracy there — the terseness is leaking into private reasoning."
        } else if dsum <= -15.0 {
            "Caveman cut billed thinking on reasoning-active tasks but accuracy held — compression without measurable poison (on this small sample)."
        } else {
            "No poison signal: caveman did not systematically cut billed thinking on reasoning-active tasks."
        };
        r.push_str(verdict); r.push_str("\n\n");
    }

    // verdict
    r.push_str("## Verdict\n\n");
    let (ba, bn) = arm_acc(samples, "baseline");
    let (ca, cn) = arm_acc(samples, "caveman");
    let cav_out = median(&arm_out(samples, "caveman"));
    let d = if base_out > 0.0 { (cav_out - base_out) / base_out * 100.0 } else { 0.0 };
    r.push_str(&format!("Median billed output: baseline {:.0} → caveman {:.0} ({:+.0}%). ", base_out, cav_out, d));
    r.push_str(&format!("Accuracy: baseline {}/{}, caveman {}/{}.\n\n", ba, bn, ca, cn));
    let wrong = samples.iter().filter(|s| !s.correct).count();
    r.push_str(&format!("Overall accuracy {}/{}. Caveats: AIME / MMLU-Pro-CS are largely one-shot (output≈answer, so they barely separate the arms); MATH-L5 carries most of the output signal; MATH grading is normalized-string (approximate); HumanEval output is code the skill leaves unchanged by rule. The headline ~75% compression is a *prose* claim measured separately — this bench's job is to show the SKILL.md neither breaks accuracy nor inflates token cost on hard answer-only evals.\n", samples.len()-wrong, samples.len()));

    r.push_str("\n## Per-rep detail\n\n| id | suite | arm | rep | output | thinking | stop | ok |\n|---|---|---|--:|--:|--:|---|:--:|\n");
    for s in samples { r.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n", s.id, s.suite, s.arm, s.rep, s.output, s.thinking, s.stop, if s.correct {"✓"} else {"✗"})); }
    r
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "run".into());
    let reps: usize = std::env::var("REPS").ok().and_then(|s| s.parse().ok()).unwrap_or(2);
    let root = std::path::PathBuf::from(std::env::var("SKILLBENCH_ROOT").unwrap_or_else(|_| ".".into()))
        .canonicalize().unwrap_or_else(|_| std::path::PathBuf::from("."));

    match mode.as_str() {
        "mock" => {
            println!("== MOCK run ==");
            let tasks = load_tasks(&root);
            let skill = skill_body(&root);
            let s = run_benchmark(&MockLlm, &tasks, &ARMS, &skill, reps);
            for &a in &ARMS { println!("  {:<9} Σout={}", a, arm_out(&s, a).iter().sum::<i64>()); }
            assert!(arm_out(&s, "caveman").iter().sum::<i64>() < arm_out(&s, "baseline").iter().sum::<i64>(), "mock should compress");
            assert!(!skill.is_empty() && skill.contains("caveman"), "skill body must load");
            println!("mock OK ({} samples, {} arms, skill {} chars) — wiring + grading + report verified", s.len(), ARMS.len(), skill.len());
            let _ = build_report(&s, reps);
        }
        "report" => {
            let s = load_samples(&root);
            std::fs::write(root.join("REPORT.md"), build_report(&s, s.iter().map(|x| x.rep).max().unwrap_or(0))).unwrap();
            println!("regenerated REPORT.md from {} samples (no API)", s.len());
        }
        _ => { // run: both arms fresh
            let tasks = load_tasks(&root);
            let skill = skill_body(&root);
            println!("== REAL run ({MODEL}, effort={EFFORT}, reps={reps}, tasks={}, arms={:?}, skill={} chars) ==", tasks.len(), ARMS, skill.len());
            let llm = RealLlm::new();
            let samples = run_benchmark(&llm, &tasks, &ARMS, &skill, reps);
            save_samples(&root, &samples);
            std::fs::write(root.join("REPORT.md"), build_report(&samples, reps)).unwrap();
            println!("wrote {} samples -> data/samples.json + REPORT.md", samples.len());
        }
    }
}
