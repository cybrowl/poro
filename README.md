# Poro

**Privacy-first, borderless AI access on the Internet Computer.**

Pay once with USDC — get fast, private access to open-source LLMs and image models without credit cards or centralized tracking.

### Why Poro?
- **$4.20/month** — affordable for everyone, especially the unbanked.
- **True privacy** — on-chain subscriptions, rate limiting, and authentication. Prompts never touch a centralized database.
- **Hybrid speed** — fast off-chain inference + fully on-chain control layer (payments, usage, agents).
- **Built on ICP** — Internet Identity login, USDC payments, and persistent storage that scales to 500 GiB in a single canister.
- **Open source** — core canister and frontend are public.

### Current Status (March 2026)
- **User Canister** — live in Motoko (`persistent actor` + `mo:core/Map`) with full subscription management, daily rate limiting, and usage tracking.
- **Frontend** — SvelteKit + TypeScript with Carta-powered Markdown editor (click anywhere to focus).
- **AI Proxy** — working unified endpoint (Hyperbolic + future multi-model routing).
- **Wallet UI** — visual guide for ckUSDC flows and components ready.

### Features
- Internet Identity login
- $4.20/month USDC subscription (ckUSDC)
- Free tier + paid tier with automatic daily reset
- On-chain rate limiting
- Unified API for multiple LLMs
- Privacy-focused chat interface
- Open-source core canister

### Quick Start

```bash
# Clone the repo
git clone https://github.com/cybrowl/poro.git
cd poro

# Install dependencies
npm install
mops install   # for Motoko core

# Start local replica (PocketIC)
dfx start --clean --background

# Deploy canisters
dfx deploy

# Start frontend
npm run dev
```

Open `http://localhost:5173` to try the editor.

### Project Structure
```
src/
├── user/          # Motoko canister (subscriptions + rate limiting)
├── ui/            # SvelteKit + TypeScript frontend
└── ai_vercel/     # Proxy endpoint for inference
```

### Roadmap
**Q2 2026**
- Public beta with unified API
- Real ckUSDC ICRC-2 payment integration
- Basic on-chain agent support

**Q3 2026**
- ZK prompt privacy layer
- Mobile-first wallet onboarding
- Multi-model routing + fallback

**Q4 2026**
- Smart agent SDK for developers
- Emerging market push

### Contributing
We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) (coming soon).

The core canister is the heart of the project — improvements there have the biggest impact.

### Community
- X: [@poro_app](https://x.com/poro_app)
- GitHub: [cybrowl/poro](https://github.com/cybrowl/poro)

---

**Built with ❤️ on the Internet Computer**
