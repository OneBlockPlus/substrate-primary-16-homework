// SPDX-License-Identifier: MIT
pragma solidity >=0.8.2 <0.9.0;

contract BraisedSixERC20 {
    address owner; // 代币发行者
    string public name; // 代币名称
    string public symbol; // 代币符号
    uint256 public totalSuplly; // 代币发行总量
    uint8 public decimals; // 代币最小单位

    mapping(address => uint256) balance; // 代币持有余额
    mapping(address => mapping(address => uint256)) allowed; // 批准代币使用

    event Approval(address indexed _from, address indexed _to, uint256 amount);
    event Transfer(address indexed _from, address indexed _to, uint256 amount);

    // 构造器初始化
    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _totalSuplly,
        uint8 _decimals
    ) {
        name = _name;
        symbol = _symbol;
        totalSuplly = _totalSuplly;
        decimals = _decimals;
        owner = msg.sender;
        // 初始化所有代币给到创建者
        balance[msg.sender] = _totalSuplly;
        emit Transfer(address(0), msg.sender, _totalSuplly);
    }

    // 授权代币使用数量
    function approve(address _spender, uint256 amount) public returns (bool) {
        require(balance[msg.sender] >= amount, "Insufficient balance"); // 余额不足
        require(
            allowed[msg.sender][_spender] + amount <= balance[msg.sender],
            "Authorization amount is too large"
        );
        allowed[msg.sender][_spender] = allowed[msg.sender][_spender] + amount;
        //记录到事件
        emit Approval(msg.sender, _spender, amount);
        return true;
    }

    // 转账
    function transfer(
        address _to,
        uint256 amount
    ) public returns (bool success) {
        require(_to != address(0), "Cannot send to all zero address.");
        require(balance[msg.sender] >= amount, "Insufficient balance"); // 余额不足
        balance[msg.sender] -= amount;
        balance[_to] += amount;
        //记录到事件
        emit Transfer(msg.sender, _to, amount);
        success = true;
    }

    // 从谁的账户转账
    function transferFrom(
        address _from,
        address _to,
        uint256 amount
    ) public returns (bool success) {
        require(_to != address(0), "Cannot send to all zero address.");
        require(
            allowed[_from][msg.sender] >= amount,
            "Insufficient authorization balance"
        ); // 授权余额不足
        require(balance[_from] >= amount, "Insufficient balance");
        balance[_from] -= amount;
        balance[_to] += amount;
        allowed[_from][msg.sender] -= amount;
        //记录到事件
        emit Transfer(_from, _to, amount);
        success = true;
    }

    // 查询授权
    function allowance(
        address _owner,
        address _spender
    ) public view returns (uint256 remaining) {
        remaining = allowed[_owner][_spender];
    }

    // 查询余额
    function balanceOf(address _owner) public view returns (uint256) {
        return balance[_owner];
    }
}
