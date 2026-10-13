---
title: One heavy run at a time across workers; 8 GB free first
date: "2026-10-12"
track: knowledge
category: conventions
tags: [memory, owner, workers]
applies_when: dispatching workers that build trees or run gates
decision_status: accepted
last_updated: "2026-10-13"
related_to: [knowledge/conventions/reset-review-round-caps-without-asking-2026-10-09]
---

The owner (2026-10-12), with the desktop lagging: "are you hogging my drive or smth>? pc lagging like crazy". Two workers ran full-tree CPU builds at once (oak 80 at 3 GB, Telperion toward 5 GB) beside another project's 7 GB process; free memory fell to 2 GB and the machine swapped.

**Why:** the owner works on this desktop; a parallel heavy run makes it unusable.

**How to apply:** every worker brief says: before a heavy run, check `free -g`; start only with at least 8 GB available and no other telperion example or test binary running; one tree at a time across all workers. The host runs at most one heavy worker phase at a time.

## Update 2026-10-12

The owner (2026-10-12), with the desktop lagging: "are you hogging my drive or smth>? pc lagging like crazy". Two workers ran full-tree CPU builds at once (oak 80 at 3 GB, Telperion toward 5 GB) beside another project's 7 GB process; free memory fell to 2 GB and the machine swapped.

**Why:** the owner works on this desktop; a parallel heavy run makes it unusable.

**How to apply:** every worker brief says: before a heavy run, check free memory; start only with at least 8 GB available and no other telperion example or test binary running; one tree at a time across all workers. A checked gate script exists on branch fn-245-field-spike as `scripts/heavy-gate.sh` (it matches each process's executable, so it never waits on itself); carry it to F4 with the next merge. The host runs at most one heavy worker phase at a time.

## Update 2026-10-13

Owner (2026-10-12, refined 2026-10-13: "doesn't completely block the machine but if there is capacity you can run multiple things").

Every heavy run (a workspace gate, an example over a few GB, a GPU build) goes through `telperion-heavy <mem-GB> [--gpu <vram-GB>] [--threads N] -- <command>` (machine-local, `~/.local/bin`). It admits the job only when available RAM minus a 6 GB desktop reserve minus what running heavy jobs may still claim covers the declared memory, and with `--gpu` when free VRAM covers the job and other processes hold under half the card (the owner games). The job runs in a user scope capped at its declared memory with no swap (over the cap it is killed, it never thrashes the desk), low CPU and IO weight, nice 10, 12 threads by default. Several heavy jobs may run at once when they fit.

**Why:** two parallel unbounded CPU builds swapped the desktop into lag on 2026-10-12; one-at-a-time then idled the machine.

**How to apply:** every worker brief names the wrapper and a declared memory per heavy command (the gate about 12 GB; Telperion examples about 8 GB); a job killed at its cap is re-run with a larger declaration, never unwrapped.

## Update 2026-10-13

Owner (2026-10-12, refined 2026-10-13: "doesn't completely block the machine but if there is capacity you can run multiple things").

Every heavy run (a workspace gate, an example over a few GB, a GPU build) goes through `telperion-heavy <mem-GB> [--gpu <vram-GB>] [--threads N] -- <command>` (machine-local, `~/.local/bin`). It admits the job only when available RAM minus a 6 GB desktop reserve minus what running heavy jobs may still claim covers the declared memory, and with `--gpu` when free VRAM covers the job and other processes hold under half the card (the owner games). The job runs in a user scope capped at its declared memory with no swap (over the cap it is killed, it never thrashes the desk), low CPU and IO weight, nice 10, 12 threads by default. Several heavy jobs may run at once when they fit.

**Why:** two parallel unbounded CPU builds swapped the desktop into lag on 2026-10-12; one-at-a-time then idled the machine.

**How to apply:** every worker brief names the wrapper and a declared memory per heavy command (the workspace gate 20 GB, its core lib peaking at 17-18 GB, so a gate runs alone; Telperion examples about 8 GB); a job killed at its cap is re-run with a larger declaration, never unwrapped.

## Update 2026-10-13

Owner (2026-10-12, refined 2026-10-13: "doesn't completely block the machine but if there is capacity you can run multiple things").

Every heavy run (a workspace gate, an example over a few GB, a GPU build) goes through `telperion-heavy <mem-GB> [--gpu <vram-GB>] [--threads N] -- <command>` (machine-local, `~/.local/bin`). It admits the job only when available RAM minus a 6 GB desktop reserve minus what running heavy jobs may still claim covers the declared memory, and with `--gpu` when free VRAM covers the job and other processes hold under half the card (the owner games). The job runs in a user scope capped at its declared memory with no swap (over the cap it is killed, it never thrashes the desk), low CPU and IO weight, nice 10, 12 threads by default. Several heavy jobs may run at once when they fit.

**Why:** two parallel unbounded CPU builds swapped the desktop into lag on 2026-10-12; one-at-a-time then idled the machine.

**How to apply:** every worker brief names the wrapper and a declared memory per heavy command (the wrapper caps test threads at its thread count, 12 by default, so the workspace gate declares 12 GB, rising only if killed at the cap: at 32 test threads its core lib peaked at 17-18 GB; Telperion examples about 8 GB); a job killed at its cap is re-run with a larger declaration, never unwrapped.

## Update 2026-10-13

Owner (2026-10-12, refined 2026-10-13: "doesn't completely block the machine but if there is capacity you can run multiple things").

Every heavy run (a workspace gate, an example over a few GB, a GPU build) goes through `telperion-heavy <mem-GB> [--gpu <vram-GB>] [--threads N] -- <command>` (machine-local, `~/.local/bin`). It admits the job only when available RAM minus a 4 GB desktop reserve (`TELPERION_HEAVY_RESERVE`) minus what running heavy jobs may still claim covers the declared memory, and with `--gpu` when free VRAM covers the job and other processes hold under half the card (the owner games). The job runs in a user scope capped at its declared memory with no swap (over the cap it is killed, it never thrashes the desk), low CPU and IO weight, nice 10, 12 threads by default. Several heavy jobs may run at once when they fit.

**Why:** two parallel unbounded CPU builds swapped the desktop into lag on 2026-10-12; one-at-a-time then idled the machine.

**How to apply:** every worker brief names the wrapper and a declared memory per heavy command (the wrapper caps test threads at its thread count, 12 by default, so the workspace gate runs at `--threads 4` declared 9 GB (measured: at 8 threads `telperion_core`'s unit tests pass 7.26 GB; at 32 they peaked at 17-18 GB); Telperion examples about 8 GB); a job killed at its cap is re-run with a larger declaration, never unwrapped.
