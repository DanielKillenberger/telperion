import { expect, it, vi } from "vitest";
import { latestBuild } from "./latest-build";

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}
const flush = async () => { await new Promise(resolve => setTimeout(resolve, 0)); };

it("serializes builds, coalesces pending families and publishes only the latest result", async () => {
  const first = deferred<string>();
  const last = deferred<string>();
  const run = vi.fn().mockReturnValueOnce(first.promise).mockReturnValueOnce(last.promise);
  const queue = latestBuild<string, string>(run);
  const success = vi.fn();
  const failure = vi.fn();
  queue.submit("oak-1", success, failure);
  queue.submit("spruce-2", success, failure);
  queue.submit("oak-7", success, failure);
  expect(run.mock.calls).toEqual([["oak-1"]]);
  first.resolve("old");
  await flush();
  expect(run.mock.calls).toEqual([["oak-1"], ["oak-7"]]);
  expect(success).not.toHaveBeenCalled();
  last.resolve("latest");
  await flush();
  expect(success.mock.calls).toEqual([["latest"]]);
  expect(failure).not.toHaveBeenCalled();
});

it("reports current errors and recovers, while cancelled effects suppress errors and queued work", async () => {
  const first = deferred<string>();
  const run = vi.fn().mockReturnValueOnce(first.promise).mockRejectedValueOnce("current error").mockResolvedValueOnce("recovered");
  const queue = latestBuild<string, string>(run);
  const success = vi.fn();
  const failure = vi.fn();
  const cancel = queue.submit("first", success, failure);
  cancel();
  const cancelPending = queue.submit("cancelled", success, failure);
  cancelPending();
  first.reject("stale error");
  await flush();
  expect(run).toHaveBeenCalledTimes(1);
  expect(failure).not.toHaveBeenCalled();
  queue.submit("invalid", success, failure);
  await flush();
  expect(failure.mock.calls).toEqual([["current error"]]);
  queue.submit("valid", success, failure);
  await flush();
  expect(success.mock.calls).toEqual([["recovered"]]);
});

it.each(["resolve", "reject"] as const)("disposal suppresses %s and prevents queued or subsequent builds", async outcome => {
  const active = deferred<string>();
  const run = vi.fn(() => active.promise);
  const queue = latestBuild<string, string>(run);
  const success = vi.fn();
  const failure = vi.fn();
  queue.submit("active", success, failure);
  queue.submit("pending", success, failure);
  queue.dispose();
  queue.submit("after disposal", success, failure);
  active[outcome]("finished");
  await flush();
  expect(run).toHaveBeenCalledTimes(1);
  expect(success).not.toHaveBeenCalled();
  expect(failure).not.toHaveBeenCalled();
});
