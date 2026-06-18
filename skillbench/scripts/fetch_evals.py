#!/usr/bin/env python3
"""Fetch HARD task items from established eval libraries into data/tasks.json.

Chosen to force substantial chain-of-thought (the easy sets are one-shot by
Opus 4.8, leaving nothing to separate the arms). Source: HuggingFace
datasets-server. Gated repos (GPQA, HLE) need a token — read at runtime via
`rageveil show huggingface.co/doma@doma.dev/Big_Token_Doma`, never written down.

  math : AI-MO/aimo-validation-aime          (AIME competition, integer answers)
  math : nlile/hendrycks-MATH-benchmark L5   (hardest MATH tier, exact answer)
  math : HuggingFaceH4/MATH-500              (broad MATH-500, exact answer)
  math : math-ai/aime25                      (AIME 2025, fresh, integer answers)
  cs   : TIGER-Lab/MMLU-Pro  computer science (10-option reasoning MC, letter)
  cs   : openai/openai_humaneval             (code generation, executed)
  sci  : Idavidrein/gpqa  gpqa_diamond       (graduate science MC, letter) [gated]
  any  : cais/hle                            (Humanity's Last Exam, text-only) [gated]

Deterministic first-N per suite. N defaults to 4; override with the N env var.
RAW DATASET ROWS ARE NEVER COMMITTED — data/tasks.json is gitignored.
"""
import json, os, subprocess, urllib.request, urllib.parse

N = int(os.environ.get("N", "4"))
ROOT = os.environ.get("SKILLBENCH_ROOT",
                      os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DSS = "https://datasets-server.huggingface.co"

def hf_token():
    try:
        out = subprocess.run(["rageveil", "show", "huggingface.co/doma@doma.dev/Big_Token_Doma"],
                             capture_output=True, text=True, timeout=30).stdout
        return out.strip()
    except Exception:
        return ""

TOKEN = hf_token()

def _get(path, params):
    url = f"{DSS}/{path}?" + urllib.parse.urlencode(params)
    req = urllib.request.Request(url)
    if TOKEN:
        req.add_header("Authorization", f"Bearer {TOKEN}")
    with urllib.request.urlopen(req, timeout=120) as r:
        return [x["row"] for x in json.load(r)["rows"]]

def rows(dataset, config, split, offset, length):
    return _get("rows", {"dataset": dataset, "config": config, "split": split, "offset": offset, "length": length})

def filt(dataset, config, split, where, length):
    return _get("filter", {"dataset": dataset, "config": config, "split": split, "where": where, "offset": 0, "length": length})

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

LETTERS = "ABCDEFGHIJ"
tasks = []

def add(fn, label):
    """Run a fetch step; never let one bad repo kill the whole pull."""
    try:
        before = len(tasks)
        fn()
        print(f"  + {label}: {len(tasks)-before}")
    except Exception as e:
        print(f"  ! {label}: SKIPPED ({type(e).__name__}: {str(e)[:120]})")

def aime():
    for i, r in enumerate(rows("AI-MO/aimo-validation-aime", "default", "train", 0, N)):
        tasks.append({"suite": "aime", "domain": "math", "id": f"aime-{i}", "kind": "numeric",
                      "instruction": "Output only the final integer answer (0-999), nothing else.",
                      "prompt": r["problem"], "gold": str(r["answer"]).strip()})

def math5():
    for i, r in enumerate(paged("nlile/hendrycks-MATH-benchmark", "default", "test", N, lambda r: int(r["level"]) == 5)):
        tasks.append({"suite": "math5", "domain": "math", "id": f"math5-{i}", "kind": "exact",
                      "instruction": "Output only the final answer, with no surrounding text or units.",
                      "prompt": r["problem"], "gold": str(r["answer"]).strip()})

def math500():
    for i, r in enumerate(rows("HuggingFaceH4/MATH-500", "default", "test", 0, N)):
        tasks.append({"suite": "math500", "domain": "math", "id": f"math500-{i}", "kind": "exact",
                      "instruction": "Output only the final answer, with no surrounding text or units.",
                      "prompt": r["problem"], "gold": str(r["answer"]).strip()})

def aime25():
    for i, r in enumerate(rows("math-ai/aime25", "default", "test", 0, N)):
        tasks.append({"suite": "aime25", "domain": "math", "id": f"aime25-{i}", "kind": "numeric",
                      "instruction": "Output only the final integer answer (0-999), nothing else.",
                      "prompt": r["problem"], "gold": str(r["answer"]).strip()})

def mmlu_pro_cs():
    for i, r in enumerate(filt("TIGER-Lab/MMLU-Pro", "default", "test", "\"category\"='computer science'", N)):
        opts = r["options"]
        body = r["question"] + "\n" + "\n".join(f"{l}) {c}" for l, c in zip(LETTERS, opts))
        tasks.append({"suite": "mmlu_pro_cs", "domain": "cs", "id": f"mmlupro-cs-{i}", "kind": "letter",
                      "instruction": f"Output only the single letter (A-{LETTERS[len(opts)-1]}) of the correct option.",
                      "prompt": body, "gold": str(r["answer"]).strip()})

def humaneval():
    for i, r in enumerate(rows("openai/openai_humaneval", "openai_humaneval", "test", 0, N)):
        tasks.append({"suite": "humaneval", "domain": "cs", "id": r["task_id"].replace("/", "_"), "kind": "code",
                      "instruction": "Complete the function. Output only the full Python function definition (including the signature), no markdown fences, no commentary.",
                      "prompt": r["prompt"], "gold": "", "test": r["test"], "entry_point": r["entry_point"]})

def gpqa():
    # gated. Deterministic option order (sorted by text) so position isn't a tell.
    for i, r in enumerate(rows("Idavidrein/gpqa", "gpqa_diamond", "train", 0, N)):
        correct = r["Correct Answer"].strip()
        opts = sorted([correct, r["Incorrect Answer 1"].strip(),
                       r["Incorrect Answer 2"].strip(), r["Incorrect Answer 3"].strip()])
        body = r["Question"].strip() + "\n" + "\n".join(f"{l}) {c}" for l, c in zip(LETTERS, opts))
        gold = LETTERS[opts.index(correct)]
        tasks.append({"suite": "gpqa_diamond", "domain": "sci", "id": f"gpqa-{i}", "kind": "letter",
                      "instruction": "Output only the single letter (A-D) of the correct option.",
                      "prompt": body, "gold": gold})

def hle():
    # gated. text-only items (drop image questions); honor answer_type.
    got = 0
    for r in rows("cais/hle", "default", "test", 0, max(N * 6, 60)):
        if got >= N: break
        if r.get("image"):  # skip multimodal — this harness is text-only
            continue
        atype = (r.get("answer_type") or "").lower()
        kind = "letter" if "multiple" in atype else "exact"
        instr = ("Output only the single letter of the correct option."
                 if kind == "letter" else
                 "Output only the final answer, with no surrounding text, units, or explanation.")
        tasks.append({"suite": "hle", "domain": "any", "id": f"hle-{got}", "kind": kind,
                      "instruction": instr, "prompt": r["question"].strip(), "gold": str(r["answer"]).strip()})
        got += 1

add(aime, "aime")
add(math5, "math5")
add(math500, "math500")
add(aime25, "aime25")
add(mmlu_pro_cs, "mmlu_pro_cs")
add(humaneval, "humaneval")
add(gpqa, "gpqa_diamond [gated]")
add(hle, "hle [gated]")

out = os.path.join(ROOT, "data", "tasks.json")
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump(tasks, open(out, "w"), indent=2)
print(f"wrote {len(tasks)} tasks -> {out}  (token={'yes' if TOKEN else 'NO'})")
for s in sorted({t["suite"] for t in tasks}):
    print(f"  {s}: {sum(1 for t in tasks if t['suite']==s)}")
