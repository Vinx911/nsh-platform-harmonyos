# Counter

An NSH app written in Rust (`src/lib.rs`; `nsh.json` names the root, `"root": "App"`), packaged for HarmonyOS:

```sh
nsh package examples/counter --target harmonyos            # a DevEco Studio project in .nsh/out/harmonyos
nsh run examples/counter --target harmonyos                # installs and starts it on the device hdc sees
```

The same app runs on NSH's own platforms too (`nsh run`), and its test headless (`nsh test`).
