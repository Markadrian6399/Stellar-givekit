# Stellar GiveKit

**Soroban-powered micro-donation infrastructure for campaigns, community giving, and on-chain analytics.**

Stellar GiveKit is an open-source full-stack starter for building transparent donation products on the Stellar ecosystem. It combines Soroban smart contracts, a backend API, a web dashboard, and an insights layer so developers can launch micro-donation campaigns quickly.

## Why this repo exists

A lot of donation tools stop at payment collection. Stellar GiveKit goes further by supporting:

- campaign creation and milestone tracking
- small-value donations with wallet integration
- transparent donation history
- donor badges and engagement quests
- analytics for campaigns, donors, and transaction activity
- modular architecture for hackathons, MVPs, and production pilots

## Core features

- **Campaign management**: create, update, close, and monitor fundraising campaigns
- **Donation processing**: record and verify donations made through Stellar/Soroban flows
- **Rewards layer**: issue supporter badges or quest completions
- **Insights dashboard**: track volume, active donors, campaign performance, and trends
- **Wallet support**: connect Stellar-compatible wallets from the web app
- **Developer-friendly API**: REST endpoints for campaigns, donations, analytics, and rewards

## Monorepo structure

```text
stellar-givekit/
├── README.md
├── .gitignore
├── docs/
│   ├── architecture.md
│   ├── api-spec.md
│   └── roadmap.md
├── contracts/
│   ├── Cargo.toml
│   ├── campaign-contract/
│   ├── donation-contract/
│   └── reward-contract/
├── api/
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   └── tests/
├── web/
│   ├── package.json
│   ├── app/
│   ├── components/
│   └── lib/
├── insights/
│   ├── dashboards/
│   └── event-processors/
└── .github/
    └── workflows/
```

## Suggested tech stack

### Smart contracts
- Rust
- Soroban SDK

### API
- Node.js
- NestJS
- PostgreSQL
- Prisma or TypeORM
- Redis optional for rate limiting and caching

### Web
- Next.js
- TypeScript
- Tailwind CSS
- Stellar wallet integration

### Insights
- Event processor for chain and API events
- PostgreSQL materialized views or ClickHouse for analytics
- Grafana / Metabase dashboards

## MVP scope

### Smart contracts
- create campaign
- donate to campaign
- track total raised
- close campaign
- issue reward metadata

### API
- campaign CRUD
- donation verification by transaction hash
- donor history
- campaign leaderboard
- analytics summary
- reward lookups

### Web
- landing page
- campaign list
- campaign detail page
- donation flow
- dashboard for creators

## REST API preview

### Campaigns
- `POST /api/v1/campaigns`
- `GET /api/v1/campaigns`
- `GET /api/v1/campaigns/:id`
- `PATCH /api/v1/campaigns/:id`
- `POST /api/v1/campaigns/:id/publish`
- `POST /api/v1/campaigns/:id/close`

### Donations
- `POST /api/v1/donations`
- `POST /api/v1/donations/verify`
- `GET /api/v1/donations/:id`
- `GET /api/v1/campaigns/:id/donations`
- `GET /api/v1/donors/:walletAddress/donations`

### Rewards
- `POST /api/v1/rewards/evaluate`
- `GET /api/v1/donors/:walletAddress/rewards`

### Analytics
- `GET /api/v1/analytics/overview`
- `GET /api/v1/analytics/campaigns/:id`
- `GET /api/v1/analytics/donors/:walletAddress`

## Example use cases

- non-profits running transparent micro-campaigns
- creators collecting community support in small amounts
- hackathon teams building Stellar social-good products
- student developers learning Soroban + full-stack integration
- developer communities needing a reusable donation starter kit

## Getting started

### 1. Clone the repository
```bash
git clone https://github.com/Markadrian6399/stellar-givekit.git
cd stellar-givekit
```

### 2. Install API dependencies
```bash
cd api
npm install
```

### 3. Install web dependencies
```bash
cd ../web
npm install
```

### 4. Build Soroban contracts
```bash
cd ../contracts
cargo build
```

## Environment variables

### API
```env
PORT=4000
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/stellar_givekit
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
JWT_SECRET=change_me
```

### Web
```env
NEXT_PUBLIC_API_BASE_URL=http://localhost:4000/api/v1
NEXT_PUBLIC_STELLAR_NETWORK=testnet
```

## Roadmap

- wallet auth with challenge signing
- campaign milestones and escrow-like release rules
- recurring donations
- donation feed and social sharing
- badge NFTs or attestation-style rewards
- admin moderation tools
- full observability for chain and API events

## Contribution guide

1. Fork the repo
2. Create a branch: `feature/your-change`
3. Commit with clear messages
4. Add tests where needed
5. Open a pull request

## License

MIT
