import type { TimingReport } from "../src/browser/render";

/** The timing session as the panel says it, and the old sweep's honesty
 *  rule kept word for word: WITHOUT A VALID VERDICT THERE IS NO
 *  MILLISECOND FIGURE IN THIS STRING. An invalid session carries no
 *  number that could be read as a pass, so there is none to print. */
export function describeTiming(report: TimingReport): string {
  const who = `${report.adapter} (${report.backend})`;
  if (report.verdict !== "valid" || report.p50_ms === undefined || report.p95_ms === undefined) {
    return `${report.verdict}: ${report.reason ?? "no reason given"} - no frame time reported, on ${who}`;
  }
  return `vegetation p50 ${report.p50_ms.toFixed(3)} ms, p95 ${report.p95_ms.toFixed(3)} ms over ${report.samples} frames on ${who} - valid`;
}
