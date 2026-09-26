//! One contact sheet a round: the references, the current tree and the
//! bundle's strength variants at one view, in an order code shuffles and
//! records. The reviewer ranks them per priority and grades each adjacent
//! step; code reads off what each variant did against the tree it knows is
//! current, which the sheet never says.
//!
//! A new uncalibrated question, under the same terms as the side-by-side
//! review: bootstrap authority only, labelled uncalibrated wherever recorded,
//! and one visual pass per sheet.
mod adapter;
mod outcome;
mod request;
mod review;
pub use adapter::{dispatch, envelope, prompt_request};
pub use outcome::Outcome;
pub use request::{look, skeleton, Look, NotShown, INERT};
pub(in crate::tuning) use review::review;

use super::{evaluation::Image, look::Priority};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;

pub const VERSION: &str = "tuning-sheet-v2";
pub const UNCALIBRATED: &str =
    "uncalibrated contact-sheet review: a ranking and a graded comparison, never a score or a readiness claim";
pub const PENDING: &str = "sheet review";
/// What the persisted judgment input is called, before the sheet is asked.
pub const INPUT_LABEL: &str = "uncalibrated sheet review";
/// What the round records when the sheet it paid for failed or would not
/// bind. The round is over: nothing was judged, so nothing is adopted, and
/// the bundles it drew count as tried.
pub const FAILED_NOTE: &str = "sheet review failed; attempt charged";
pub const PROMPT: &str = "You are shown reference photographs of a tree species, then several renders of the same generated tree at one view and seed, numbered 1 upward. They differ by how far one set of parameters was moved; nothing here says which render is the starting point or in what order they were made.\n\nFor each listed priority: name the render closest to the references, then rank every render from best to worst for that priority, then grade each adjacent pair of your own ranking - clear when the better one is plainly better on that priority, slight when the difference is real but small, none when you cannot tell them apart.\n\nThen rank every render again, best to worst, for one question: which is the most believable tree of this species overall against the references, judging the whole tree and not the listed priorities.\n\nThen, for every render, say what looks wrong in it against the references - at most two things each, and nothing for a render where you see nothing wrong.\n\nThen list anything a render breaks that the others do not, naming the render, and say in one or two sentences what improved across the set and what is still missing in all of them against the references.\n\nGive no numbers, no scores and no overall winner beyond that ranking.";

/// What the reviewer is sent. The renders are numbered, and nothing says
/// which of them the loop is standing on.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub target_species: String,
    pub view: String,
    pub seed: u32,
    pub references: Vec<Image>,
    pub renders: Vec<Image>,
    pub priorities: Vec<Priority>,
    pub owner_notes: String,
}

impl Request {
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn prompt_hash() -> String {
        sha256_hex(PROMPT.as_bytes())
    }
    /// The label of the render at this position, as the reviewer sees it.
    pub fn label(index: usize) -> String {
        (index + 1).to_string()
    }
    pub fn verify(&self) -> Result<(), String> {
        if self.schema != VERSION
            || self.priorities.is_empty()
            || self.priorities.len() > super::look::MAX_PRIORITIES
            || self.references.is_empty()
            || self.renders.len() < 2
            || self.renders.len() > 5
            || self.renders.iter().any(|r| r.view != self.view)
        {
            return Err("invalid contact-sheet request".into());
        }
        let mut seen = vec![];
        for render in &self.renders {
            if seen.contains(&render.sha256) {
                return Err("the sheet shows the same picture twice".into());
            }
            seen.push(render.sha256.clone());
        }
        for image in self.references.iter().chain(self.renders.iter()) {
            image.verify()?;
        }
        Ok(())
    }
}

/// The sheet as code knows it: what was sent, which trial each label is, and
/// which of them is the tree the loop is standing on. The last is recorded
/// and never sent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub request: Request,
    /// The trial key at each label, in label order.
    pub order: Vec<String>,
    /// The label the current tree took.
    pub current: String,
}

/// The order the renders are shown in: decided by all the keys together, so
/// it is stable for the same set and readable off none of them.
pub fn order(keys: &[String]) -> Vec<String> {
    let seed = sha256_hex(&serde_json::to_vec(keys).unwrap());
    let mut rows = keys
        .iter()
        .cloned()
        .map(|key| (sha256_hex(format!("{seed}|{key}").as_bytes()), key))
        .collect::<Vec<_>>();
    rows.sort();
    rows.into_iter().map(|(_, key)| key).collect()
}

/// The sheet for one round: the current tree and its variants, shuffled.
pub fn plan(
    species: &str,
    view: &str,
    seed: u32,
    references: &[Image],
    current: (&str, &Image),
    variants: &[(String, Image)],
    priorities: &[Priority],
    owner_notes: &str,
) -> Result<Plan, String> {
    let mut stills: Vec<(String, Image)> = vec![(current.0.to_owned(), current.1.clone())];
    stills.extend(variants.iter().cloned());
    let shuffled = order(
        &stills
            .iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>(),
    );
    let request = Request {
        schema: VERSION.into(),
        target_species: species.into(),
        view: view.into(),
        seed,
        references: references
            .iter()
            .filter(|i| i.view == view)
            .cloned()
            .collect(),
        renders: shuffled
            .iter()
            .map(|key| {
                stills
                    .iter()
                    .find(|(k, _)| k == key)
                    .map(|(_, image)| image.clone())
                    .ok_or("lost a render while shuffling")
            })
            .collect::<Result<Vec<_>, _>>()?,
        priorities: priorities.to_vec(),
        owner_notes: owner_notes.into(),
    };
    request.verify()?;
    let current = shuffled
        .iter()
        .position(|key| key == current.0)
        .map(Request::label)
        .ok_or("lost the current tree while shuffling")?;
    Ok(Plan {
        request,
        order: shuffled,
        current,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    Clear,
    Slight,
    None,
}

/// One adjacent pair of the reviewer's own ranking, and how far apart they are.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub from: String,
    pub to: String,
    pub grade: Grade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriorityAnswer {
    pub priority_id: String,
    pub closest: String,
    pub ranking: Vec<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Break {
    pub render: String,
    pub text: String,
}

/// What looks wrong in one render against the references. Not a comparison and
/// not a grade: the words the owner's eye would have used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Wrong {
    pub render: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    pub priorities: Vec<PriorityAnswer>,
    /// Every render, best to worst, on believability as this species overall.
    /// The priorities are two of the tree's qualities; this is the tree.
    pub overall: Vec<String>,
    pub wrong: Vec<Wrong>,
    pub breaks: Vec<Break>,
    pub improved: String,
    pub missing: String,
}

/// What one render did against the current tree on one priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Movement {
    Clear,
    Slight,
    None,
    Worse,
}

/// Read off the ranking: a render above the current tree is clear when any
/// adjacent step between them is clear, slight when none is clear but one is
/// slight, and none when every step between them is none. A render below the
/// current tree is worse unless every step down to it is none.
pub fn grade(ranking: &[String], steps: &[Step], current: &str, render: &str) -> Movement {
    let (Some(here), Some(there)) = (
        ranking.iter().position(|r| r == current),
        ranking.iter().position(|r| r == render),
    ) else {
        return Movement::None;
    };
    let (lo, hi) = (here.min(there), here.max(there));
    let between = &steps[lo.min(steps.len())..hi.min(steps.len())];
    let worst = |want: Grade| between.iter().any(|s| s.grade == want);
    if there == here {
        Movement::None
    } else if there < here {
        if worst(Grade::Clear) {
            Movement::Clear
        } else if worst(Grade::Slight) {
            Movement::Slight
        } else {
            Movement::None
        }
    } else if worst(Grade::Clear) || worst(Grade::Slight) {
        Movement::Worse
    } else {
        Movement::None
    }
}

/// What one render did, read back against the tree the sheet did not name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Render {
    pub key: String,
    pub label: String,
    pub per_priority: BTreeMap<String, Movement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breaks: Vec<String>,
    /// Its place in the overall believability ranking, best is 1.
    #[serde(default)]
    pub overall: usize,
    /// True when the reviewer put the current tree above it overall. A step
    /// that wins a priority and loses the tree is not a step forward.
    #[serde(default)]
    pub below_current: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wrong: Vec<String>,
}

impl Render {
    pub fn improved(&self) -> usize {
        self.per_priority
            .values()
            .filter(|m| matches!(m, Movement::Clear | Movement::Slight))
            .count()
    }
    pub fn clear(&self) -> bool {
        self.per_priority.values().any(|m| *m == Movement::Clear)
    }
    pub fn worse(&self) -> bool {
        self.per_priority.values().any(|m| *m == Movement::Worse)
    }
    /// Better somewhere, worse nowhere, breaking nothing of its own, and not
    /// a less believable tree than the one the loop is standing on.
    pub fn adoptable(&self) -> bool {
        self.improved() > 0 && !self.worse() && self.breaks.is_empty() && !self.below_current
    }
    /// Better somewhere and worse nowhere, but it breaks something: the case
    /// a split is for.
    pub fn better_with_breaks(&self) -> bool {
        self.improved() > 0 && !self.worse() && !self.breaks.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verdict {
    pub renders: Vec<Render>,
    pub improved: String,
    pub missing: String,
    pub ledger: String,
    pub model: String,
    pub current: String,
    pub order: Vec<String>,
    pub uncalibrated: String,
}

impl Verdict {
    pub fn render(&self, key: &str) -> Option<&Render> {
        self.renders.iter().find(|r| r.key == key)
    }
}

/// Strict: every priority answered once, every ranking a permutation of the
/// labels on the sheet, one step per adjacent pair in that ranking, the
/// closest render and every break naming a label that is on the sheet.
pub fn bind(
    plan: &Plan,
    answer: &Answer,
    ledger: String,
    model: String,
) -> Result<Verdict, String> {
    let labels = (0..plan.request.renders.len())
        .map(Request::label)
        .collect::<Vec<_>>();
    let mut per_render: BTreeMap<String, BTreeMap<String, Movement>> = BTreeMap::new();
    let mut seen = vec![];
    for priority in &answer.priorities {
        if !plan
            .request
            .priorities
            .iter()
            .any(|p| p.id == priority.priority_id)
        {
            return Err(format!(
                "sheet verdict for an unrequested priority {}",
                priority.priority_id
            ));
        }
        if seen.contains(&priority.priority_id) {
            return Err(format!("sheet verdict repeats {}", priority.priority_id));
        }
        seen.push(priority.priority_id.clone());
        let mut sorted = priority.ranking.clone();
        sorted.sort();
        let mut want = labels.clone();
        want.sort();
        if sorted != want {
            return Err("a ranking is not every render on the sheet, once each".into());
        }
        if !labels.contains(&priority.closest) {
            return Err("the closest render is not on the sheet".into());
        }
        if priority.steps.len() + 1 != priority.ranking.len()
            || priority
                .steps
                .iter()
                .zip(priority.ranking.windows(2))
                .any(|(step, pair)| step.from != pair[0] || step.to != pair[1])
        {
            return Err("the steps are not the adjacent pairs of the ranking".into());
        }
        for (index, label) in labels.iter().enumerate() {
            let movement = grade(&priority.ranking, &priority.steps, &plan.current, label);
            per_render
                .entry(plan.order[index].clone())
                .or_default()
                .insert(priority.priority_id.clone(), movement);
        }
    }
    if seen.len() != plan.request.priorities.len() {
        return Err("the sheet answer does not cover every priority".into());
    }
    if answer.improved.trim().is_empty() || answer.missing.trim().is_empty() {
        return Err("the sheet answer lacks what improved or what is missing".into());
    }
    let mut sorted = answer.overall.clone();
    sorted.sort();
    let mut want = labels.clone();
    want.sort();
    if sorted != want {
        return Err("the overall ranking is not every render on the sheet, once each".into());
    }
    let place = |label: &str| answer.overall.iter().position(|r| r == label);
    let here = place(&plan.current).ok_or("the overall ranking lost the current tree")?;
    let mut renders = vec![];
    for (index, key) in plan.order.iter().enumerate() {
        let label = Request::label(index);
        let wrong = answer
            .wrong
            .iter()
            .filter(|w| w.render == label)
            .map(|w| w.text.clone())
            .collect::<Vec<_>>();
        if wrong.len() > 2 {
            return Err("more than two things wrong with one render".into());
        }
        let overall = place(&label).ok_or("the overall ranking lost a render")?;
        renders.push(Render {
            key: key.clone(),
            label: label.clone(),
            per_priority: per_render.remove(key).unwrap_or_default(),
            breaks: answer
                .breaks
                .iter()
                .filter(|b| b.render == label)
                .map(|b| b.text.clone())
                .collect(),
            overall: overall + 1,
            below_current: overall > here,
            wrong,
        });
    }
    for entry in &answer.wrong {
        if !labels.contains(&entry.render) || entry.text.trim().is_empty() {
            return Err(format!(
                "an empty note, or one naming no render on the sheet: {}",
                entry.render
            ));
        }
    }
    for entry in &answer.breaks {
        if !labels.contains(&entry.render) {
            return Err(format!(
                "a break names no render on the sheet: {}",
                entry.render
            ));
        }
    }
    Ok(Verdict {
        renders,
        improved: answer.improved.clone(),
        missing: answer.missing.clone(),
        ledger,
        model,
        current: plan.current.clone(),
        order: plan.order.clone(),
        uncalibrated: UNCALIBRATED.into(),
    })
}

/// Which variant the round keeps: better somewhere, worse nowhere, breaking
/// nothing, and no less believable a tree than the current one. A clear
/// improvement beats a slight one, then more priorities improved, then the
/// smaller strength.
pub fn adopt(verdict: &Verdict, shown: &[(String, f64)]) -> Option<String> {
    let mut eligible = shown
        .iter()
        .filter_map(|(key, strength)| {
            let render = verdict.render(key)?;
            render
                .adoptable()
                .then(|| (key.clone(), render.clear(), render.improved(), *strength))
        })
        .collect::<Vec<_>>();
    eligible.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(a.3.total_cmp(&b.3)));
    eligible.first().map(|(key, _, _, _)| key.clone())
}

/// The best variant that is better but breaks something: what a split starts
/// from when nothing is adoptable.
pub fn to_split(verdict: &Verdict, shown: &[(String, f64)]) -> Option<(String, f64)> {
    let mut candidates = shown
        .iter()
        .filter_map(|(key, strength)| {
            let render = verdict.render(key)?;
            render
                .better_with_breaks()
                .then(|| (key.clone(), render.clear(), render.improved(), *strength))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(a.3.total_cmp(&b.3)));
    candidates
        .first()
        .map(|(key, _, _, strength)| (key.clone(), *strength))
}

/// The reviewer's words about one sheet, for whoever is asked next.
pub fn words(verdict: &Verdict) -> serde_json::Value {
    json!({"improved":verdict.improved,"missing":verdict.missing,
        "renders":verdict.renders.iter().map(|r| json!({"label":r.label,
            "per_priority":r.per_priority,"breaks":r.breaks})).collect::<Vec<_>>(),
        "uncalibrated":UNCALIBRATED})
}
