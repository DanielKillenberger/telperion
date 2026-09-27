/* ------------------------------------------------------------------ *
 * THE LOOK ON THE PANEL
 *
 * `?look=<name>` opens a look (`look.ts`) on the dials once the page is
 * up, says where it comes from, and switches between it and the preset
 * under it in place. Which of the two is drawn is read off the dials, so
 * a dial, a preset or reset that moves them off both says so. Accepting
 * a run's tree stays `species <id> --accept`.
 * ------------------------------------------------------------------ */

import { useEffect, useState } from "react";

import { fetchLook, showing, type Opened } from "./look";
import { normalizeSeed, type GrowerParams } from "./params";

export interface LookState {
  opened: Opened | null;
  error: string | null;
}

/** Opens the page's look, if it names one, through `apply`, which puts a
 *  tree on the dials and its seed in the seed box. `?seed=` wins over the
 *  look's own seed. */
export function useLook(apply: (params: GrowerParams) => void): LookState {
  const [opened, setOpened] = useState<Opened | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const query = new URLSearchParams(window.location.search);
    const name = query.get("look");
    if (name === null) return;
    let cancelled = false;
    fetchLook(name, query.get("species")).then(found => {
      if (cancelled) return;
      setOpened(found);
      const asked = normalizeSeed(query.get("seed") ?? "");
      apply({ ...found.look, seed: asked ?? found.look.seed });
    }, (failed: unknown) => { if (!cancelled) setError(failed instanceof Error ? failed.message : String(failed)); });
    return () => { cancelled = true; };
  }, [apply]);

  return { opened, error };
}

/** Where the look comes from, which tree the dials hold, and the switch
 *  between the kept tree and the preset under it, at the seed shown. */
export function LookControls({ state, params, apply }: {
  state: LookState; params: GrowerParams; apply: (params: GrowerParams) => void;
}) {
  const { opened, error } = state;
  if (error !== null) return <div role="alert" className="gd-note gd-warn">{error}</div>;
  if (opened === null) return null;
  const s = opened.source;
  const from = s === null
    ? `a plain overlay over ${opened.preset}, from no run`
    : `run ${s.run ?? "unnamed"}, revision ${s.revision ?? "unknown"}, round ${s.round ?? "?"} (${s.label ?? "no label"}), ` +
      `${s.kept ?? "?"} kept, tree ${s.key?.slice(0, 12) ?? "unkeyed"}, over ${opened.preset}`;
  const shown = showing(opened, params);
  const drawn = shown === "look" ? "drawn: the kept tree" : shown === "shipped" ? `drawn: ${opened.preset}` :
    "drawn: neither; the dials have moved off the look and the preset";
  const show = (tree: GrowerParams): void => apply({ ...tree, seed: params.seed });
  return (
    <fieldset>
      <legend>look {opened.name}</legend>
      <p className="gd-note">{from}. accept with species &lt;id&gt; --accept.</p>
      <p role="status" className={shown === null ? "gd-note gd-warn" : "gd-note"}>{drawn}</p>
      <div className="gd-row">
        <button className="gd-button" type="button" aria-pressed={shown === "look"} onClick={() => show(opened.look)}>
          kept tree
        </button>
        <button className="gd-button" type="button" aria-pressed={shown === "shipped"} onClick={() => show(opened.shipped)}>
          {opened.preset}
        </button>
      </div>
    </fieldset>
  );
}
