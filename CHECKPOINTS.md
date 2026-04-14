# Poro Checkpoints

The poro repo now has a local checkpoint flow that writes compressed archives into:

- `/Users/cyberowl/Repos/poro_checkpoint`

The checkpoint script is:

- [checkpoint_poro.sh](/Users/cyberowl/Repos/poro/scripts/checkpoint_poro.sh)

It creates:

- `archives/*.tar.gz`
- `manifests/*.json`
- `latest.tar.gz`
- `latest.json`

Important behavior:

- includes the repo contents and `.git`
- excludes `node_modules`
- excludes `build`
- excludes `.svelte-kit`
- excludes `.dfx`
- excludes `.harness`

Manual checkpoint:

```bash
cd /Users/cyberowl/Repos/poro
./scripts/checkpoint_poro.sh
```

Install the recurring checkpoint job:

```bash
cd /Users/cyberowl/Repos/poro
./scripts/install_poro_checkpoint_launchd.sh
```

Check status:

```bash
cd /Users/cyberowl/Repos/poro
./scripts/status_poro_checkpoint_launchd.sh
```

Default cadence:

- every 6 hours
- plus one run immediately when the job is installed
