// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IERC20 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract MyToken is IERC20 {
    string public name = "MyToken"; // 代币名称
    string public symbol = "MTK"; // 代币符号
    uint8 public decimals = 18; // 小数位数
    uint256 private _totalSupply; // 总供应量
    mapping(address => uint256) private _balances; // 账户余额
    mapping(address => mapping(address => uint256)) private _allowances; // 授权额度

    constructor(uint256 initialSupply) {
        _totalSupply = initialSupply * (10 ** uint256(decimals)); // 设置初始供应量
        _balances[msg.sender] = _totalSupply; // 将初始供应量分配给合约创建者
        emit Transfer(address(0), msg.sender, _totalSupply); // 触发转账事件
    }

    function totalSupply() public view override returns (uint256) {
        return _totalSupply; // 返回总供应量
    }

    function balanceOf(address account) public view override returns (uint256) {
        return _balances[account]; // 返回指定账户的余额
    }

    function transfer(address recipient, uint256 amount) public override returns (bool) {
        require(recipient != address(0), "ERC20: transfer to the zero address");
        require(_balances[msg.sender] >= amount, "ERC20: transfer amount exceeds balance");

        _balances[msg.sender] -= amount; // 减少发送者余额
        _balances[recipient] += amount; // 增加接收者余额
        emit Transfer(msg.sender, recipient, amount); // 触发转账事件
        return true;
    }

    function approve(address spender, uint256 amount) public override returns (bool) {
        require(spender != address(0), "ERC20: approve to the zero address");

        _allowances[msg.sender][spender] = amount; // 设置授权额度
        emit Approval(msg.sender, spender, amount); // 触发授权事件
        return true;
    }

    function transferFrom(address sender, address recipient, uint256 amount) public override returns (bool) {
        require(sender != address(0), "ERC20: transfer from the zero address");
        require(recipient != address(0), "ERC20: transfer to the zero address");
        require(_balances[sender] >= amount, "ERC20: transfer amount exceeds balance");
        require(_allowances[sender][msg.sender] >= amount, "ERC20: transfer amount exceeds allowance");

        _balances[sender] -= amount; // 减少发送者余额
        _balances[recipient] += amount; // 增加接收者余额
        _allowances[sender][msg.sender] -= amount; // 减少授权额度
        emit Transfer(sender, recipient, amount); // 触发转账事件
        return true;
    }

    function allowance(address owner, address spender) public view override returns (uint256) {
        return _allowances[owner][spender]; // 返回授权额度
    }
}