import { execFileSync } from "node:child_process";

/* The native release binaries the harness tests call, built once, one
   after the other, before any test file runs: two test files building in
   parallel wait on cargo's lock and time each other out. */
export default function setup(): void {
  const builds = [
    ["-p", "telperion-core", "--example", "node_buffer"],
    ["-p", "telperion-jev", "--bin", "species"],
  ];
  for (const target of builds) {
    execFileSync("cargo", ["build", "--release", ...target], { stdio: "inherit" });
  }
}
