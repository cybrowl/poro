# Poro

**A local-first desktop AI coding workspace.**

Poro is being rebuilt as an open-source desktop interface for agentic coding. The product focus is no longer crypto payments, chat subscriptions, or hosted inference. The new direction is a calm, beautiful UI for working with local or bring-your-own-provider backends such as `claw-code`.

### Current Focus
- Desktop-first Svelte + Tauri shell
- Local-first `claw-code` runtime integration
- Ollama + Gemma 4 as the default no-API-key path
- Reusable visual primitives from the original Poro design language

### Repo Status
This repository has been cleaned up to remove the old ICP/chat/payment implementation so it can serve as the base for the new desktop app.

### Quick Start

```bash
npm install
npm run dev
```

Open `http://localhost:5173` to view the current UI shell.

### Local-First Desktop Flow

Poro is being shaped around this default stack:

- Poro UI
- `claw` runtime
- Ollama local server
- `gemma4` model in Ollama

To use the desktop app without a hosted API key:

```bash
ollama pull gemma4:e2b
```

Make sure your `claw` binary is installed or built locally, then run:

```bash
npm run tauri:dev
```

Inside Poro, leave the provider on `Ollama Local`, keep the model on `gemma4:e2b`, and point the backend path at `claw` or your local `claw` binary path.

### Project Structure
```
notes/             # Product, business, integration, and MVP planning docs
src/ui/            # SvelteKit frontend foundation for the desktop app
```

### Notes
- `notes/spec.md`: product definition
- `notes/market.md`: business and monetization direction
- `notes/integration.md`: `claw-code` and desktop integration approach
- `notes/plan.md`: MVP build order and next steps

### Community
- X: [@poro_app](https://x.com/poro_app)
- GitHub: [cybrowl/poro](https://github.com/cybrowl/poro)

---

**Built as a local-first UI for AI coding workflows**
