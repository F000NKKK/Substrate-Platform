import { expect, it, vi } from "vitest";
import { LatestTask } from "./LatestTask";

it("retains the final drag edit when more edits arrive during the second request", async () => {
  const calls: string[] = [], finish: (() => void)[] = [];
  const queue = new LatestTask(error => { throw error; });
  const task = (name: string) => async () => {
    calls.push(name);
    await new Promise<void>(resolve => finish.push(resolve));
  };
  queue.enqueue(task("first")); queue.enqueue(task("second"));
  finish[0](); await Promise.resolve(); await Promise.resolve();
  queue.enqueue(task("intermediate")); queue.enqueue(task("final"));
  finish[1](); await Promise.resolve(); await Promise.resolve();
  expect(calls).toEqual(["first", "second", "final"]);
  finish[2]();
});

it("continues accepting edits after a request fails", async () => {
  const report = vi.fn(), applied = vi.fn();
  const queue = new LatestTask(report);
  queue.enqueue(async () => { throw new Error("failed edit"); });
  queue.enqueue(async () => { applied(); });
  await Promise.resolve(); await Promise.resolve();
  expect(report).toHaveBeenCalledOnce();
  expect(applied).toHaveBeenCalledOnce();
});

it("drops waiting edits when their document is closed", async () => {
  let finish!: () => void;
  const stale = vi.fn();
  const queue = new LatestTask(() => {});
  queue.enqueue(() => new Promise<void>(resolve => { finish = resolve; }));
  queue.enqueue(async () => { stale(); });
  queue.cancelPending(); finish();
  await Promise.resolve(); await Promise.resolve();
  expect(stale).not.toHaveBeenCalled();
});
