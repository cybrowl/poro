# Poro Business Plan (Realistic Version – March 2026)

## Executive Summary
Poro is a privacy-first AI access platform built on the Internet Computer (ICP). It provides a single unified API for LLMs and image models with **on-chain subscription management, rate limiting, payments in USDC, and user authentication** — all handled in Rust canisters.

The product targets two groups:
- **Unbanked and low-income users** (1.4 billion globally) who cannot use traditional $20/month AI subscriptions.
- **Web3 developers** who want privacy-preserving, on-chain AI integration.

Core offering: **$4.20/month in USDC** for generous access to open-source models (Llama 3.1, Mistral, DeepSeek, etc.) plus limited queries to frontier models. Everything else (payments, rate limits, usage tracking) runs fully on-chain.

**Architecture**: Hybrid — fast off-chain inference via trusted proxies + on-chain control layer on ICP. This is the only practical model that delivers both speed and true decentralization today.

We are in early MVP stage. The core subscription canister is built and deployed. The project is open-source on GitHub.

**Conservative projections**:
- Year 1: 3,000–5,000 paying users → **$80K–$150K ARR**
- Year 2: 15,000–20,000 users → **$400K–$600K ARR**
- Year 3: 40,000+ users → **$1M+ ARR**

Gross margins are high (~85–90%) because inference costs are passed through and ICP cycles are cheap. The business is capital-efficient and has a clear path to profitability.

## Market Opportunity
The broader AI API market is large (~$44B in 2026), but the **decentralized/privacy-first segment** that Poro targets is still emerging — estimated at **$150–250M** in 2026 and growing at ~25% CAGR.

Key drivers:
- Rising demand for AI without credit cards (especially in emerging markets).
- Growing privacy concerns and regulatory pressure.
- Web3 developers needing on-chain AI for agents, DeFi, and DAOs.

Poro’s differentiation:
- Extremely low price ($4.20 vs $20 average).
- True on-chain payments and rate limiting (no centralized database).
- Open-source core canister.
- Borderless USDC access.

## Product & Architecture
**Current MVP**:
- Rust canister handling subscriptions, rate limiting, usage tracking, and Internet Identity auth.
- Simple unified proxy endpoint for inference.
- Basic SvelteKit frontend.

**Hybrid model** (the realistic choice):
- Inference runs off-chain through secure proxies (fast + cost-effective).
- All business logic (payments, limits, auth) stays on-chain on ICP.

Future additions (Q3–Q4 2026):
- On-chain smart agents.
- Basic ZK prompt privacy.
- Mobile-first onboarding for emerging markets.

## Go-to-Market Strategy (Realistic)
**Phase 1 (Months 1–4)**: ICP community launch
- Target existing ICP users and developers.
- Free tier + referral program.
- Goal: 1,000–2,000 users.

**Phase 2 (Months 5–9)**: Emerging market focus
- Marketing via Telegram bots, local crypto communities, and X in Latin America, Africa, and Southeast Asia.
- Partnerships with ICP wallets and USDC on-ramps.
- Goal: Reach 5,000 paying users.

**Phase 3 (Month 10+)**: Developer & enterprise push
- SDK for on-chain agents.
- Paid acquisition in Web3 AI communities.

**Customer Acquisition Cost (CAC)**: Expected $8–15 per user in early stages (mostly organic + community).

## Financial Projections (Conservative)
**Pricing**:
- Free tier: Limited testing.
- Poro Basic: $4.20/month (main product).
- Poro Pro: $9.90/month (higher limits + priority support).

**Revenue & Growth** (realistic):
- End of Year 1: 4,000 paying users → ~$100K ARR
- End of Year 2: 18,000 users → ~$450K ARR
- End of Year 3: 45,000 users → ~$1.1M ARR

**Gross Margin**: 85–90% (inference costs passed through, low ICP cycle costs, ~1% payment fees).

**Break-even**: Expected in late Year 2 with disciplined spending.

**Funding needs**: Bootstrap possible for first 12 months. Seed round of $500K–$750K would accelerate marketing and agent development.

## Risks & Mitigations
- **Slow user adoption**: Focus first on ICP community (warm audience).
- **Inference costs**: Strict rate limits + open-source model priority.
- **Competition**: Differentiate on price, privacy, and on-chain integration.
- **Regulatory**: Minimal data storage + USDC focus keeps compliance light.
- **Technical**: Start simple, add ZK/agents only after product-market fit.

## Roadmap
- **Q2 2026**: Public beta launch + basic unified API.
- **Q3 2026**: Smart agent SDK + mobile wallet improvements.
- **Q4 2026**: First 10,000 users + profitability path clear.
- **2027**: Multi-model routing + enterprise features.

## Conclusion
Poro is a focused, capital-efficient product that solves a real pain point: affordable, private AI access for people and developers who are currently locked out.

The hybrid architecture is realistic, the price point is memorable, and the on-chain foundation gives us a defensible moat. With disciplined execution, this can become a profitable, meaningful business in the decentralized AI space.
