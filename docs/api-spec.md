# API Specification

## Base URL
`/api/v1`

## Health
### GET /health
Returns service health status.

---

## Campaigns
### POST /campaigns
Create a campaign.

**Request body**
```json
{
  "title": "Books for 100 students",
  "summary": "Help fund textbooks and stationery.",
  "targetAmount": "5000",
  "assetCode": "USDC",
  "ownerWalletAddress": "G..."
}
```

### GET /campaigns
List campaigns with pagination.

### GET /campaigns/:id
Get campaign details.

### PATCH /campaigns/:id
Update draft campaign details.

### POST /campaigns/:id/publish
Publish campaign and attach on-chain identifiers.

### POST /campaigns/:id/close
Close campaign.

---

## Donations
### POST /donations
Create a pending donation record.

### POST /donations/verify
Verify a Stellar/Soroban transaction and mark donation as completed.

**Request body**
```json
{
  "campaignId": "uuid",
  "transactionHash": "abc123",
  "donorWalletAddress": "G..."
}
```

### GET /donations/:id
Get donation details.

### GET /campaigns/:id/donations
List campaign donations.

### GET /donors/:walletAddress/donations
List donor donation history.

---

## Rewards
### POST /rewards/evaluate
Evaluate donor rewards for a donation or total contribution history.

### GET /donors/:walletAddress/rewards
List donor badges and reward states.

---

## Analytics
### GET /analytics/overview
Returns global donation metrics.

### GET /analytics/campaigns/:id
Returns metrics for a single campaign.

### GET /analytics/donors/:walletAddress
Returns donor statistics.
