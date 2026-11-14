"use client";

/** The mark and the reset of a control that differs from the preset. */
export function Changed({ label, onReset }: { label: string; onReset: () => void }) {
  return <button className="gd-reset" type="button" title={`back to the preset's ${label}`}
    aria-label={`reset ${label}`} onClick={onReset}>changed · reset</button>;
}
