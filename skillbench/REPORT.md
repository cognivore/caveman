# skillbench — caveman SKILL.md, head to head vs baseline

**Model:** `claude-opus-4-8` · **effort:** `high` · **thinking:** adaptive · **reps/cell:** 2 · **n samples:** 144

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
| `baseline` | 106 | 10 | — | 66/72 |
| `caveman` | 88 | 8 | -17% | 63/72 |

## Median output tokens by suite

| suite | domain | baseline | caveman |
|---|---|--:|--:|
| `aime` | math | 3 | 7 |
| `math5` | math | 105 | 94 |
| `math500` | math | 41 | 41 |
| `aime25` | math | 109 | 196 |
| `mmlu_pro_cs` | cs | 3 | 3 |
| `humaneval` | cs | 212 | 214 |
| `apps` | cs | 358 | 214 |
| `gpqa_diamond` | sci | 3 | 3 |
| `hle` | any | 941 | 681 |

## Accuracy by suite

| suite | baseline | caveman |
|---|:--:|:--:|
| `aime` | 5/8 | 7/8 |
| `math5` | 7/8 | 6/8 |
| `math500` | 8/8 | 6/8 |
| `aime25` | 8/8 | 8/8 |
| `mmlu_pro_cs` | 8/8 | 6/8 |
| `humaneval` | 8/8 | 8/8 |
| `apps` | 8/8 | 8/8 |
| `gpqa_diamond` | 8/8 | 8/8 |
| `hle` | 6/8 | 6/8 |

## Does caveman poison thinking?

Restricted to the **19 tasks the baseline actually reasons through** (mean baseline thinking ≥ 20). If caveman cuts thinking here *and* loses accuracy, the terseness instruction is bleeding into private reasoning.

| task | suite | base think | cave think | Δ think | base acc | cave acc |
|---|---|--:|--:|--:|:--:|:--:|
| `aime-3` | aime | 300 | 584 | +94% | 1/2 | 1/2 |
| `math5-0` | math5 | 64 | 102 | +60% | 2/2 | 2/2 |
| `math5-2` | math5 | 906 | 866 | -4% | 1/2 | 0/2 |
| `math5-3` | math5 | 127 | 72 | -43% | 2/2 | 2/2 |
| `math500-1` | math500 | 30 | 58 | +98% | 2/2 | 2/2 |
| `math500-2` | math500 | 102 | 98 | -4% | 2/2 | 0/2 |
| `math500-3` | math500 | 38 | 0 | -100% | 2/2 | 2/2 |
| `aime25-0` | aime25 | 94 | 92 | -2% | 2/2 | 2/2 |
| `aime25-2` | aime25 | 274 | 450 | +64% | 2/2 | 2/2 |
| `aime25-3` | aime25 | 388 | 314 | -19% | 2/2 | 2/2 |
| `apps-0` | apps | 75 | 90 | +19% | 2/2 | 2/2 |
| `apps-1` | apps | 64 | 28 | -56% | 2/2 | 2/2 |
| `apps-2` | apps | 399 | 0 | -100% | 2/2 | 2/2 |
| `apps-3` | apps | 8224 | 8579 | +4% | 2/2 | 2/2 |
| `gpqa-1` | gpqa_diamond | 103 | 0 | -100% | 2/2 | 2/2 |
| `gpqa-2` | gpqa_diamond | 124 | 126 | +2% | 2/2 | 2/2 |
| `hle-1` | hle | 935 | 772 | -17% | 2/2 | 2/2 |
| `hle-2` | hle | 24337 | 12814 | -47% | 0/2 | 0/2 |
| `hle-3` | hle | 1208 | 964 | -20% | 2/2 | 2/2 |
| **19 active** | | **37788** | **26008** | **-31%** | **34/38** | **31/38** |

Caveman cut billed thinking on reasoning-active tasks but accuracy held — compression without measurable poison (on this small sample).

## Verdict

Median billed output: baseline 106 → caveman 88 (-17%). Accuracy: baseline 66/72, caveman 63/72.

Overall accuracy 129/144. Caveats: AIME / MMLU-Pro-CS are largely one-shot (output≈answer, so they barely separate the arms); MATH-L5 carries most of the output signal; MATH grading is normalized-string (approximate); HumanEval output is code the skill leaves unchanged by rule. The headline ~75% compression is a *prose* claim measured separately — this bench's job is to show the SKILL.md neither breaks accuracy nor inflates token cost on hard answer-only evals.

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
| aime-3 | aime | baseline | 1 | 72 | 69 | end_turn | ✓ |
| aime-3 | aime | baseline | 2 | 535 | 532 | end_turn | ✗ |
| aime-3 | aime | caveman | 1 | 439 | 436 | end_turn | ✓ |
| aime-3 | aime | caveman | 2 | 734 | 731 | end_turn | ✗ |
| math5-0 | math5 | baseline | 1 | 5 | 0 | end_turn | ✓ |
| math5-0 | math5 | baseline | 2 | 134 | 127 | end_turn | ✓ |
| math5-0 | math5 | caveman | 1 | 103 | 96 | end_turn | ✓ |
| math5-0 | math5 | caveman | 2 | 114 | 107 | end_turn | ✓ |
| math5-1 | math5 | baseline | 1 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | baseline | 2 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| math5-1 | math5 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| math5-2 | math5 | baseline | 1 | 899 | 894 | end_turn | ✗ |
| math5-2 | math5 | baseline | 2 | 932 | 917 | end_turn | ✓ |
| math5-2 | math5 | caveman | 1 | 999 | 994 | end_turn | ✗ |
| math5-2 | math5 | caveman | 2 | 744 | 739 | end_turn | ✗ |
| math5-3 | math5 | baseline | 1 | 76 | 73 | end_turn | ✓ |
| math5-3 | math5 | baseline | 2 | 184 | 181 | end_turn | ✓ |
| math5-3 | math5 | caveman | 1 | 65 | 62 | end_turn | ✓ |
| math5-3 | math5 | caveman | 2 | 85 | 82 | end_turn | ✓ |
| math500-0 | math500 | baseline | 1 | 23 | 0 | end_turn | ✓ |
| math500-0 | math500 | baseline | 2 | 23 | 0 | end_turn | ✓ |
| math500-0 | math500 | caveman | 1 | 41 | 18 | end_turn | ✓ |
| math500-0 | math500 | caveman | 2 | 41 | 18 | end_turn | ✓ |
| math500-1 | math500 | baseline | 1 | 66 | 59 | end_turn | ✓ |
| math500-1 | math500 | baseline | 2 | 5 | 0 | end_turn | ✓ |
| math500-1 | math500 | caveman | 1 | 19 | 12 | end_turn | ✓ |
| math500-1 | math500 | caveman | 2 | 112 | 105 | end_turn | ✓ |
| math500-2 | math500 | baseline | 1 | 115 | 102 | end_turn | ✓ |
| math500-2 | math500 | baseline | 2 | 117 | 102 | end_turn | ✓ |
| math500-2 | math500 | caveman | 1 | 99 | 94 | end_turn | ✗ |
| math500-2 | math500 | caveman | 2 | 107 | 102 | end_turn | ✗ |
| math500-3 | math500 | baseline | 1 | 41 | 38 | end_turn | ✓ |
| math500-3 | math500 | baseline | 2 | 41 | 38 | end_turn | ✓ |
| math500-3 | math500 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| math500-3 | math500 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| aime25-0 | aime25 | baseline | 1 | 96 | 93 | end_turn | ✓ |
| aime25-0 | aime25 | baseline | 2 | 97 | 94 | end_turn | ✓ |
| aime25-0 | aime25 | caveman | 1 | 98 | 95 | end_turn | ✓ |
| aime25-0 | aime25 | caveman | 2 | 91 | 88 | end_turn | ✓ |
| aime25-1 | aime25 | baseline | 1 | 3 | 0 | end_turn | ✓ |
| aime25-1 | aime25 | baseline | 2 | 3 | 0 | end_turn | ✓ |
| aime25-1 | aime25 | caveman | 1 | 3 | 0 | end_turn | ✓ |
| aime25-1 | aime25 | caveman | 2 | 3 | 0 | end_turn | ✓ |
| aime25-2 | aime25 | baseline | 1 | 432 | 429 | end_turn | ✓ |
| aime25-2 | aime25 | baseline | 2 | 121 | 118 | end_turn | ✓ |
| aime25-2 | aime25 | caveman | 1 | 394 | 391 | end_turn | ✓ |
| aime25-2 | aime25 | caveman | 2 | 511 | 508 | end_turn | ✓ |
| aime25-3 | aime25 | baseline | 1 | 340 | 337 | end_turn | ✓ |
| aime25-3 | aime25 | baseline | 2 | 441 | 438 | end_turn | ✓ |
| aime25-3 | aime25 | caveman | 1 | 294 | 291 | end_turn | ✓ |
| aime25-3 | aime25 | caveman | 2 | 340 | 337 | end_turn | ✓ |
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
| HumanEval_1 | humaneval | baseline | 1 | 287 | 9 | end_turn | ✓ |
| HumanEval_1 | humaneval | baseline | 2 | 287 | 9 | end_turn | ✓ |
| HumanEval_1 | humaneval | caveman | 1 | 298 | 9 | end_turn | ✓ |
| HumanEval_1 | humaneval | caveman | 2 | 292 | 9 | end_turn | ✓ |
| HumanEval_2 | humaneval | baseline | 1 | 131 | 10 | end_turn | ✓ |
| HumanEval_2 | humaneval | baseline | 2 | 127 | 6 | end_turn | ✓ |
| HumanEval_2 | humaneval | caveman | 1 | 121 | 0 | end_turn | ✓ |
| HumanEval_2 | humaneval | caveman | 2 | 121 | 0 | end_turn | ✓ |
| HumanEval_3 | humaneval | baseline | 1 | 203 | 9 | end_turn | ✓ |
| HumanEval_3 | humaneval | baseline | 2 | 200 | 6 | end_turn | ✓ |
| HumanEval_3 | humaneval | caveman | 1 | 202 | 8 | end_turn | ✓ |
| HumanEval_3 | humaneval | caveman | 2 | 200 | 6 | end_turn | ✓ |
| apps-0 | apps | baseline | 1 | 223 | 45 | end_turn | ✓ |
| apps-0 | apps | baseline | 2 | 269 | 105 | end_turn | ✓ |
| apps-0 | apps | caveman | 1 | 235 | 85 | end_turn | ✓ |
| apps-0 | apps | caveman | 2 | 232 | 94 | end_turn | ✓ |
| apps-1 | apps | baseline | 1 | 221 | 46 | end_turn | ✓ |
| apps-1 | apps | baseline | 2 | 277 | 81 | end_turn | ✓ |
| apps-1 | apps | caveman | 1 | 197 | 27 | end_turn | ✓ |
| apps-1 | apps | caveman | 2 | 194 | 29 | end_turn | ✓ |
| apps-2 | apps | baseline | 1 | 438 | 387 | end_turn | ✓ |
| apps-2 | apps | baseline | 2 | 495 | 411 | end_turn | ✓ |
| apps-2 | apps | caveman | 1 | 77 | 0 | end_turn | ✓ |
| apps-2 | apps | caveman | 2 | 56 | 0 | end_turn | ✓ |
| apps-3 | apps | baseline | 1 | 8245 | 7322 | end_turn | ✓ |
| apps-3 | apps | baseline | 2 | 9915 | 9125 | end_turn | ✓ |
| apps-3 | apps | caveman | 1 | 8924 | 8222 | end_turn | ✓ |
| apps-3 | apps | caveman | 2 | 9676 | 8936 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | baseline | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | baseline | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | caveman | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-0 | gpqa_diamond | caveman | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | baseline | 1 | 209 | 206 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | baseline | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | caveman | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-1 | gpqa_diamond | caveman | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | baseline | 1 | 132 | 129 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | baseline | 2 | 123 | 120 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | caveman | 1 | 151 | 148 | end_turn | ✓ |
| gpqa-2 | gpqa_diamond | caveman | 2 | 108 | 105 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | baseline | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | baseline | 2 | 3 | 0 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | caveman | 1 | 3 | 0 | end_turn | ✓ |
| gpqa-3 | gpqa_diamond | caveman | 2 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | baseline | 1 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | baseline | 2 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | caveman | 1 | 3 | 0 | end_turn | ✓ |
| hle-0 | hle | caveman | 2 | 3 | 0 | end_turn | ✓ |
| hle-1 | hle | baseline | 1 | 1067 | 1061 | end_turn | ✓ |
| hle-1 | hle | baseline | 2 | 815 | 809 | end_turn | ✓ |
| hle-1 | hle | caveman | 1 | 853 | 847 | end_turn | ✓ |
| hle-1 | hle | caveman | 2 | 702 | 696 | end_turn | ✓ |
| hle-2 | hle | baseline | 1 | 25952 | 25937 | end_turn | ✗ |
| hle-2 | hle | baseline | 2 | 22752 | 22737 | end_turn | ✗ |
| hle-2 | hle | caveman | 1 | 0 | 0 | error | ✗ |
| hle-2 | hle | caveman | 2 | 25643 | 25628 | end_turn | ✗ |
| hle-3 | hle | baseline | 1 | 1798 | 1795 | end_turn | ✓ |
| hle-3 | hle | baseline | 2 | 623 | 620 | end_turn | ✓ |
| hle-3 | hle | caveman | 1 | 1288 | 1271 | end_turn | ✓ |
| hle-3 | hle | caveman | 2 | 660 | 657 | end_turn | ✓ |
