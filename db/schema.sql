-- SQLite schema for Peer Review Reward off-chain mirror
CREATE TABLE IF NOT EXISTS reviews (
  id INTEGER PRIMARY KEY,
  submission_id INTEGER NOT NULL,
  reviewer TEXT NOT NULL,
  score INTEGER NOT NULL,
  comment TEXT,
  timestamp INTEGER NOT NULL,
  rewarded INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS balances (
  reviewer TEXT PRIMARY KEY,
  balance INTEGER NOT NULL DEFAULT 0
);
