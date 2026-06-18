#!/usr/bin/env python3
"""Fetch HARD task items from established eval libraries into data/tasks.json.

Chosen to force substantial chain-of-thought (the easy sets are one-shot by
Opus 4.8, leaving nothing to separate the arms):

  math : AI-MO/aimo-validation-aime          (AIME competition, integer answers)
  math : nlile/hendrycks-MATH-benchmark L5   (hardest MATH tier, exact answer)
  cs   : TIGER-Lab/MMLU-Pro  computer science (10-option reasoning MC, letter)
  cs   : openai/openai_humaneval             (code generation, executed)

Source: HuggingFace datasets-server (public, no auth). Deterministic first-N.
N defaults to 4; override with the N env var.
"""
import json, os, urllib.request, urllib.parse

N = int(os.environ.get("N", "4"))
ROOT = os.environ.get("SKILLBENCH_ROOT",
                      os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

def rows(dataset, config, split, offset, length):
    url = "https://datasets-server.huggingface.co/rows?" + urllib.parse.urlencode(
        {"dataset": dataset, "config": config, "split": split, "offset": offset, "length": length})
    with urllib.request.urlopen(url, timeout=90) as r:
        return [x["row"] for x in json.load(r)["rows"]]

def filt(dataset, config, split, where, length):
    url = "https://datasets-server.huggingface.co/filter?" + urllib.parse.urlencode(
        {"dataset": dataset, "config": config, "split": split, "where": where, "offset": 0, "length": length})
    with urllib.request.urlopen(url, timeout=90) as r:
        return [x["row"] for x in json.load(r)["rows"]]

def paged(dataset, config, split, n, pred, cap=6000):
    got, off = [], 0
    while len(got) < n and off < cap:
        rs = rows(dataset, config, split, off, 100)
        if not rs: break
        for r in rs:
            if pred(r): got.append(r)
            if len(got) >= n: break
        off += 100
    return got

tasks = []

for i, r in enumerate(rows("AI-MO/aimo-validation-aime", "default", "train", 0, N)):
    tasks.append({"suite": "aime", "domain": "math", "id": f"aime-{i}", "kind": "numeric",
                  "instruction": "Output only the final integer answer (0-999), nothing else.",
                  "prompt": r["problem"], "gold": str(r["answer"]).strip()})

for i, r in enumerate(paged("nlile/hendrycks-MATH-benchmark", "default", "test", N, lambda r: int(r["level"]) == 5)):
    tasks.append({"suite": "math5", "domain": "math", "id": f"math5-{i}", "kind": "exact",
                  "instruction": "Output only the final answer, with no surrounding text or units.",
                  "prompt": r["problem"], "gold": str(r["answer"]).strip()})

LETTERS = "ABCDEFGHIJ"
for i, r in enumerate(filt("TIGER-Lab/MMLU-Pro", "default", "test", "\"category\"='computer science'", N)):
    opts = r["options"]
    body = r["question"] + "\n" + "\n".join(f"{l}) {c}" for l, c in zip(LETTERS, opts))
    tasks.append({"suite": "mmlu_pro_cs", "domain": "cs", "id": f"mmlupro-cs-{i}", "kind": "letter",
                  "instruction": f"Output only the single letter (A-{LETTERS[len(opts)-1]}) of the correct option.",
                  "prompt": body, "gold": str(r["answer"]).strip()})

for i, r in enumerate(rows("openai/openai_humaneval", "openai_humaneval", "test", 0, N)):
    tasks.append({"suite": "humaneval", "domain": "cs", "id": r["task_id"].replace("/", "_"), "kind": "code",
                  "instruction": "Complete the function. Output only the full Python function definition (including the signature), no markdown fences, no commentary.",
                  "prompt": r["prompt"], "gold": "", "test": r["test"], "entry_point": r["entry_point"]})

out = os.path.join(ROOT, "data", "tasks.json")
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump(tasks, open(out, "w"), indent=2)
print(f"wrote {len(tasks)} tasks -> {out}")
for s in ("aime", "math5", "mmlu_pro_cs", "humaneval"):
    print(f"  {s}: {sum(1 for t in tasks if t['suite']==s)}")
