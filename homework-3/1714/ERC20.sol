// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.21;

contract ERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;

    uint public totalSupply;

    mapping (address => uint) public balances;
    mapping (address => mapping(address => uint)) public allowances;

    event Transfer(address from, address to, uint value);
    event Approve(address to, uint value);

    constructor(string memory _name, string memory _symbol, uint8 _decimals, uint _totalSupply) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;

        totalSupply = _totalSupply;

        balances[msg.sender] = totalSupply;
    }

    function transfer(address _to, uint _value) public {
        require(balances[msg.sender] >= _value, "Balance too low");

        balances[msg.sender] -= _value;
        balances[_to] += _value;

        emit Transfer(msg.sender, _to, _value);
    }

    function approve(address _to, uint _value) public {
        allowances[msg.sender][_to] = _value;

        emit Approve(_to, _value);
    }

    function transferFrom(address _from, address _to, uint _value) public {
        require(allowances[_from][msg.sender] >= _value, "no permission");

        allowances[_from][msg.sender] -= _value;
        
        balances[_from] -= _value;
        balances[_to] += _value;

        emit Transfer(_from, _to, _value);
    }
}
