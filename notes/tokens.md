# Token use

One row per measured session, newest last (`python3 tools/tokens.py`). Each call re-reads the whole context, so re-read ≈ mean context × calls.

| When | Session | Calls | Context at first plan | Peak | Mean | Compactions | Re-read | Above 200k |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2026-11-12 08:49 | be6be7a2 | 269 | 198k (call 71) | 414k | 222k | 1 | 59.1M | 26% |
