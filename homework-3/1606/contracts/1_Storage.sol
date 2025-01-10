// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

contract ERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    mapping(address=>uint256) public balances;
    mapping (address=>mapping(address=>uint256)) allownaces;

    event Transfer(address from, address to, uint256 value);
    event Approval(address from, address to, uint256 value);

    constructor(string memory _name, string memory _symbol, uint8 _decimals, uint256 _totalSupply){
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        totalSupply = _totalSupply;


        balances[msg.sender] = totalSupply;
        emit Transfer(address(0), msg.sender, totalSupply);

    }

    function transfer(address _to, uint _value) public {
        require(_to != address(0), "wrong recevier");
        require(balances[msg.sender] >= _value, "balance too low");
        balances[msg.sender] -= _value;
        balances[_to] += _value;

        emit Transfer(msg.sender, _to, _value);
    }

    function approve(address _to, uint _value) public  {
        require(_to !=  address(0), "wrong receiver");
        allownaces[msg.sender][_to] = _value;
        emit Approval(msg.sender, _to, _value);
    }

    function transferFrom(address _from, address _to, uint256 _value) public {
        require(allownaces[_from][msg.sender] >= _value, "no access");
        require(balances[_from] >=_value, "balance too low" );

        balances[_from]  -= _value;
        balances[_to] += _value;

        emit  Transfer(_from, _to, _value);
    }

}
