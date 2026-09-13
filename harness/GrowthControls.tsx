import { useEffect, useRef, useState } from 'react';
import type { GrowthSubmitted } from '../src/browser/render';

export interface GrowthControlsProps {
  age: number; frontier: number | null; disabled: boolean; failed: boolean;
  seek(age: number): void; rebuild(): void;
}
export function GrowthControls({ age, frontier, disabled, failed, seek, rebuild }: GrowthControlsProps) {
  const [playing, setPlaying] = useState(false);
  const [rate, setRate] = useState(1);
  const current = useRef({ age, seek });
  current.current = { age, seek };
  useEffect(() => { if (failed) setPlaying(false); }, [failed]);
  useEffect(() => {
    if (!playing || disabled) return;
    let previous = performance.now(), frame = 0;
    const tick = (now: number) => {
      const next = Math.min(1_000_000, current.current.age + (now - previous) / 1000 * rate);
      previous = now;
      current.current.seek(next);
      if (next < 1_000_000) frame = requestAnimationFrame(tick);
      else setPlaying(false);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, [playing, rate, disabled]);
  return <fieldset disabled={disabled} className="gd-growth">
    <legend>growth over time</legend>
    <label htmlFor="gd-age">age (years)</label>
    <input id="gd-age" type="number" min="0" max="1000000" step="0.25" value={age}
      onChange={e => { setPlaying(false); if (e.target.value !== '') seek(Number(e.target.value)); }} />
    <input aria-label="scrub age" type="range" min="0" max={Math.max(200, age, frontier ?? 0)} step="0.25" value={age}
      onChange={e => { setPlaying(false); seek(Number(e.target.value)); }} />
    <p data-growth-age={age} data-growth-frontier={frontier ?? ''}>frontier: {frontier?.toFixed(2) ?? 'building'} years</p>
    <button className="gd-button" onClick={() => { setPlaying(false); rebuild(); }}>rebuild at age</button>
    <label htmlFor="gd-growth-rate">years per second</label>
    <input id="gd-growth-rate" type="number" min="0.01" max="1000" step="0.25" value={rate}
      onChange={e => { const value = Number(e.target.value); if (Number.isFinite(value) && value > 0 && value <= 1000) setRate(value); }} />
    <button className="gd-button" onClick={() => setPlaying(p => !p)}>{playing ? 'pause growth' : 'play growth'}</button>
  </fieldset>;
}
export type { GrowthSubmitted };
