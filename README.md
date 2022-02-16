# Reproduces [Issue 3484](https://github.com/tauri-apps/tauri/issues/3484)

To run repository, make sure you have the [Tauri prerequisites](https://tauri.studio/docs/getting-started/prerequisites).

Install dependencies

```bash
npm i
```

Run project

```bash
npm run tauri dev
```

### Expected behavior

1. Rust sends `bootstrap` message to JS
1. JS listens for `bootstrap` event
1. App loads and renders

### Actual behavior

1. Rust sends `bootstrap` message to JS
1. JS fails to listen for `bootstrap` event, _it was sent before JS was ready_
1. App fails to load, as bootstrap config is not yet ready

### Quick fix

[main.rs](./src-tauri/src/main.rs) has some commented code, that "fixes" the issue, by adding a timeout for the event, so JS is ready when Rust sends the event.
