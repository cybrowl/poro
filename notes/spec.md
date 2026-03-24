# Poro Feature Specification

**Product**: Privacy-first, borderless AI access platform on the Internet Computer (ICP)  
**Target Users**: Unbanked individuals + Web3 developers  
**Core Promise**: Pay $4.20/month in USDC for private, credit-card-free access to powerful AI models.

## 1. High-Level Architecture

**Hybrid Model** (chosen for speed + decentralization):
- **On-chain layer** (Motoko canister): All trust, money, and control logic
- **Off-chain layer** (Vercel/Cloudflare proxy): Fast inference and streaming
- Communication: Short-lived tokens issued by the canister

**Key Benefits**:
- True on-chain payments and rate limiting
- Fast streaming responses
- Secret API keys never exposed
- Scales to 500 GiB stable memory in one canister

## 2. Core Features

### 2.1 User Management & Authentication
- Internet Identity login
- Persistent user profiles stored in stable memory
- Automatic free-tier creation on first use

### 2.2 Subscription System
- **Free Tier**: 15 requests/day (open-source models only)
- **Basic Tier**: $4.20/month in USDC
  - 300 requests/day
  - Unlimited open-source models
  - Limited proprietary model access
- Subscription status, expiry, and lifetime paid amount tracked on-chain
- Automatic daily reset of usage counters

### 2.3 Payments (ckUSDC)
- One-click subscribe using ICRC-2 `transfer_from`
- Automatic activation on successful payment
- Full payment history stored on-chain (timestamp, amount, tx_id)
- USDC-only (borderless, no credit card required)

### 2.4 Rate Limiting & Gating
- On-chain daily usage tracking
- `get_chat_token()` method returns short-lived token only if user is within limits
- Token validation in the proxy
- Automatic rejection when daily limit reached

### 2.5 AI Access & Proxy
- Unified API endpoint (`/api/chat`)
- Supports multiple models via proxy
- Full streaming responses
- Short-lived token authentication
- Future: model routing, fallbacks, cost tracking

### 2.6 Chat Interface
- Full-screen Markdown editor (Carta)
- Click-anywhere on right panel to focus input
- Submit icon (yellow ↑ button)
- Fixed top toolbar (when signed in)
- Real-time typing with proper Dracula theme
- Placeholder appears below submit row

### 2.7 Wallet & Payment UI Components
- CashOutAmount
- AddressToInput
- CashBalanceContainer
- FundWalletWarning
- ToggleSwitch (deposit/cash-out)
- Transaction rows
- Banner messages
- CoinAddressContainer (ICP + BTC example)
- TransactionHistoryButton

## 3. User Flows

**New User**
1. Lands → clicks anywhere → signs in with Internet Identity
2. Gets free tier automatically
3. Can start chatting immediately (15 queries/day)

**Paid User**
1. Clicks “Subscribe” → approves ckUSDC spend
2. Canister pulls $4.20 → activates subscription
3. Gets 300 queries/day + higher limits

**Developer / API Usage**
1. Calls `get_chat_token()` from canister
2. Uses token in proxy requests
3. All usage tracked on-chain

## 4. Technical Stack

- **Canister**: Motoko (`persistent actor` + `mo:core/Map`)
- **Frontend**: SvelteKit + TypeScript
- **Editor**: Carta Markdown Editor
- **Proxy**: Vercel (Node.js/TypeScript)
- **Payments**: ckUSDC via ICRC-2
- **Auth**: Internet Identity
- **Styling**: Tailwind + custom dark theme

## 5. Scalability & Limits

- Single canister supports up to **500 GiB** stable memory
- Realistic capacity: **50–100+ million users** before sharding
- Rate limiting and usage tracking fully on-chain

## 6. Current Status (March 2026)

**Completed**:
- Motoko user canister with full subscription + rate limiting logic
- Hybrid proxy architecture
- Click-to-focus editor with fixed toolbar
- Wallet component visual guide
- TypeScript migration complete

**Next Priorities**:
- Real ckUSDC ICRC-2 integration in `subscribe()`
- Secure token issuance (JWT-style)
- Multi-model routing in proxy
- Basic on-chain agent support
