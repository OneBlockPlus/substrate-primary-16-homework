// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ERC20{
    string public name;
    string public symbol;
    uint8 public decimals;

    uint public totalSupply;
    
    mapping (address => uint) public balances;
    mapping (address => mapping (address => uint)) public allowances;

    constructor (string memory _name, string memory _symbol , uint8 _decimals, uint _totalSupply){
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        totalSupply = _totalSupply;
        // 把所有的token给到创建者
        balances[msg.sender] = totalSupply;
    }
    // 在config.json里加入"viaIR": true, 启用via ir pipeline
    error NoEnouphBalance();
    error NoEnouphAllowances(uint allowances);

    modifier CheckBalance(uint _value){
        // 更省gas
        // require(balances[msg.sender] >= _value, NoEnouphBalance());
        if (balances[msg.sender] <= _value){
            revert NoEnouphBalance();
        }
        _;
    }

    modifier CheckAllowance(address _from, uint _value){
        if (allowances[_from][msg.sender] <= _value){
            revert NoEnouphAllowances(allowances[_from][msg.sender]);
        }
        _;
    }

    event Transfer(address _from, address _to, uint _value);

    
    function transfer(address _to, uint _value) public CheckBalance(_value){

        balances[msg.sender] -= _value;
        balances[_to] += _value;

        emit Transfer(msg.sender, _to, _value);

    }

    event Approve(address _owner, address spender, uint _value);

    // 授权其他地址发送部分自己账号的金额
    function approve (address spender, uint _value) public CheckBalance(_value){
        allowances[msg.sender][spender] = _value;
        emit Approve(msg.sender, spender, _value);
    }

    // 从指定账号转钱
    function transferFrom (address _from, address _to, uint _value) public CheckAllowance(_from, _value){
        allowances[_from][msg.sender] -= _value;
        balances[_from] -= _value;
        balances[_to] += _value;

        emit Transfer(_from, _to, _value);
    }




}