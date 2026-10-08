# fn-228 friction

## 2026-10-08, worker, task 1

- **Doing:** running the R1 measurement with its log redirected through a shell variable path.
- **What slowed it:** the local dcg hook refused the redirect because its target was a variable. The command had to be rewritten with the literal path.
- **Cost:** about one minute.
- **What would have removed it:** nothing in the repository. This is local hook configuration, so it is reported, not specced.
