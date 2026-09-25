//! fn-119 on the date palm's recorded state: its fn-80 reference inventory,
//! its owner priorities and its two tracks. No render, no paid look, no Jev
//! call; the stills are stand-in bytes with the palm's views.
use serde_json::{json, Value};
use std::path::PathBuf;
use telperion_jev::{
    sha256_hex,
    tuning::{
        bundle::Track,
        continuation::HumanDecision,
        engine::Run,
        evaluation::Image,
        objectives::{self, LeftOut},
        priority::{scope, Approval, Checkpoint, Evidence, Gap},
        progress,
        reference_first::Inventory,
        state::{Cell, Visual},
        stride,
        unexpressed::Unexpressed,
    },
};

const VIEWS: [&str; 3] = ["P-WHOLE", "P-TRUNK", "P-BASE"];

/// A still at `view` with its own bytes, hash-checked like any image.
fn image(tag: &str, view: &str) -> Image {
    // One directory per test process: a fixed path collided with the same
    // suite running in another checkout (fn-80 and fn-130 gates, 2026-09-23).
    let dir = std::env::temp_dir().join(format!("fn119-objectives-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path: PathBuf = dir.join(format!("{tag}.png"));
    std::fs::write(&path, tag.as_bytes()).unwrap();
    Image {
        path,
        sha256: sha256_hex(tag.as_bytes()),
        view: view.into(),
        seed: 1,
    }
}

/// The palm's inventory as recorded, its three photographs swapped for
/// stand-in bytes at the same views.
fn inventory() -> Inventory {
    let mut raw: Value =
        serde_json::from_str(include_str!("fixtures/fn80-palm-inventory.json")).unwrap();
    for shot in raw["request"]["references"].as_array_mut().unwrap() {
        let id = shot["id"].as_str().unwrap().to_string();
        let view = shot["image"]["view"].as_str().unwrap().to_string();
        shot["image"] = serde_json::to_value(image(&id, &view)).unwrap();
    }
    serde_json::from_value(raw).unwrap()
}

/// One render per assessed view, then the three references.
fn evidence(inventory: &Inventory) -> Vec<Evidence> {
    let renders = VIEWS.iter().map(|view| Evidence {
        id: format!("render-{view}"),
        role: "render".into(),
        image: image(&format!("render-{view}"), view),
    });
    let references = inventory.request.references.iter().map(|r| Evidence {
        id: r.id.clone(),
        role: "reference".into(),
        image: r.image.clone(),
    });
    renders.chain(references).collect()
}

fn required() -> Vec<Cell> {
    VIEWS
        .iter()
        .map(|view| Cell {
            item: "date palm character".into(),
            view: (*view).into(),
            seed: 1,
        })
        .collect()
}

fn fruit() -> Vec<Unexpressed> {
    vec![Unexpressed {
        trait_id: "fruit-clusters-pendent".into(),
        spec: "fn-111-the-palms-infructescence-a-hanging-date".into(),
    }]
}

/// The owner's three priorities from the palm's second revision.
fn owner() -> Vec<Gap> {
    [
        ("owner-fronds-long-arching", "long arching pinnate fronds"),
        ("owner-trunk-straight-constant", "a straight trunk"),
        ("owner-trunk-bare", "nothing grows on the trunk"),
    ]
    .iter()
    .map(|(id, observation)| Gap {
        id: (*id).into(),
        observation: (*observation).into(),
        evidence_ids: vec!["render-P-WHOLE".into(), "reference-0".into()],
        views: vec!["P-WHOLE".into()],
        track: None,
    })
    .collect()
}

/// The pause's checkpoint, carrying the proposed approval.
fn checkpoint(inventory: &Inventory) -> (Checkpoint, String) {
    let evidence = evidence(inventory);
    let visual: Visual = serde_json::from_value(json!({"identity":"palm-run-3","model":"reviewer",
        "ledger":"review","cells":[],"defects":[],"findings":[]}))
    .unwrap();
    let references = inventory
        .request
        .references
        .iter()
        .map(|r| r.image.clone())
        .collect::<Vec<_>>();
    let scope = scope("date-palm", "owner notes", &required(), &references);
    let mut checkpoint = Checkpoint::new("palm", &scope, visual, evidence.clone()).unwrap();
    let (traits, left_out) =
        objectives::from_inventory(inventory, &fruit(), &required(), &evidence);
    checkpoint.proposed = objectives::propose(owner(), traits);
    checkpoint.left_out = left_out;
    checkpoint.verify().unwrap();
    (checkpoint, scope)
}

fn ids(gaps: &[Gap]) -> Vec<&str> {
    gaps.iter().map(|g| g.id.as_str()).collect()
}

/// R1: the owner's priorities, then every expressible core and secondary
/// trait in inventory order; the unexpressed date cluster and the four
/// variation traits are recorded, not aimed at.
#[test]
fn the_palm_s_proposed_approval_is_the_owner_s_priorities_then_every_drawable_trait() {
    let inventory = inventory();
    let (checkpoint, scope) = checkpoint(&inventory);
    assert_eq!(
        ids(&checkpoint.proposed),
        vec![
            "owner-fronds-long-arching",
            "owner-trunk-straight-constant",
            "owner-trunk-bare",
            "crown-pinnate-arching-fronds",
            "leaflet-arrangement-stiff-narrow",
            "trunk-leaf-base-diamond-pattern",
            "trunk-fibrous-matting",
            "solitary-columnar-stem",
            "dead-frond-skirt",
            "petiole-base-orange-sheath",
            "trunk-colour-and-weathering",
            "basal-flare",
            "crown-shaft-absent",
            "basal-leaflet-spines",
        ]
    );
    let variation = "variation: recorded, not an objective";
    let left = |id: &str, reason: &str| LeftOut {
        trait_id: id.into(),
        reason: reason.into(),
    };
    assert_eq!(
        checkpoint.left_out,
        vec![
            left(
                "fruit-clusters-pendent",
                "unexpressed until fn-111-the-palms-infructescence-a-hanging-date lands"
            ),
            left("frond-colour-range", variation),
            left("trunk-lean-and-curve", variation),
            left("epiphytes-on-trunk", variation),
            left("crown-silhouette-proportions", variation),
        ]
    );
    // A trait is judged at the shots of its references, with the current
    // render of each and the references it cites.
    let colour = checkpoint
        .proposed
        .iter()
        .find(|g| g.id == "trunk-colour-and-weathering")
        .unwrap();
    assert_eq!(colour.views, VIEWS);
    assert_eq!(
        colour.evidence_ids,
        vec![
            "render-P-WHOLE",
            "render-P-TRUNK",
            "render-P-BASE",
            "reference-0",
            "reference-1",
            "reference-2"
        ]
    );

    // Approved as offered, fourteen objectives, past the old cap of eight.
    let approval = Approval {
        checkpoint_sha256: checkpoint.hash(),
        scope_sha256: scope.clone(),
        ordered: checkpoint.proposed.clone(),
    };
    approval.verify(&checkpoint, &scope).unwrap();
    // A trait reworded is not the trait offered.
    let mut reworded = approval.clone();
    reworded.ordered[4].observation = "something else".into();
    assert!(reworded.verify(&checkpoint, &scope).is_err());
    // A left-out trait cannot come back under its own id.
    let mut variation_back = approval.clone();
    variation_back.ordered.push(Gap {
        id: "frond-colour-range".into(),
        ..colour.clone()
    });
    assert!(variation_back.verify(&checkpoint, &scope).is_err());
}

fn palm_tracks() -> Vec<Track> {
    let track = |name: &str, groups: &[&str], view: Option<&str>| Track {
        name: name.into(),
        groups: groups.iter().map(|g| (*g).to_string()).collect(),
        view: view.map(str::to_string),
        extra_views: view.map(str::to_string).into_iter().collect(),
    };
    vec![
        track(
            "structure",
            &[
                "canopy",
                "element",
                "radii",
                "shellDepth",
                "skeleton",
                "surface",
            ],
            None,
        ),
        track("materials", &["material"], Some("P-BASE")),
    ]
}

/// The palm's approval with the owner's notes on the structure track and the
/// trunk's materials on the materials track, reordered so the colour leads.
fn palm_run() -> Run {
    let inventory = inventory();
    let (checkpoint, scope) = checkpoint(&inventory);
    let mut ordered = checkpoint.proposed.clone();
    let materials = ["trunk-colour-and-weathering", "trunk-fibrous-matting"];
    ordered.sort_by_key(|g| {
        let rank = materials.iter().position(|m| *m == g.id);
        (!g.id.starts_with("owner-"), rank.unwrap_or(usize::MAX))
    });
    for gap in &mut ordered {
        if gap.id.starts_with("owner-") {
            gap.track = Some("structure".into());
        } else if materials.contains(&gap.id.as_str()) {
            gap.track = Some("materials".into());
        }
    }
    let approval = Approval {
        checkpoint_sha256: checkpoint.hash(),
        scope_sha256: scope.clone(),
        ordered: ordered.clone(),
    };
    approval.verify(&checkpoint, &scope).unwrap();
    objectives::verify_tracks(&ordered, &palm_tracks()).unwrap();
    let decision: HumanDecision = serde_json::from_value(json!({"pause_id":"p","identity":"palm",
        "action":"approve gap priorities","by":"owner","rationale":"recorded palm state",
        "preserve_evidence":true,"priority_approval":approval}))
    .unwrap();
    let mut state: Run = serde_json::from_value(json!({"identity":"palm","preset":"date-palm",
        "seed":1,"effective":{},"overrides":{},"dials":[],"owner_notes":"owner notes",
        "required":required(),"budget":{"evaluations":0,"images":0,"tokens":0,"rounds":0},
        "usage_known":true,"trials":[],"current":null,"visual":null,"pause":null,
        "machine_ready":false,"pending":null,"routes":[]}))
    .unwrap();
    state.routes = ordered.iter().map(|g| format!("{}=tuning", g.id)).collect();
    state.priority_checkpoints = vec![checkpoint];
    state.authorizations = vec![decision];
    state
}

/// R2: the materials track leads with the trunk's colour, not the fronds the
/// structure track leads with, so it is no longer drawn at their class.
#[test]
fn the_palm_s_materials_track_leads_with_its_own_objective() {
    let state = palm_run();
    let tracks = palm_tracks();
    let lead = |i: usize| stride::lead(&state, &tracks[i]).map(|g| g.id);
    assert_eq!(lead(0).as_deref(), Some("owner-fronds-long-arching"));
    assert_eq!(lead(1).as_deref(), Some("trunk-colour-and-weathering"));

    let materials = progress::track_priorities(&state, &tracks[1]);
    assert!(materials.iter().all(|g| !g.id.starts_with("owner-")));
    assert_eq!(materials.len(), 11, "its two, then every unassigned trait");
    let structure = progress::track_priorities(&state, &tracks[0]);
    assert!(!structure.iter().any(|g| g.id == "trunk-fibrous-matting"));
    assert_eq!(structure.len(), 12, "the owner's three and nine unassigned");
    assert!(materials.len() <= progress::MAX_PRIORITIES);
    assert!(structure.len() <= progress::MAX_PRIORITIES);
}
