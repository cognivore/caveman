# skillbench

Token-I/O benchmark for one question about the caveman skill:

> Holding model, effort, task and answer-format fixed, does adding the caveman
> **`SKILL.md`** to the system prompt change the **billed output footprint**, and
> does **accuracy survive**?

Two arms, identical except the system prompt:

| arm | system prompt |
|---|---|
| `baseline` | neutral solver prompt only |
| `caveman` | same prompt + the full `skills/caveman/SKILL.md` body (frontmatter stripped) |

The caveman skill body is read live from `../skills/caveman/SKILL.md` — this benches
the real shipped behavior file, nothing hand-copied.

## Evals

Tasks are the first N items (deterministic) of established eval libraries, pulled by
`scripts/fetch_evals.py` (HuggingFace datasets-server, no auth) into `data/tasks.json`:

| suite | domain | source | grading |
|---|---|---|---|
| `aime` | math | AI-MO AIME (competition, integer answers) | numeric |
| `math5` | math | Hendrycks MATH **level 5** (hardest tier) | exact (normalized LaTeX, approximate) |
| `mmlu_pro_cs` | cs | MMLU-Pro computer science (10-option reasoning MC) | letter |
| `humaneval` | cs | OpenAI HumanEval (code) | **execution** (assemble + run unit test) |

## Run

```sh
nix develop                             # devShell: rust-script + cargo + rustc + curl + jq + python3
N=4 python3 scripts/fetch_evals.py      # pull eval items -> data/tasks.json
rust-script bench.rs mock               # no API — checks wiring + grading + skill load + aggregation
rust-script bench.rs run                # real — Opus 4.8 via the key from rageveil
rust-script bench.rs report             # regenerate REPORT.md from saved samples, no API
# or: nix run .#mock  /  nix run .#bench
```

Results land in [`REPORT.md`](./REPORT.md) (generated from data, never hand-typed) and
raw evidence in `data/samples.json`.

## Design — final tagless

The DSL and the interpreter are separate, so the aggregation algorithm is developed and
checked against a pure mock before a single real token is spent.

```
trait Llm                 # the DSL: one op, complete(req) -> resp
  MockLlm                 # interpreter 1: pure fake (bench.rs mock)
  RealLlm                 # interpreter 2: impure boundary — shells curl + rageveil
fn run_benchmark<T: Llm>  # the algorithm: written once, runs on either interpreter
```

The whole impure surface (network, secret) is `RealLlm`. The key is read at runtime via
`rageveil show geosurge.ai/api.anthropic.com/onehr-cellvm/pool` and never written down.

## Measurement & honest limit

Metric is the API's own `usage.output_tokens` (full billed generation), with
`usage.output_tokens_details.thinking_tokens` reported alongside. Model, effort, prompt
and answer format are identical across arms; the only variable is the caveman `SKILL.md`.

**Limit:** these eval splits force a terse final answer (a bare integer, letter, or
function body), so there is little prose for caveman to compress, and HumanEval output is
code the skill leaves unchanged by its own rule. This bench therefore measures whether the
SKILL.md is *safe* — accuracy preserved, output not bloated — not the README's headline
~75% prose-compression figure, which is measured on explanatory prompts in `benchmarks/`.
Borrowed wholesale from the [`alan`](../) illegible-thinking harness; same structure, new arm.
