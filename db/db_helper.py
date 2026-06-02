#!/usr/bin/env python3
"""Small helper to create and inspect the local SQLite mirror DB."""
import sqlite3
import time
from pathlib import Path

DB_PATH = Path(__file__).parent / "reviews.db"

def init_db(path=DB_PATH):
    path.parent.mkdir(parents=True, exist_ok=True)
    conn = sqlite3.connect(path)
    with open(Path(__file__).parent / "schema.sql", "r", encoding="utf8") as f:
        conn.executescript(f.read())
    conn.commit()
    conn.close()

def add_review(submission_id: int, reviewer: str, score: int, comment: str, path=DB_PATH):
    ts = int(time.time())
    conn = sqlite3.connect(path)
    cur = conn.cursor()
    cur.execute(
        "INSERT INTO reviews (submission_id, reviewer, score, comment, timestamp, rewarded) VALUES (?,?,?,?,?,?)",
        (submission_id, reviewer, score, comment, ts, 0),
    )
    conn.commit()
    conn.close()

def list_reviews(path=DB_PATH):
    conn = sqlite3.connect(path)
    cur = conn.cursor()
    cur.execute("SELECT id,submission_id,reviewer,score,comment,timestamp,rewarded FROM reviews ORDER BY id DESC")
    rows = cur.fetchall()
    conn.close()
    return rows

def set_balance(reviewer: str, balance: int, path=DB_PATH):
    conn = sqlite3.connect(path)
    cur = conn.cursor()
    cur.execute("INSERT INTO balances (reviewer,balance) VALUES (?,?) ON CONFLICT(reviewer) DO UPDATE SET balance=excluded.balance", (reviewer, balance))
    conn.commit()
    conn.close()

if __name__ == "__main__":
    init_db()
    print("DB initialized at:", DB_PATH)
    # sample insert
    add_review(1, "GABCD...", 90, "Well argued")
    print("Recent reviews:")
    for r in list_reviews():
        print(r)
