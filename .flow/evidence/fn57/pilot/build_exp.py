#!/usr/bin/env python3
"""Build experiment request files for Jev over the fetched growth sources."""
import json
import re
import pathlib

REFS = pathlib.Path("refs")


def text(name: str) -> str:
    return REFS.joinpath(name + ".txt").read_text()


def context(src: str, sentence: str, radius: int = 320) -> str:
    i = src.find(sentence[:60])
    if i < 0:
        return sentence
    return src[max(0, i - radius): i + len(sentence) + radius].strip()


SOURCES = {
    "owic": ("Oregon Wood Innovation Center, Oregon State University: Oregon white oak (Quercus garryana) species page",
             "https://owic.oregonstate.edu/oregon-white-oak-quercus-garryana"),
    "iastate": ("Iowa State University Extension, Iowa trees: Norway spruce",
                "https://naturalresources.extension.iastate.edu/forestry/iowa_trees/trees/norway_spruce.html"),
    "ncta": ("National Christmas Tree Association: Norway spruce variety page",
             "https://realchristmastrees.org/education/tree-varieties/norway-spruce/"),
}

# ---------- Experiment 1: screen candidate sentences ----------

SCREEN_Q = {
    "kind": {
        "type": "choice",
        "instructions": "The state holds one sentence from a source about a tree species, with the surrounding text. "
                        "What kind of statement is `candidate.sentence`, read in its context?",
        "criteria": {
            "measured_size_at_age": {
                "what": "A height or diameter paired with a specific age or age span, reported as what trees of that age reach",
                "examples": ["reaches 6 m at 20 years", "8 to 11 years are required to grow a 6-7 foot tree"],
            },
            "site_quality_criterion": {
                "what": "A growth figure used as a test of how good a site is, listed among indicators of site potential; it describes a threshold, not what trees typically do",
                "not_for": "an observed growth rate reported as typical",
            },
            "mature_size_range": {
                "what": "The size of mature or full-grown trees with no age given, or a maximum size",
            },
            "typical_growth_rate": {
                "what": "A rate of growth per year or per period reported as typical or usual for the species, not tied to a site test",
                "not_for": "stump sprouts, cultivars or nursery stock",
            },
            "sprout_cultivar_or_nursery": {
                "what": "Growth or size of stump sprouts, coppice, a named cultivar, or nursery seedling stock, which does not describe a seed-grown wild tree",
            },
            "not_about_tree_size": {
                "what": "The number is about something else: elevation, range, cones, needles, longevity, wood durability, seed bearing",
            },
        },
    },
    "condition": {
        "type": "choice",
        "instructions": "Under what growing condition does `candidate.sentence`, read in its context, describe the tree?",
        "criteria": {
            "open_grown": "A tree in the open, a landscape, a yard or a field with no neighbouring canopy competition",
            "stand_grown": "A tree inside a forest stand or closed canopy, or a forestry site-index or stand context",
            "plantation_or_nursery": "A planted crop such as a Christmas tree plantation, a nursery bed or a seedling lot",
            "unstated": "The context does not say under what condition the tree grew",
        },
    },
    "anchor_usable": {
        "type": "noul",
        "instructions": "Could code copy a height-at-age pair for a seed-grown tree of this species straight from `candidate.sentence`, "
                        "using only numbers the sentence states and no assumption about age or condition?",
        "criteria": {
            "true": "The sentence itself states both a height (or height range) and the age at which it is reached",
            "false": "Age or height is missing, the figure is a rate, a threshold, a maximum, a mature range, or it is about sprouts, cultivars or nursery stock",
        },
    },
}

SCREEN_CASES = [
    ("owic", "Mature Oregon white oaks are 50 to 90 ft tall (120 ft maximum) and 24 to 40 in.", "mature_size_range / unstated / false"),
    ("owic", "Oregon white oaks may live 500 years.", "not_about_tree_size"),
    ("owic", "Sustained height growth of 1 to 2 ft per year for trees 10 to 30 years old", "site_quality_criterion / stand_grown / false  <-- the CLAUDE.md case"),
    ("owic", "It grows from sea level to 3800 ft in the north and at elevations of up to 7500 ft at the southern end of its range.", "not_about_tree_size"),
    ("owic", "Height growth is usually less than 1 ft per year and diameter growth is often 15 to 20 rings/in.", "typical_growth_rate / false"),
    ("owic", "Stump sprouts may grow as much as 3 ft per year during the first 3 years.", "sprout_cultivar_or_nursery / false"),
    ("owic", "untreated fence posts lasted an average of 18 years before failure.", "not_about_tree_size"),
    ("iastate", "(may grow to 75 feet in 50 years).", "measured_size_at_age / open_grown or unstated / true"),
    ("iastate", "Norway spruce grow 75 to 100 feet tall.", "mature_size_range / false"),
    ("ncta", "Growth during the first 10 years after field planting is relatively slow and 8 to 11 years are required to grow a 6-7 foot tree.", "measured_size_at_age / plantation_or_nursery / true"),
    ("ncta", "In Europe, Norway spruce grows from 130 to 215 feet in height, but in the United States is seldom more than 130 feet tall.", "mature_size_range / false"),
    ("ncta", "It is a cool climate species and is found at elevations of 3,300 feet to 7,500 feet.", "not_about_tree_size"),
]


def screen_cases():
    out = []
    for src, sentence, expect in SCREEN_CASES:
        title, url = SOURCES[src]
        state = {
            "source": {"title": title, "url": url},
            "candidate": {"sentence": sentence, "context": context(text(src), sentence)},
        }
        out.append({"label": f"{src}: {sentence[:70]}", "expect": expect,
                    "request": {"state": state, "questions": SCREEN_Q}})
    return out


# ---------- Experiment 2: citation check of spec claims ----------

CITE_Q = {
    "relation": {
        "type": "choice",
        "instructions": "How does `section` relate to `claim`? The claim was written by a research agent summarizing the source; "
                        "unit conversions (feet to metres) are acceptable paraphrase.",
        "criteria": {
            "supports": "The section states the claim or directly implies that it is true",
            "contradicts": "The section states the opposite of the claim or implies it is false",
            "says_nothing": "The section does not address what the claim asserts, either way",
        },
    },
}

CITE_CASES = [
    ("iastate", "Norway spruce in the landscape: moderate to fast when young, about 23 m in 50 years.",
     "(may grow to 75 feet in 50 years).", "supports"),
    ("iastate", "Norway spruce is stiff when young, becoming more graceful with age.",
     "Stiff when young, becoming more graceful with age.", "supports"),
    ("ncta", "Norway spruce plantation: growth in the first ten years after planting is slow; 8 to 11 years to reach a 1.8 to 2.1 m tree.",
     "Growth during the first 10 years after field planting is relatively slow and 8 to 11 years are required to grow a 6-7 foot tree.", "supports"),
    ("ncta", "Norway spruce reaches a 1.8 to 2.1 m tree within five years of field planting.",
     "Growth during the first 10 years after field planting is relatively slow and 8 to 11 years are required to grow a 6-7 foot tree.", "contradicts"),
    ("owic", "Stump sprouts reach up to 0.9 m a year for the first three years.",
     "Stump sprouts may grow as much as 3 ft per year during the first 3 years.", "supports"),
    ("owic", "Wild seedlings put down a deep taproot and the shoot stays small and shrubby for many years.",
     "The shoot of natural seedlings often remains small and shrubby for many years, perhaps to accommodate development of deep roots.", "supports"),
    ("owic", "Open-grown Oregon white oak reaches 2.4 m in height at ten years of age.",
     "Sustained height growth of 1 to 2 ft per year for trees 10 to 30 years old", "says_nothing  <-- the O1 misuse"),
    ("owic", "Oregon white oak typically grows 1 to 2 ft per year between ages 10 and 30.",
     "Sustained height growth of 1 to 2 ft per year for trees 10 to 30 years old", "says_nothing or contradicts (it is a site criterion, not a typical rate)"),
    ("owic", "Oregon white oak height growth is usually under 0.3 m a year.",
     "Height growth is usually less than 1 ft per year and diameter growth is often 15 to 20 rings/in.", "supports"),
]


def cite_cases():
    out = []
    for src, claim, sentence, expect in CITE_CASES:
        title, url = SOURCES[src]
        state = {"source": {"title": title, "url": url}, "claim": claim,
                 "section": context(text(src), sentence, 260)}
        out.append({"label": f"claim: {claim[:80]}", "expect": expect,
                    "request": {"state": state, "questions": CITE_Q}})
    return out


# ---------- Experiment 3: select a pre-parsed number ----------

NUM_RE = re.compile(r"\d[\d,]*(?:\.\d+)?(?:\s*(?:to|-|–)\s*\d[\d,]*(?:\.\d+)?)?\s*(?:ft|feet|foot|in\.|inches|m\b|cm|years?|rings/in)", re.I)


def select_cases():
    out = []
    for src, questions in [
        ("owic", {
            "mature_height": "Which candidate span states the height of a mature Oregon white oak under ordinary conditions (not the maximum)?",
            "height_at_10y": "Which candidate span states the height an Oregon white oak has reached at ten years of age?",
            "lifespan": "Which candidate span states how long an Oregon white oak may live?",
        }),
        ("ncta", {
            "young_height_with_age": "Which candidate span is the height a Norway spruce reaches at a stated age after field planting?",
            "young_age": "Which candidate span is the age at which a field-planted Norway spruce reaches 6 to 7 feet?",
        }),
    ]:
        t = text(src)
        cands = []
        for m in NUM_RE.finditer(t):
            s = re.sub(r"\s+", " ", m.group(0)).strip()
            if s not in cands:
                cands.append(s)
        cands = cands[:200]
        qs = {}
        for qid, instr in questions.items():
            crit = {c: None for c in cands}
            crit["none"] = "No candidate span states this"
            qs[qid] = {"type": "choice", "instructions": instr, "criteria": crit}
        title, url = SOURCES[src]
        out.append({"label": f"select over {src} ({len(cands)} candidates)",
                    "expect": "owic: 50 to 90 ft / none / 500 years; ncta: 6-7 foot / 8 to 11 years",
                    "request": {"state": {"source": {"title": title, "url": url}, "document": t}, "questions": qs}})
    return out


if __name__ == "__main__":
    json.dump(screen_cases(), open("exp1_screen.json", "w"), indent=1)
    json.dump(cite_cases(), open("exp2_cite.json", "w"), indent=1)
    json.dump(select_cases(), open("exp3_select.json", "w"), indent=1)
    print("built", len(screen_cases()), len(cite_cases()), len(select_cases()))
