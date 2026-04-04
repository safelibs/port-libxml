# Performance Harness

`safe/benchmarks/original-baseline.json` freezes the local original-libxml timing and
streaming-memory baseline for the workloads already encoded in `original/Makefile.am`
and `original/testSAX.c`.

Use:

```bash
safe/scripts/run-upstream-tests.sh performance
safe/scripts/verify-performance.sh --baseline safe/benchmarks/original-baseline.json original safe/target/stage
```

Rules:

- The baseline is only created if it does not already exist.
- The collector refuses to create or refresh the original baseline unless
  `safe/abi/baseline/original-oracles.txt` already records both
  `original/dba100000.xml` and `original/testSAX`.
- After this phase, treat `safe/benchmarks/original-baseline.json` as read-only.
