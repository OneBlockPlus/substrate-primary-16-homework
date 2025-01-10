# substrate操作步骤

## 

## 1. 终端启动

```bash
./target/release/solochain-template-node --dev             //--dev 开发模式启动  --tmp  临时启动，删除当前启动后的数据                                                              
```

![](./img/单节点运行终端.png)

浏览器出块和转账如图：

![image-20241209205610314](./img/image.png)



## 2. 其它启动模式

- 指定账户单节点启动

  ```
  ./target/release/solochain-template-node  --dev -d /tmp/alice   //转账数据会存储起来，重启后会一直在
  ```

  ![](./img/image.png)

  ```bash
  ./target/release/solochain-template-node purge-chain --dev -d /tmp/alice   //删除本地数据
  ```

  ![](./img/删除节点终端代码.jpg)

  转账记录无:

  ![](./img/转账记录无.jpg)

- 双节点共同出块启动

  记录节点1启动参数节点地址:<font color=red>12D3KooWKTB3Q2GAkCAtChM1pDqoF6wJSQC6XcbKYPCPBDpyw1yQ </font>

  ![](./img/节点1启动identity-id.jpg)

  ```bash
  ./target/release/solochain-template-node --chain=local --alice --tmp --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRJacs1zeQeKWr3TsfWf7mhxyY6p2wcG6E1jVPLEAuSBK
  
  ./target/release/solochain-template-node --chain=local --bob --tmp --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWKN5KUtMfpPY3o1wPYaHVat4GHDTj7rWV4vTdiya1axsC
  ```

  双节点组网浏览器：

  ![](./img/双节点组网.jpg)

  双节点客户端01：

  ![](./img/双节点-01.jpg)

  双节点客户端02：

  ![](./img/双节点02.jpg)

- 多节点客户端组网

  生成区块链初始化信息配置文件

  ```bash
  ./target/release/solochain-template-node key inspect //Charlie --scheme sr25519
  ```

  ![](./img/生成配置信息.jpg)

  生成结果：

  ![](./img/生成配置结果.jpg)

  生成新账户节点信息：

  ```bash
  ./target/release/solochain-template-node key inspect //Charlie --scheme sr25519
  ```

  ![](./img/生成新节点.jpg)

  添加新账户节点：

  ![](./img/添加新账户节点.jpg)

  将公钥加入到grandpa当中去:

  ```bash
  ./target/release/solochain-template-node key inspect //Charlie --scheme ed25519
  ```

  

  build成raw格式文件：

  ```bash
  ./target/release/solochain-template-node build-spec --chain=localSpec.json --disable-default-bootnode --raw > rawLocalSpec.json      
  ```

  最后启动alice、bob以及新账户节点Charlie

  ```bash
  ./target/release/solochain-template-node --chain=rawLocalSpec.json --alice --tmp   //alice启动
  ```

  ![](./img/alice启动.jpg)

  bob启动：

  ```bash
  ./target/release/solochain-template-node --chain=rawLocalSpec.json --bob --tmp --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLAWrivZ6A5DJnHCREVPB6jQZ8U26hiFJ9hFUvZB2twER
  ```

  ![](./img/bob启动.jpg)

  可以看到虽然出块了，但实际最后区块没有进行同步，finalized一直是0，下面加入最后一个节点：

  ```bash
   ./target/release/solochain-template-node --chain=rawLocalSpec.json --bob --tmp --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSmy9VrbyTWJLRN1Rfx5xLSGDrM5stMNMChCAbn8fgKSW  
   
   ./target/release/solochain-template-node --chain=rawLocalSpec.json --charlie --tmp --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSmy9VrbyTWJLRN1Rfx5xLSGDrM5stMNMChCAbn8fgKSW
  ```

  最后结果如下图：

  ![](./img/最终结果.jpg)

  浏览器验证：

  ![](./img/节点运行图.jpg)
