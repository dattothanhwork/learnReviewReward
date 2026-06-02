# Peer Review Reward — PRD

Project Name: Peer Review Reward

Problem (1 sentence): Students write peer reviews for classmates' assignments but reviewers are rarely rewarded transparently or on time.

Solution (1 sentence): A lightweight Soroban contract that records peer reviews on-chain and issues on-chain "reward points" that can be tracked and claimed by reviewers; an off-chain SQLite DB mirrors reviews for analytics and classroom UIs.

Stellar Feature Used:
- Soroban smart contract (custom on-chain registry + simple balance ledger)

Target User: University instructors and students running peer-assessment assignments.

Core Feature (MVP): Submit a peer review (on-chain) and have an admin distribute reward points to reviewer addresses; reviewers can view and claim balances.

Why Stellar: Low fees + fast transactions make frequent small-value rewards (micro-incentives) practical for classroom workflows.

Success Metrics (for demo):
- Ability to submit reviews and read them on-chain
- Admin can mint / assign reward points
- On-chain review IDs + at least one transaction hash to show deployment

Security Notes:
- Private keys must never be stored in the repo. Use Freighter or test keys when interacting.
- Admin-only actions require `require_auth()`.
