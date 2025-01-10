### 编写合约
![img.png](img.png)
### 编写ERC20测试
![img_1.png](img_1.png)
### 部署脚本
![img_2.png](img_2.png)
### 部署与编译
forge build 
### 部署本地网络
![img_3.png](img_3.png)
![img_4.png](img_4.png)
### 部署合约
forge script script/ERC20Script.sol:ERC20Script --fork-url http://localhost:8545 --broadcast
![img_5.png](img_5.png)
![img_6.png](img_6.png)
![img_7.png](img_7.png)