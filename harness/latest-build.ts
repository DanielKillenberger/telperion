/** One in-flight build and one replaceable pending request per renderer. */
export function latestBuild<Input, Output>(run: (input: Input) => Promise<Output>) {
  type Request = { input: Input; success: (output: Output) => void; failure: (error: unknown) => void; cancelled: boolean };
  let latest: Request | undefined;
  let pending: Request | undefined;
  let running = false;
  let disposed = false;

  const drain = async (): Promise<void> => {
    if (running || disposed) return;
    running = true;
    try {
      while (pending && !disposed) {
        const request = pending;
        pending = undefined;
        if (request.cancelled) continue;
        const current = () => !disposed && !request.cancelled && latest === request;
        try {
          const output = await run(request.input);
          if (current()) request.success(output);
        } catch (error) {
          if (current()) request.failure(error);
        }
      }
    } finally {
      running = false;
    }
  };

  return {
    submit(input: Input, success: (output: Output) => void, failure: (error: unknown) => void): () => void {
      const request: Request = { input, success, failure, cancelled: false };
      if (!disposed) {
        latest = pending = request;
        void drain();
      }
      return () => { request.cancelled = true; };
    },
    dispose(): void {
      disposed = true;
      latest = pending = undefined;
    },
  };
}
