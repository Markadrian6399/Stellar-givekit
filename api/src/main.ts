import cors from 'cors';
import express from 'express';

const app = express();
const port = process.env.PORT || 4000;

app.use(cors());
app.use(express.json());

app.get('/api/v1/health', (_req, res) => {
  res.json({
    name: 'stellar-givekit-api',
    status: 'ok',
    timestamp: new Date().toISOString()
  });
});

app.get('/api/v1/campaigns', (_req, res) => {
  res.json({
    items: [
      {
        id: 'camp_demo_001',
        title: 'Books for 100 Students',
        status: 'draft',
        targetAmount: '5000',
        raisedAmount: '0'
      }
    ]
  });
});

app.post('/api/v1/campaigns', (req, res) => {
  res.status(201).json({
    id: 'camp_demo_001',
    message: 'Campaign created successfully.',
    data: req.body
  });
});

app.post('/api/v1/donations/verify', (req, res) => {
  res.json({
    message: 'Donation verification stub completed.',
    verified: true,
    data: req.body
  });
});

app.get('/api/v1/analytics/overview', (_req, res) => {
  res.json({
    totalCampaigns: 1,
    totalDonations: 0,
    totalRaised: '0',
    activeDonors: 0
  });
});

app.listen(port, () => {
  console.log(`Stellar GiveKit API running on port ${port}`);
});
