# skillbench — caveman SKILL.md, head to head vs baseline

**Model:** `claude-opus-4-8` · **effort:** `high` · **thinking:** adaptive · **reps/cell:** 2 · **n samples:** 64

## Question

The caveman skill claims it cuts billed output ~75% while keeping full technical accuracy. **Holding the model, effort, task and answer format fixed, does adding the caveman `SKILL.md` to the system prompt change billed output tokens, and does accuracy survive?**

## Arms

| arm | system prompt |
|---|---|
| `baseline` | neutral solver prompt only |
| `caveman` | same prompt + the full `skills/caveman/SKILL.md` body (YAML frontmatter stripped) |

**Evals:** AIME + Hendrycks MATH level-5 (math); MMLU-Pro computer science + HumanEval (cs). Metric = API `usage.output_tokens` (full billed generation; `thinking_tokens` shown alongside). Only the system prompt differs across arms.

> **Limits:** these splits force a terse final answer (bare integer / letter / function body), so there is little prose for caveman to compress and HumanEval output is code ("code blocks unchanged" by the skill's own rule). This bench therefore measures whether the SKILL.md is *safe* — accuracy preserved, output not bloated — not the README's ~75% prose figure, which is measured on explanatory prompts in `benchmarks/`. Raw chain of thought is never returned; we measure its billed footprint, not its content.

## Results — median output tokens, by arm

| arm | median output | median thinking | Δ output vs baseline | accuracy |
|---|---:|---:|---:|:--:|
| `baseline` | 38 | 0 | — | 27/32 |
| `caveman` | 14 | 0 | -61% | 26/32 |

## Median output tokens by suite

| suite | domain | baseline | caveman |
|---|---|--:|--:|
| `aime` | math | 3 | 7 |
| `math5` | math | 130 | 69 |
| `mmlu_pro_cs` | cs | 3 | 3 |
| `humaneval` | cs | 210 | 213 |

## Accuracy by suite

| suite | baseline | caveman |
|---|:--:|:--:|
| `aime` | 4/8 | 6/8 |
| `math5` | 8/8 | 6/8 |
| `mmlu_pro_cs` | 8/8 | 6/8 |
| `humaneval` | 7/8 | 8/8 |

## Verdict

Median billed output: baseline 38 → caveman 14 (-61%). Accuracy: baseline 27/32, caveman 26/32.

Overall accuracy 53/64. Caveats: AIME / MMLU-Pro-CS are largely one-shot (output≈answer, so they barely separate the arms); MATH-L5 carries most of the output signal; MATH grading is normalized-string (approximate); HumanEval output is code the skill leaves unchanged by rule. The headline ~75% compression is a *prose* claim measured separately — this bench's job is to show the SKILL.md neither breaks accuracy nor inflates token cost on hard answer-only evals.

## Per-rep detail

| id | suite | arm | rep | output | thinking | stop | ok |
|---|---|---|--:|--:|--:|---|:--:|
| aime-0 | aime | baseline | 1 | 3 | 0 | end_turn | ✓ |
| aime-0 | aime | baseline | 2 | 3 | 0 | end_turn | ✓ |
| aime-0 | aime | caveman | 1 | 3 | 0 | end_turn | ✓ |
| aime-0 | aime | caveman | 2 | 3 | 0 | end_turn | ✓ |
| aime-1 | aime | baseline | 1 | 3 | 0 | end_turn | ✓ |
| aime-1 | aime | baseline | 2 | 3 | 0 | end_turn | ✓ |
| aime-1 | aime | caveman | 1 | 3 | 0 | end_turn | ✓ |
| aime-1 | aime | caveman | 2 | 3 | 0 | end_turn | ✓ |
| aime-2 | aime | baseline | 1 | 3 | 0 | end_turn | ✗ |
| aime-2 | aime | baseline | 2 | 3 | 0 | end_turn | ✗ |
| aime-2 | aime | caveman | 1 | 11 | 8 | end_turn | ✓ |
| aime-2 | aime | caveman | 2 | 11 | 8 | end_turn | ✓ |
| aime-3 | aime | baseline | 1 | 571 | 568 | end_turn | ✗ |
| aime-3 | aime | baseline | 2 | 378 | 375 | end_turn | ✗ |
| aime-3 | aime | caveman | 1 | 252 | 249 | end_turn | ✗ |
| aime-3 | aime | caveman | 2 | 253 | 250 | end_turn | ✗ |
| math5-0 | math5 | baseline | 1 | 127 | 120 | end_turn | ✓ |
| math5-0 | math5 | baseline | 2 | 134 | 127 | end_turn | ✓ |
| math5-0 | math5 | caveman | 1 | 18 | 11 | end_turn | ✓ |
| math5-0 | math5 | caveman | 2 | 19 | 12 | end_turn | ✓ |
| math5-1 | math5 | baseline | 1 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | baseline | 2 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| math5-2 | math5 | baseline | 1 | 804 | 789 | end_turn | ✓ |
| math5-2 | math5 | baseline | 2 | 2566 | 2551 | end_turn | ✓ |
| math5-2 | math5 | caveman | 1 | 1010 | 1005 | end_turn | ✗ |
| math5-2 | math5 | caveman | 2 | 838 | 832 | end_turn | ✗ |
| math5-3 | math5 | baseline | 1 | 72 | 69 | end_turn | ✓ |
| math5-3 | math5 | baseline | 2 | 142 | 139 | end_turn | ✓ |
| math5-3 | math5 | caveman | 1 | 119 | 116 | end_turn | ✓ |
| math5-3 | math5 | caveman | 2 | 135 | 132 | end_turn | ✓ |
| mmlupro-cs-0 | mmlu_pro_cs | baseline | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-0 | mmlu_pro_cs | baseline | 2 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-0 | mmlu_pro_cs | caveman | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-0 | mmlu_pro_cs | caveman | 2 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-1 | mmlu_pro_cs | baseline | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-1 | mmlu_pro_cs | baseline | 2 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-1 | mmlu_pro_cs | caveman | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-1 | mmlu_pro_cs | caveman | 2 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-2 | mmlu_pro_cs | baseline | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-2 | mmlu_pro_cs | baseline | 2 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-2 | mmlu_pro_cs | caveman | 1 | 3 | 0 | end_turn | ✗ |
| mmlupro-cs-2 | mmlu_pro_cs | caveman | 2 | 3 | 0 | end_turn | ✗ |
| mmlupro-cs-3 | mmlu_pro_cs | baseline | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-3 | mmlu_pro_cs | baseline | 2 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-3 | mmlu_pro_cs | caveman | 1 | 3 | 0 | end_turn | ✓ |
| mmlupro-cs-3 | mmlu_pro_cs | caveman | 2 | 3 | 0 | end_turn | ✓ |
| HumanEval_0 | humaneval | baseline | 1 | 221 | 0 | end_turn | ✓ |
| HumanEval_0 | humaneval | baseline | 2 | 221 | 0 | end_turn | ✓ |
| HumanEval_0 | humaneval | caveman | 1 | 225 | 0 | end_turn | ✓ |
| HumanEval_0 | humaneval | caveman | 2 | 225 | 0 | end_turn | ✓ |
| HumanEval_1 | humaneval | baseline | 1 | 284 | 6 | end_turn | ✓ |
| HumanEval_1 | humaneval | baseline | 2 | 303 | 6 | end_turn | ✓ |
| HumanEval_1 | humaneval | caveman | 1 | 292 | 9 | end_turn | ✓ |
| HumanEval_1 | humaneval | caveman | 2 | 291 | 8 | end_turn | ✓ |
| HumanEval_2 | humaneval | baseline | 1 | 121 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | baseline | 2 | 121 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | caveman | 1 | 121 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | caveman | 2 | 121 | 0 | end_turn | ✓ |
| HumanEval_3 | humaneval | baseline | 1 | 194 | 6 | end_turn | ✗ |
| HumanEval_3 | humaneval | baseline | 2 | 200 | 6 | end_turn | ✓ |
| HumanEval_3 | humaneval | caveman | 1 | 201 | 7 | end_turn | ✓ |
| HumanEval_3 | humaneval | caveman | 2 | 201 | 7 | end_turn | ✓ |
