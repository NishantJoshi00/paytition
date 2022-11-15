## On-chain program

## Data models
1. Users
2. Petitions
> Mappings
> User 1-to-many Petitions

```
Petition {
  topic: String <-> 50 characters only
  link: String <-> 50 characters only
  author: Pubkey,
  votes: u64,
  goal: u64,
  timestamp: i64 <- Clock::get().unwrap().unix_timestamp
  vote_lamports: u32,
}
```

