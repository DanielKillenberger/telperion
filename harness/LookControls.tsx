/* ------------------------------------------------------------------ *
 * THE LOOK ON THE PANEL
 *
 * `?look=<name>` opens a look (`look.ts`) on the dials once the page is
 * up, says where it comes from, and switches between it and the preset
 * under it in place. Read-only: the dials still move, but accepting a
 * run's tree stays `species <id> --accept`.
 * ------------------------------------------------------------------ */

import { useEffect, useState } from "react";

import { fetchLook, type Opened } from "./look";
import type { GrowerParams } from "./params";

export interface LookState {
  opened: Opened | null;
  error: string | null;
  showing: "look" | "shipped";
  show: (which: "look" | "shipped") => void;
}

/** Opens the page's look, if it names one, and keeps which tree is shown.
 *  A switch keeps the seed the seed box holds. */
export function useLook(setParams: (next: (prev: GrowerParams) => GrowerParams) => void, frame: () => void): LookState {
  const [opened, setOpened] = useState<Opened | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showing, setShowing] = useState<"look" | "shipped">("look");

  useEffect(() => {
    const query = new URLSearchParams(window.location.search);
    const name = query.get("look");
    if (name === null) return;
    let cancelled = false;
    fetchLook(name, query.get("species")).then(found => {
      if (cancelled) return;
      setOpened(found);
      setParams(prev => ({ ...found.look, seed: query.has("seed") ? prev.seed : found.look.seed }));
      frame();
    }, (failed: unknown) => { if (!cancelled) setError(failed instanceof Error ? failed.message : String(failed)); });
    return () => { cancelled = true; };
  }, [setParams, frame]);

  const show = (which: "look" | "shipped"): void => {
    if (opened === null) return;
    setShowing(which);
    setParams(prev => ({ ...(which === "look" ? opened.look : opened.shipped), seed: prev.seed }));
  };
  return { opened, error, showing, show };
}

/** Where the look comes from, and the switch to the preset under it. */
export function LookControls({ state }: { state: LookState }) {
  const { opened, error, showing, show } = state;
  if (error !== null) return <div role="alert" className="gd-note gd-warn">{error}</div>;
  if (opened === null) return null;
  const s = opened.source;
  const from = s === null
    ? `a plain overlay over ${opened.preset}, from no run`
    : `run ${s.run ?? "unnamed"}, revision ${s.revision ?? "unknown"}, round ${s.round ?? "?"} (${s.label ?? "no label"}), ` +
      `${s.kept ?? "?"} kept, tree ${s.key?.slice(0, 12) ?? "unkeyed"}, over ${opened.preset}`;
  return (
    <fieldset>
      <legend>look {opened.name}</legend>
      <p className="gd-note">{from}. accept with species &lt;id&gt; --accept.</p>
      <div className="gd-row">
        <button className="gd-button" type="button" aria-pressed={showing === "look"} onClick={() => show("look")}>
          kept tree
        </button>
        <button className="gd-button" type="button" aria-pressed={showing === "shipped"} onClick={() => show("shipped")}>
          {opened.preset}
        </button>
      </div>
    </fieldset>
  );
}
