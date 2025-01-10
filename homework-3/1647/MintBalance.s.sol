// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "../src/DRKToken.sol";

contract MintBalanceScript {
    DRKToken public drkToken;

    // 移除构造函数中的tokenAddress参数
    constructor() {}

    // 添加run函数，接受tokenAddress作为参数
    function run(address tokenAddress) external {
        drkToken = DRKToken(tokenAddress); // 在这里初始化drkToken

        // Mint tokens to two different addresses
        address wallet1 = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
        address wallet2 = address(0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        drkToken.mint(wallet1, 100 * 10 ** 18); // Mint 100 tokens to wallet1
        drkToken.mint(wallet2, 200 * 10 ** 18); // Mint 200 tokens to wallet2
    }
}
