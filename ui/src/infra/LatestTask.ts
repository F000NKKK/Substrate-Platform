/** Serializes expensive edits while retaining the latest waiting edit throughout a drag. */
export class LatestTask {
  private busy = false;
  private pending: (() => Promise<void>) | null = null;

  constructor(private readonly onError: (error: unknown) => void) {}

  enqueue(task: () => Promise<void>) {
    this.pending = task;
    if (!this.busy) void this.drain();
  }

  /** Drop waiting work when its document or selection is no longer active. */
  cancelPending() { this.pending = null; }

  private async drain() {
    this.busy = true;
    try {
      while (this.pending) {
        const task = this.pending;
        this.pending = null;
        try { await task(); }
        catch (error) { this.onError(error); }
      }
    } finally { this.busy = false; }
  }
}
