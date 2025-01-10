### 合约编译截图
![image](https://github.com/user-attachments/assets/10d36945-4c19-4a3a-a331-9735890ca1ce)

### 合约部署记录
```
[root@Alan-PC dark-token]# make deploy
forge create src/DRKToken.sol:DRKToken --rpc-url http://127.0.0.1:8545 --private-key ${PRIVATE_KEY} --constructor-args 1000000
[⠊] Compiling...
No files changed, compilation skipped
Deployer: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
Deployed to: 0x5FbDB2315678afecb367f032d93F642f64180aa3
Transaction hash: 0xeeff3230fe33516a8bce971bfe41ae7f9def6a9c72d6d7c9dd155d2d93f2ab1a
```
### minttoken
```bash
[root@Alan-PC dark-token]# cast send --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 0x5FbDB2315678afecb367f032d93F642f64180aa3 "mint(address,uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000000000000000000
blockHash               0x08d902a3ffc4f7154f561e6085ad47a391f8ff3b6ee95fee0a5b43bc752d4115
blockNumber             2
contractAddress         
cumulativeGasUsed       34233
effectiveGasPrice       879794351
from                    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed                 34233
logs                    [{"address":"0x5fbdb2315678afecb367f032d93f642f64180aa3","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x0000000000000000000000000000000000000000000000000000000000000000","0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"],"data":"0x0000000000000000000000000000000000000000000000056bc75e2d63100000","blockHash":"0x08d902a3ffc4f7154f561e6085ad47a391f8ff3b6ee95fee0a5b43bc752d4115","blockNumber":"0x2","blockTimestamp":"0x676049be","transactionHash":"0x88aac960eec04221b2795200cd97cd407d1da10822d41e061f1a80787aa1b1a9","transactionIndex":"0x0","logIndex":"0x0","removed":false}]
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000040020000000000000100000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000042000000200000000000000000000000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000
root                    0x67abbc837677fcd07d20b8477944181df9ecc2409300df07b8b273db49a6b6b4
status                  1 (success)
transactionHash         0x88aac960eec04221b2795200cd97cd407d1da10822d41e061f1a80787aa1b1a9
transactionIndex        0
type                    2
blobGasPrice            1
blobGasUsed             
authorizationList       
to                      0x5FbDB2315678afecb367f032d93F642f64180aa3
[root@Alan-PC dark-token]# cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "balanceOf(address)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
0x0000000000000000000000000000000000000000000000056bc75e2d631f4240

[root@Alan-PC dark-token]# cast send --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 0x5FbDB2315678afecb367f032d93F642f64180aa3 "mint(address,uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000000000000000000

blockHash               0xfa942b941f9bdc8742d4649bf96ff0b39327bd9bf1f473cd4f9bbe1705043cce
blockNumber             3
contractAddress         
cumulativeGasUsed       34233
effectiveGasPrice       770071041
from                    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed                 34233
logs                    [{"address":"0x5fbdb2315678afecb367f032d93f642f64180aa3","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x0000000000000000000000000000000000000000000000000000000000000000","0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"],"data":"0x0000000000000000000000000000000000000000000000056bc75e2d63100000","blockHash":"0xfa942b941f9bdc8742d4649bf96ff0b39327bd9bf1f473cd4f9bbe1705043cce","blockNumber":"0x3","blockTimestamp":"0x67604a0a","transactionHash":"0x92f6b9768714150de54d56794400eb2d38ab528a0253971f59ada98a314021b1","transactionIndex":"0x0","logIndex":"0x0","removed":false}]
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000040020000000000000100000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000042000000200000000000000000000000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000
root                    0x23717520bfa7f8ca47fddf6fbd76a623835fd44e709572e30e7d13ac9f0a5a31
status                  1 (success)
transactionHash         0x92f6b9768714150de54d56794400eb2d38ab528a0253971f59ada98a314021b1
transactionIndex        0
type                    2
blobGasPrice            1
blobGasUsed             
authorizationList       
to                      0x5FbDB2315678afecb367f032d93F642f64180aa3
[root@Alan-PC dark-token]# cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url http://127.0.0.1:8545
200000000000001000000 [2e20]
[root@Alan-PC dark-token]# cast send --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 0x5FbDB2315678afecb367f032d93F642f64180aa3 "mint(address,uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000000000000000000

blockHash               0x2ed2864f9971f7b22f7e30bade8a069a26168abb36ad6eb07b212184c9f5aed9
blockNumber             4
contractAddress         
cumulativeGasUsed       34233
effectiveGasPrice       674031844
from                    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed                 34233
logs                    [{"address":"0x5fbdb2315678afecb367f032d93f642f64180aa3","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x0000000000000000000000000000000000000000000000000000000000000000","0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"],"data":"0x0000000000000000000000000000000000000000000000056bc75e2d63100000","blockHash":"0x2ed2864f9971f7b22f7e30bade8a069a26168abb36ad6eb07b212184c9f5aed9","blockNumber":"0x4","blockTimestamp":"0x67604ab6","transactionHash":"0xa6b0cc5402af360bcd9b1a4238b1ad235822bb8e1b9e5bac2bc0936749415c10","transactionIndex":"0x0","logIndex":"0x0","removed":false}]
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000040020000000000000100000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000042000000200000000000000000000000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000
root                    0xb4e6107520f128523a347391ee32991da67764fdbe37f3439f21c2bf3c6136dd
status                  1 (success)
transactionHash         0xa6b0cc5402af360bcd9b1a4238b1ad235822bb8e1b9e5bac2bc0936749415c10
transactionIndex        0
type                    2
blobGasPrice            1
blobGasUsed             
authorizationList       
to                      0x5FbDB2315678afecb367f032d93F642f64180aa3
[root@Alan-PC dark-token]# cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url http://127.0.0.1:8545
300000000000001000000 [3e20]
```

### 完整日志
```bash
[root@Alan-PC oneblockplus]# forge init dark-token
Initializing /root/web3/solidity-solution/oneblockplus/dark-token...
Installing forge-std in /root/web3/solidity-solution/oneblockplus/dark-token/lib/forge-std (url: Some("https://github.com/foundry-rs/forge-std"), tag: None)
Cloning into '/root/web3/solidity-solution/oneblockplus/dark-token/lib/forge-std'...
remote: Enumerating objects: 1957, done.
remote: Counting objects: 100% (1952/1952), done.
remote: Compressing objects: 100% (734/734), done.
remote: Total 1957 (delta 1311), reused 1763 (delta 1171), pack-reused 5 (from 1)
Receiving objects: 100% (1957/1957), 618.53 KiB | 370.00 KiB/s, done.
Resolving deltas: 100% (1311/1311), done.
    Installed forge-std v1.9.4
    Initialized forge project
[root@Alan-PC oneblockplus]# cd dark-token
[root@Alan-PC dark-token]# forge install OpenZeppelin/openzeppelin-contracts
Installing openzeppelin-contracts in /root/web3/solidity-solution/oneblockplus/dark-token/lib/openzeppelin-contracts (url: Some("https://github.com/OpenZeppelin/openzeppelin-contracts"), tag: None)
Cloning into '/root/web3/solidity-solution/oneblockplus/dark-token/lib/openzeppelin-contracts'...
remote: Enumerating objects: 48817, done.
remote: Counting objects: 100% (1545/1545), done.
remote: Compressing objects: 100% (982/982), done.
remote: Total 48817 (delta 874), reused 1000 (delta 540), pack-reused 47272 (from 1)
Receiving objects: 100% (48817/48817), 46.33 MiB | 4.24 MiB/s, done.
Resolving deltas: 100% (30467/30467), done.
Submodule 'lib/erc4626-tests' (https://github.com/a16z/erc4626-tests.git) registered for path 'lib/erc4626-tests'
Submodule 'lib/forge-std' (https://github.com/foundry-rs/forge-std) registered for path 'lib/forge-std'
Submodule 'lib/halmos-cheatcodes' (https://github.com/a16z/halmos-cheatcodes) registered for path 'lib/halmos-cheatcodes'
Cloning into '/root/web3/solidity-solution/oneblockplus/dark-token/lib/openzeppelin-contracts/lib/erc4626-tests'...
remote: Enumerating objects: 11, done.        
remote: Counting objects: 100% (11/11), done.        
remote: Compressing objects: 100% (10/10), done.        
remote: Total 11 (delta 3), reused 7 (delta 1), pack-reused 0 (from 0)        
Receiving objects: 100% (11/11), 22.18 KiB | 141.00 KiB/s, done.
Resolving deltas: 100% (3/3), done.
Cloning into '/root/web3/solidity-solution/oneblockplus/dark-token/lib/openzeppelin-contracts/lib/forge-std'...
remote: Enumerating objects: 1957, done.        
remote: Counting objects: 100% (1952/1952), done.        
remote: Compressing objects: 100% (743/743), done.        
remote: Total 1957 (delta 1312), reused 1753 (delta 1162), pack-reused 5 (from 1)        
Receiving objects: 100% (1957/1957), 616.50 KiB | 397.00 KiB/s, done.
Resolving deltas: 100% (1312/1312), done.
Cloning into '/root/web3/solidity-solution/oneblockplus/dark-token/lib/openzeppelin-contracts/lib/halmos-cheatcodes'...
remote: Enumerating objects: 43, done.        
remote: Counting objects: 100% (43/43), done.        
remote: Compressing objects: 100% (34/34), done.        
remote: Total 43 (delta 17), reused 25 (delta 9), pack-reused 0 (from 0)        
Receiving objects: 100% (43/43), 23.36 KiB | 101.00 KiB/s, done.
Resolving deltas: 100% (17/17), done.
Submodule 'lib/ds-test' (https://github.com/dapphub/ds-test) registered for path 'lib/forge-std/lib/ds-test'
Cloning into '/root/web3/solidity-solution/oneblockplus/dark-token/lib/openzeppelin-contracts/lib/forge-std/lib/ds-test'...
remote: Enumerating objects: 313, done.        
remote: Counting objects: 100% (171/171), done.        
remote: Compressing objects: 100% (79/79), done.        
remote: Total 313 (delta 91), reused 132 (delta 83), pack-reused 142 (from 1)        
Receiving objects: 100% (313/313), 71.35 KiB | 142.00 KiB/s, done.
Resolving deltas: 100% (130/130), done.
    Installed openzeppelin-contracts v5.1.0
[root@Alan-PC dark-token]# forge build
[⠊] Compiling...
[⠒] Compiling 33 files with Solc 0.8.26
[⠆] Solc 0.8.26 finished in 1.05s
Compiler run successful!
[root@Alan-PC dark-token]# forge build
[⠊] Compiling...
[⠊] Compiling 2 files with Solc 0.8.26
[⠢] Solc 0.8.26 finished in 906.07ms
Compiler run successful!
[root@Alan-PC dark-token]# forge test
[⠊] Compiling...
No files changed, compilation skipped

Ran 2 tests for test/DRKToken.t.sol:DRKTokenTest
[PASS] testBalance() (gas: 24030)
[PASS] testMint() (gas: 39301)
Suite result: ok. 2 passed; 0 failed; 0 skipped; finished in 8.97ms (2.45ms CPU time)

Ran 2 tests for test/Counter.t.sol:CounterTest
[PASS] testFuzz_SetNumber(uint256) (runs: 256, μ: 31054, ~: 31288)
[PASS] test_Increment() (gas: 31303)
Suite result: ok. 2 passed; 0 failed; 0 skipped; finished in 15.69ms (9.23ms CPU time)

Ran 2 test suites in 24.53ms (24.66ms CPU time): 4 tests passed, 0 failed, 0 skipped (4 total tests)
[root@Alan-PC dark-token]# make build
forge build
[⠊] Compiling...
[⠒] Compiling 3 files with Solc 0.8.26
[⠢] Solc 0.8.26 finished in 964.95ms
Compiler run successful!
[root@Alan-PC dark-token]# make test
forge test --match-path test/DRKToken.t.sol
[⠊] Compiling...
No files changed, compilation skipped

Ran 1 test for test/DRKToken.t.sol:DRKTokenTest
[FAIL. Reason: panic: arithmetic underflow or overflow (0x11)] testMintAndTransfer() (gas: 76375)
Suite result: FAILED. 0 passed; 1 failed; 0 skipped; finished in 898.68µs (244.52µs CPU time)

Ran 1 test suite in 4.01ms (898.68µs CPU time): 0 tests passed, 1 failed, 0 skipped (1 total tests)

Failing tests:
Encountered 1 failing test in test/DRKToken.t.sol:DRKTokenTest
[FAIL. Reason: panic: arithmetic underflow or overflow (0x11)] testMintAndTransfer() (gas: 76375)

Encountered a total of 1 failing tests, 0 tests succeeded
make: *** [makefile:12: test] Error 1
[root@Alan-PC dark-token]# make build
forge build
[⠊] Compiling...
[⠊] Compiling 1 files with Solc 0.8.26
[⠒] Solc 0.8.26 finished in 912.34ms
Compiler run successful!
[root@Alan-PC dark-token]# make deploy
forge create src/DRKToken.sol:DRKToken --rpc-url  --private-key  --constructor-args 1000000
error: a value is required for '--rpc-url <URL>' but none was supplied

For more information, try '--help'.
make: *** [makefile:9: deploy] Error 2
[root@Alan-PC dark-token]# .env
bash: .env: command not found
[root@Alan-PC dark-token]# ll
total 36
-rw-r--r--  1 root root  980 Dec 16 22:46 README.md
drwxr-xr-x  2 root root 4096 Dec 16 23:11 cache
-rw-r--r--  1 root root  170 Dec 16 22:46 foundry.toml
drwxr-xr-x  4 root root 4096 Dec 16 22:48 lib
-rw-r--r--  1 root root  556 Dec 16 23:06 makefile
drwxr-xr-x 38 root root 4096 Dec 16 23:09 out
drwxr-xr-x  2 root root 4096 Dec 16 23:06 script
drwxr-xr-x  2 root root 4096 Dec 16 22:50 src
drwxr-xr-x  2 root root 4096 Dec 16 23:00 test
[root@Alan-PC dark-token]# . .env
[root@Alan-PC dark-token]# make deploy
forge create src/DRKToken.sol:DRKToken --rpc-url  --private-key  --constructor-args 1000000
error: a value is required for '--rpc-url <URL>' but none was supplied

For more information, try '--help'.
make: *** [makefile:9: deploy] Error 2
[root@Alan-PC dark-token]# export $(cat .env | xargs)
[root@Alan-PC dark-token]# make build
forge build
[⠊] Compiling...
No files changed, compilation skipped
[root@Alan-PC dark-token]# make deploy
forge create src/DRKToken.sol:DRKToken --rpc-url http://127.0.0.1:8545 --private-key ${PRIVATE_KEY} --constructor-args 1000000
[⠊] Compiling...
No files changed, compilation skipped
Deployer: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
Deployed to: 0x5FbDB2315678afecb367f032d93F642f64180aa3
Transaction hash: 0xeeff3230fe33516a8bce971bfe41ae7f9def6a9c72d6d7c9dd155d2d93f2ab1a
[root@Alan-PC dark-token]# source .env
[root@Alan-PC dark-token]# export $(cat .env | xargs)
[root@Alan-PC dark-token]# cast wallet import your-account-name --private-key ${PRIVATE_KEY} --interactive
Enter private key:
Enter password: 
`your-account-name` keystore was saved successfully. Address: 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
[root@Alan-PC dark-token]# cast wallet list
your-account-name (Local)
[root@Alan-PC dark-token]# cast wallet list
your-account-name (Local)
[root@Alan-PC dark-token]# cast send --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 0x5FbDB2315678afecb367f032d93F642f64180aa3 "mint(address,uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000000000000000000
blockHash               0x08d902a3ffc4f7154f561e6085ad47a391f8ff3b6ee95fee0a5b43bc752d4115
blockNumber             2
contractAddress         
cumulativeGasUsed       34233
effectiveGasPrice       879794351
from                    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed                 34233
logs                    [{"address":"0x5fbdb2315678afecb367f032d93f642f64180aa3","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x0000000000000000000000000000000000000000000000000000000000000000","0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"],"data":"0x0000000000000000000000000000000000000000000000056bc75e2d63100000","blockHash":"0x08d902a3ffc4f7154f561e6085ad47a391f8ff3b6ee95fee0a5b43bc752d4115","blockNumber":"0x2","blockTimestamp":"0x676049be","transactionHash":"0x88aac960eec04221b2795200cd97cd407d1da10822d41e061f1a80787aa1b1a9","transactionIndex":"0x0","logIndex":"0x0","removed":false}]
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000040020000000000000100000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000042000000200000000000000000000000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000
root                    0x67abbc837677fcd07d20b8477944181df9ecc2409300df07b8b273db49a6b6b4
status                  1 (success)
transactionHash         0x88aac960eec04221b2795200cd97cd407d1da10822d41e061f1a80787aa1b1a9
transactionIndex        0
type                    2
blobGasPrice            1
blobGasUsed             
authorizationList       
to                      0x5FbDB2315678afecb367f032d93F642f64180aa3
[root@Alan-PC dark-token]# cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "balanceOf(address)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
0x0000000000000000000000000000000000000000000000056bc75e2d631f4240

[root@Alan-PC dark-token]# cast send --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 0x5FbDB2315678afecb367f032d93F642f64180aa3 "mint(address,uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000000000000000000

blockHash               0xfa942b941f9bdc8742d4649bf96ff0b39327bd9bf1f473cd4f9bbe1705043cce
blockNumber             3
contractAddress         
cumulativeGasUsed       34233
effectiveGasPrice       770071041
from                    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed                 34233
logs                    [{"address":"0x5fbdb2315678afecb367f032d93f642f64180aa3","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x0000000000000000000000000000000000000000000000000000000000000000","0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"],"data":"0x0000000000000000000000000000000000000000000000056bc75e2d63100000","blockHash":"0xfa942b941f9bdc8742d4649bf96ff0b39327bd9bf1f473cd4f9bbe1705043cce","blockNumber":"0x3","blockTimestamp":"0x67604a0a","transactionHash":"0x92f6b9768714150de54d56794400eb2d38ab528a0253971f59ada98a314021b1","transactionIndex":"0x0","logIndex":"0x0","removed":false}]
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000040020000000000000100000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000042000000200000000000000000000000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000
root                    0x23717520bfa7f8ca47fddf6fbd76a623835fd44e709572e30e7d13ac9f0a5a31
status                  1 (success)
transactionHash         0x92f6b9768714150de54d56794400eb2d38ab528a0253971f59ada98a314021b1
transactionIndex        0
type                    2
blobGasPrice            1
blobGasUsed             
authorizationList       
to                      0x5FbDB2315678afecb367f032d93F642f64180aa3
[root@Alan-PC dark-token]# cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url http://127.0.0.1:8545
200000000000001000000 [2e20]
[root@Alan-PC dark-token]# cast send --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 0x5FbDB2315678afecb367f032d93F642f64180aa3 "mint(address,uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000000000000000000

blockHash               0x2ed2864f9971f7b22f7e30bade8a069a26168abb36ad6eb07b212184c9f5aed9
blockNumber             4
contractAddress         
cumulativeGasUsed       34233
effectiveGasPrice       674031844
from                    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed                 34233
logs                    [{"address":"0x5fbdb2315678afecb367f032d93f642f64180aa3","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x0000000000000000000000000000000000000000000000000000000000000000","0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"],"data":"0x0000000000000000000000000000000000000000000000056bc75e2d63100000","blockHash":"0x2ed2864f9971f7b22f7e30bade8a069a26168abb36ad6eb07b212184c9f5aed9","blockNumber":"0x4","blockTimestamp":"0x67604ab6","transactionHash":"0xa6b0cc5402af360bcd9b1a4238b1ad235822bb8e1b9e5bac2bc0936749415c10","transactionIndex":"0x0","logIndex":"0x0","removed":false}]
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000040020000000000000100000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000042000000200000000000000000000000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000
root                    0xb4e6107520f128523a347391ee32991da67764fdbe37f3439f21c2bf3c6136dd
status                  1 (success)
transactionHash         0xa6b0cc5402af360bcd9b1a4238b1ad235822bb8e1b9e5bac2bc0936749415c10
transactionIndex        0
type                    2
blobGasPrice            1
blobGasUsed             
authorizationList       
to                      0x5FbDB2315678afecb367f032d93F642f64180aa3
[root@Alan-PC dark-token]# cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url http://127.0.0.1:8545
300000000000001000000 [3e20]
```
