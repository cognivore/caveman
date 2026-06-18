# skillbench — caveman SKILL.md, head to head vs baseline

**Model:** `claude-opus-4-8` · **effort:** `high` · **thinking:** adaptive · **reps/cell:** 2 · **n samples:** 128

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
| `baseline` | 71 | 6 | — | 60/64 |
| `caveman` | 56 | 6 | -20% | 53/64 |

## Median output tokens by suite

| suite | domain | baseline | caveman |
|---|---|--:|--:|
| `aime` | math | 3 | 7 |
| `math5` | math | 71 | 46 |
| `math500` | math | 42 | 74 |
| `aime25` | math | 98 | 214 |
| `mmlu_pro_cs` | cs | 3 | 3 |
| `humaneval` | cs | 211 | 212 |
| `gpqa_diamond` | sci | 69 | 3 |
| `hle` | any | 1082 | 968 |

## Accuracy by suite

| suite | baseline | caveman |
|---|:--:|:--:|
| `aime` | 7/8 | 6/8 |
| `math5` | 7/8 | 6/8 |
| `math500` | 8/8 | 6/8 |
| `aime25` | 8/8 | 8/8 |
| `mmlu_pro_cs` | 8/8 | 6/8 |
| `humaneval` | 8/8 | 8/8 |
| `gpqa_diamond` | 8/8 | 8/8 |
| `hle` | 6/8 | 5/8 |

## Verdict

Median billed output: baseline 71 → caveman 56 (-20%). Accuracy: baseline 60/64, caveman 53/64.

Overall accuracy 113/128. Caveats: AIME / MMLU-Pro-CS are largely one-shot (output≈answer, so they barely separate the arms); MATH-L5 carries most of the output signal; MATH grading is normalized-string (approximate); HumanEval output is code the skill leaves unchanged by rule. The headline ~75% compression is a *prose* claim measured separately — this bench's job is to show the SKILL.md neither breaks accuracy nor inflates token cost on hard answer-only evals.

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
| aime-2 | aime | baseline | 2 | 3 | 0 | end_turn | ✓ |
| aime-2 | aime | caveman | 1 | 11 | 8 | end_turn | ✓ |
| aime-2 | aime | caveman | 2 | 11 | 8 | end_turn | ✓ |
| aime-3 | aime | baseline | 1 | 832 | 829 | end_turn | ✓ |
| aime-3 | aime | baseline | 2 | 631 | 628 | end_turn | ✓ |
| aime-3 | aime | caveman | 1 | 465 | 462 | end_turn | ✗ |
| aime-3 | aime | caveman | 2 | 564 | 561 | end_turn | ✗ |
| math5-0 | math5 | baseline | 1 | 31 | 24 | end_turn | ✓ |
| math5-0 | math5 | baseline | 2 | 132 | 125 | end_turn | ✓ |
| math5-0 | math5 | caveman | 1 | 20 | 13 | end_turn | ✓ |
| math5-0 | math5 | caveman | 2 | 19 | 12 | end_turn | ✓ |
| math5-1 | math5 | baseline | 1 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | baseline | 2 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| math5-2 | math5 | baseline | 1 | 873 | 858 | end_turn | ✓ |
| math5-2 | math5 | baseline | 2 | 798 | 793 | end_turn | ✗ |
| math5-2 | math5 | caveman | 1 | 1083 | 1078 | end_turn | ✗ |
| math5-2 | math5 | caveman | 2 | 859 | 854 | end_turn | ✗ |
| math5-3 | math5 | baseline | 1 | 70 | 67 | end_turn | ✓ |
| math5-3 | math5 | baseline | 2 | 72 | 69 | end_turn | ✓ |
| math5-3 | math5 | caveman | 1 | 72 | 69 | end_turn | ✓ |
| math5-3 | math5 | caveman | 2 | 131 | 128 | end_turn | ✓ |
| math500-0 | math500 | baseline | 1 | 23 | 0 | end_turn | ✓ |
| math500-0 | math500 | baseline | 2 | 23 | 0 | end_turn | ✓ |
| math500-0 | math500 | caveman | 1 | 41 | 18 | end_turn | ✓ |
| math500-0 | math500 | caveman | 2 | 41 | 18 | end_turn | ✓ |
| math500-1 | math500 | baseline | 1 | 5 | 0 | end_turn | ✓ |
| math500-1 | math500 | baseline | 2 | 138 | 131 | end_turn | ✓ |
| math500-1 | math500 | caveman | 1 | 117 | 110 | end_turn | ✓ |
| math500-1 | math500 | caveman | 2 | 122 | 115 | end_turn | ✓ |
| math500-2 | math500 | baseline | 1 | 115 | 102 | end_turn | ✓ |
| math500-2 | math500 | baseline | 2 | 117 | 102 | end_turn | ✓ |
| math500-2 | math500 | caveman | 1 | 106 | 101 | end_turn | ✗ |
| math500-2 | math500 | caveman | 2 | 106 | 101 | end_turn | ✗ |
| math500-3 | math500 | baseline | 1 | 42 | 39 | end_turn | ✓ |
| math500-3 | math500 | baseline | 2 | 42 | 39 | end_turn | ✓ |
| math500-3 | math500 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| math500-3 | math500 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| aime25-0 | aime25 | baseline | 1 | 100 | 97 | end_turn | ✓ |
| aime25-0 | aime25 | baseline | 2 | 97 | 94 | end_turn | ✓ |
| aime25-0 | aime25 | caveman | 1 | 96 | 93 | end_turn | ✓ |
| aime25-0 | aime25 | caveman | 2 | 103 | 100 | end_turn | ✓ |
| aime25-1 | aime25 | baseline | 1 | 3 | 0 | end_turn | ✓ |
| aime25-1 | aime25 | baseline | 2 | 3 | 0 | end_turn | ✓ |
| aime25-1 | aime25 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| aime25-1 | aime25 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| aime25-2 | aime25 | baseline | 1 | 398 | 395 | end_turn | ✓ |
| aime25-2 | aime25 | baseline | 2 | 3 | 0 | end_turn | ✓ |
| aime25-2 | aime25 | caveman | 1 | 326 | 323 | end_turn | ✓ |
| aime25-2 | aime25 | caveman | 2 | 398 | 395 | end_turn | ✓ |
| aime25-3 | aime25 | baseline | 1 | 291 | 288 | end_turn | ✓ |
| aime25-3 | aime25 | baseline | 2 | 313 | 310 | end_turn | ✓ |
| aime25-3 | aime25 | caveman | 1 | 414 | 411 | end_turn | ✓ |
| aime25-3 | aime25 | caveman | 2 | 367 | 364 | end_turn | ✓ |
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
| HumanEval_0 | humaneval | baseline | 2 | 230 | 9 | end_turn | ✓ |
| HumanEval_0 | humaneval | caveman | 1 | 225 | 0 | end_turn | ✓ |
| HumanEval_0 | humaneval | caveman | 2 | 225 | 0 | end_turn | ✓ |
| HumanEval_1 | humaneval | baseline | 1 | 300 | 9 | end_turn | ✓ |
| HumanEval_1 | humaneval | baseline | 2 | 300 | 9 | end_turn | ✓ |
| HumanEval_1 | humaneval | caveman | 1 | 297 | 8 | end_turn | ✓ |
| HumanEval_1 | humaneval | caveman | 2 | 283 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | baseline | 1 | 127 | 6 | end_turn | ✓ |
| HumanEval_2 | humaneval | baseline | 2 | 121 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | caveman | 1 | 121 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | caveman | 2 | 121 | 0 | end_turn | ✓ |
| HumanEval_3 | humaneval | baseline | 1 | 201 | 7 | end_turn | ✓ |
| HumanEval_3 | humaneval | baseline | 2 | 200 | 6 | end_turn | ✓ |
| HumanEval_3 | humaneval | caveman | 1 | 200 | 6 | end_turn | ✓ |
| HumanEval_3 | humaneval | caveman | 2 | 200 | 6 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | baseline | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | baseline | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | caveman | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | caveman | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | baseline | 1 | 209 | 206 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | baseline | 2 | 208 | 205 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | caveman | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | caveman | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | baseline | 1 | 135 | 132 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | baseline | 2 | 139 | 136 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | caveman | 1 | 152 | 149 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | caveman | 2 | 82 | 79 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | baseline | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | baseline | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | caveman | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | caveman | 2 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | baseline | 1 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | baseline | 2 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | caveman | 1 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | caveman | 2 | 3 | 0 | end_turn | ✓ |
| hle-1 | hle | baseline | 1 | 743 | 737 | end_turn | ✓ |
| hle-1 | hle | baseline | 2 | 1047 | 1041 | end_turn | ✓ |
| hle-1 | hle | caveman | 1 | 991 | 985 | end_turn | ✓ |
| hle-1 | hle | caveman | 2 | 1290 | 1284 | end_turn | ✓ |
| hle-2 | hle | baseline | 1 | 19523 | 19467 | end_turn | ✗ |
| hle-2 | hle | baseline | 2 | 30456 | 30439 | end_turn | ✗ |
| hle-2 | hle | caveman | 1 | 30491 | 30477 | end_turn | ✗ |
| hle-2 | hle | caveman | 2 | 23462 | 23447 | end_turn | ✗ |
| hle-3 | hle | baseline | 1 | 1118 | 1115 | end_turn | ✓ |
| hle-3 | hle | baseline | 2 | 1558 | 1553 | end_turn | ✓ |
| hle-3 | hle | caveman | 1 | 944 | 941 | end_turn | ✗ |
| hle-3 | hle | caveman | 2 | 539 | 536 | end_turn | ✓ |
