# Architecture Overview

## High-level idea

Stellar GiveKit is designed as a modular monorepo where each layer can evolve independently:

1. **Soroban contracts** manage trusted on-chain donation state.
2. **API layer** handles campaign management, transaction verification, aggregation, and access control.
3. **Web app** provides donation flows, dashboards, and wallet interactions.
4. **Insights layer** consumes API and on-chain events for reporting.

## Main components

### 1. Contracts
The contracts folder contains the business logic that should be verifiable and deterministic:
- campaign registration
- donation recording
- reward qualification markers

### 2. API
The API acts as an application service layer:
- validates requests
- stores off-chain campaign metadata
- verifies transaction hashes against Stellar/Soroban sources
- aggregates campaign and donor analytics
- exposes REST endpoints to frontend clients

### 3. Web
The web app focuses on usability:
- browse campaigns
- view campaign pages
- connect wallet
- donate
- see donor history and creator insights

### 4. Insights
The insights layer processes events such as:
- campaign created
- campaign published
- donation submitted
- donation verified
- reward unlocked

## Data split

### On-chain data
- campaign identifiers
- target amount reference
- donation amount references
- reward or badge eligibility markers

### Off-chain data
- campaign title, story, media
- user profile info
- analytics rollups
- moderation flags
- event snapshots for dashboards

## Recommended flow

1. Creator creates campaign through API.
2. API stores metadata in database.
3. Creator publishes campaign to on-chain contract.
4. Donor initiates donation from wallet.
5. API verifies transaction hash and links it to campaign and donor.
6. Insights processor updates rollups and dashboard metrics.
7. Rewards service evaluates whether donor unlocked a badge or quest.

## Future extensions

- recurring donations
- milestone-based fund release
- donor attestations
- on-chain campaign voting
- federation and SEP integrations
