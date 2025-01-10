// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Basic ERC20 token implementation
contract ERC20 {
    // Token basic information
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    // Balances and allowances mapping
    mapping (address => uint256) public balance;
    mapping (address => mapping(address => uint256)) public allowances;

    event Transfer(address from, address to, uint256 value);
    event Approval(address from, address to, uint256 value);

    // Initialize token with basic properties
    constructor(string memory _name, string memory _symbol, uint8 _decimals, uint256 _totalSupply) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        
        totalSupply = _totalSupply;
        balance[msg.sender] = totalSupply;
        
        emit Transfer(address(0), msg.sender, totalSupply);
    }

    // Transfer tokens to another address
    function transfer(address _to, uint256 _value) public {
        require(_to != address(0), "invalid receiver");
        require(balance[msg.sender] >= _value, "balance too low");
        
        balance[msg.sender] -= _value;
        balance[_to] += _value;
        
        emit Transfer(msg.sender, _to, _value);
    }
    
    // Approve another address to spend tokens
    function approve(address _to, uint256 _value) public {
        require(_to != address(0), "invalid receiver");
        allowances[msg.sender][_to] = _value;
        
        emit Approval(msg.sender, _to, _value);
    }
    
    // Transfer tokens on behalf of another address
    function transferFrom(address _from, address _to, uint256 _value) public {
        require(allowances[_from][msg.sender] >= _value, "no permission");
        require(balance[_from] >= _value, "balance too low");
        
        allowances[_from][msg.sender] -= _value;
        
        balance[_from] -= _value;
        balance[_to] += _value;
        
        emit Transfer(_from, _to, _value);
    }
}