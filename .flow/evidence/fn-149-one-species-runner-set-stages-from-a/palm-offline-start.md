# Date palm offline check (R6, owner 2026-09-25)

Free steps only; no Jev or vision call was made.

- `node scripts/catalogue-check.mjs`: exit 0, "6 species pass the structure check" (the shipped `catalogue/date-palm` folder included).
- `species date-palm --stage start` over a copy of `catalogue/date-palm` (its recorded `packet/profile.json`) and a three-key tuning config (`preset`, `profile_id`, `profiles`): "derived 19 values (2 left out, 0 manual)". The comparison below is that overlay against the shipped `date-palm` preset (`params::metadata`).
- Not run: `--adapter fixture:<recorded> --until start`. The Sources and Profile stages call Jev (discover, screen, quality, select, verify), and none of the fn-80 run's stage records is current under the new keys, so every one would rerun and spend paid calls. The recorded cache is on the fn-80 worktree (`.flow/evidence/date-palm/pipeline/cache`, 71 MB with the run), not in this checkout.

## Start overlay against the shipped preset

| Dial | From | Shipped | Start | Difference |
|---|---|---|---|---|
| `/material/barkBlue` | appearance.bark_colour.bark_blue (A1; midpoint) | 0.135 | 0.21 | +0.075 (+56%) |
| `/material/barkGreen` | appearance.bark_colour.bark_green (A1; midpoint) | 0.1975 | 0.23500000000000001 | +0.0375 (+19%) |
| `/material/barkRed` | appearance.bark_colour.bark_red (A1; midpoint) | 0.325 | 0.25 | -0.075 (-23%) |
| `/material/barkRoughness` | appearance.bark_roughness.bark_roughness (A1; midpoint) | 0.7775 | 0.9650000000000001 | +0.1875 (+24%) |
| `/material/leafBackBlue` | appearance.leaf_back_colour.leaf_back_blue (F1; midpoint) | 0.11625 | 0.11000000000000001 | -0.00625 (-5%) |
| `/material/leafBackGreen` | appearance.leaf_back_colour.leaf_back_green (F1; midpoint) | 0.15375 | 0.16 | +0.00625 (+4%) |
| `/material/leafBackRed` | appearance.leaf_back_colour.leaf_back_red (F1; midpoint) | 0.12 | 0.12 | +0 (+0%) |
| `/material/brightnessRangeHigh` | appearance.leaf_brightness_range.brightness_range_high (; midpoint) | 0.03625 | 0.04 | +0.00375 (+10%) |
| `/material/brightnessRangeLow` | appearance.leaf_brightness_range.brightness_range_low (; midpoint) | -0.04 | -0.04 | +0 (-0%) |
| `/material/leafFrontBlue` | appearance.leaf_front_colour.leaf_front_blue (F1; midpoint) | 0.1175 | 0.11000000000000001 | -0.0075 (-6%) |
| `/material/leafFrontGreen` | appearance.leaf_front_colour.leaf_front_green (F1; midpoint) | 0.16 | 0.16 | +0 (+0%) |
| `/material/leafFrontRed` | appearance.leaf_front_colour.leaf_front_red (F1; midpoint) | 0.12 | 0.12 | +0 (+0%) |
| `/material/hueRangeHigh` | appearance.leaf_hue_range.hue_range_high (F1; midpoint) | 0.0275 | 0.0275 | +0 (+0%) |
| `/material/hueRangeLow` | appearance.leaf_hue_range.hue_range_low (F1; midpoint) | -0.0275 | -0.0275 | +0 (-0%) |
| `/radii/trunkRadius` | metrics.dbh_m (A1; ratio) | 0.013 | 0.01 | -0.003 (-23%) |
| `/canopy/rachisLength` | metrics.frond_length_m (A1; midpoint) | 7.0 | 6.0 | -1 (-14%) |
| `/skeleton/envelope/height` | metrics.height_m (A1; midpoint) | 22.86 | 22.86 | +0 (+0%) |
| `/element/length` | metrics.leaflet_length_m (A1; ratio) | 0.6197 | 0.1945531914893617 | -0.4251 (-69%) |
| `/element/width` | metrics.leaflet_width_m (P4; ratio) | 0.0725 | 0.00851063829787234 | -0.06399 (-88%) |

Left out: metrics.crown_width_m: the family meets no condition of /skeleton/envelope/spread; metrics.height_growth_m_per_year: no row in the table
