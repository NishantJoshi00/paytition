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
  -- votes: Vec<Pubkey>, <-> (!This approach limits the size of votes to be put forth)
  boxes: u32,
  goal: u64,
  box_size: u32,
  timestamp: i64 <- Clock::get().unwrap().unix_timestamp
}
```
-> To solve for the size we can create ballot boxes
```
BallotBox {
  idx: u8,
  votes: Vec<Pubkey>,
  is_full: bool
}
```

- topic \ link size exceeded
- 

