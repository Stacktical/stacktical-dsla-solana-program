#!/bin/bash

mkdir -p .anchor/test-ledger

solana-test-validator -r --ledger .anchor/test-ledger --mint 6wb1NhACotpFH7geUTs2SBW7GTA8Zj6ANwB1TDc4JS6y --bind-address 0.0.0.0 --url https://api.devnet.solana.com --rpc-port 8899  --clone 2TfB33aLaneQb5TNVwyDz3jSZXS6jdW2ARw1Dgf84XCG `# programId` \
--clone J4CArpsbrZqu1axqQ4AnrqREs3jwoyA1M5LMiQQmAzB9 `# programDataAddress` \
--clone CKwZcshn4XDvhaWVH9EXnk3iu19t6t5xP2Sy2pD6TRDp `# idlAddress` \
--clone BYM81n8HvTJuqZU1PmTVcwZ9G8uoji7FKM6EaPkwphPt `# programState` \
--clone FVLfR6C2ckZhbSwBzZY4CX7YBcddUSge5BNeGQv5eKhy `# switchboardVault` \
--clone So11111111111111111111111111111111111111112 `# switchboardMint` \
--clone AKwZmswPBosB7YwgJPHTktLpG28nvN3UfL4CnMNhsBAX `# tokenWallet` \
--clone 8zkZYWDWGwFzV64FF3HvK1LhtzYaMWgNBNs5iM5p7FWN `# queue` \
--clone 6wb1NhACotpFH7geUTs2SBW7GTA8Zj6ANwB1TDc4JS6y `# queueAuthority` \
--clone HZQvBoB3AqtcqpnzGHpD8MPUvm8Ar2yXQPmsbEHMDb2Y `# queueBuffer` \
--clone G6QEcdU82KGeV4u5WvQ6ofHAb8cRHx4iZSyjE9jAS1zt `# crank` \
--clone 2C8U9rYoGSj8g963J6wjaMfYAMUpYZ5Wiwiky3WHbKCj `# crankBuffer` \
--clone 9d5rbrY9H4zAknei1kywnfhoSN3nPutee47Zjb9Kmiuw `# oracle` \
--clone 6wb1NhACotpFH7geUTs2SBW7GTA8Zj6ANwB1TDc4JS6y `# oracleAuthority` \
--clone 9TGAKXUaCqoxJvyji538ascGZSdoiqBhryeL7Agmw9Y `# oracleEscrow` \
--clone 4V21phQ9rmB6CBqpoKUdhZDiVKxUNsyvsKrkFWYZAer4 `# oraclePermissions` 