// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract ERC20{
    string public name ;
    string public symbol;
    uint8 public decimals;
    uint public totalSupply;
    address public author;

    mapping(address=>uint)balances;
    mapping (address=>mapping (address => uint)) allowances;

    event Transfer(address indexed from, address indexed to, uint value);
    event Approval(address indexed owner, address indexed spender, uint value);

    modifier checkTransfer(address _from, address _to, uint _amount){
        require(_to != address(0),"Invalid Address");
        require(balances[msg.sender] >= _amount,"Insufficient Balance");
        _;
    }

    modifier checkAuthor(){
        require(author == msg.sender,"Not Authorized");
        _;
    }

    constructor(string memory _name , string memory _symbol, uint8 _decimals, uint _totalSupply){
        name = _name;
        symbol = _symbol; 
        decimals = _decimals;  
        totalSupply = _totalSupply; 
        author = msg.sender;

        balances[msg.sender] = _totalSupply * 10 ** decimals;
    }
    
    function balanceOf(address account) public view returns (uint){
       return balances[account];
    }
    
    function transfer(address _to, uint _amount) public checkTransfer(msg.sender, _to, _amount){
    
        balances[msg.sender] -= _amount; 
        balances[_to] += _amount;
        
        emit Transfer(msg.sender, _to, _amount);
    }
    
    function approve(address _to, uint _amount) public checkTransfer(msg.sender, _to, _amount) checkAuthor{
        allowances[msg.sender][_to] = _amount;
        emit Approval (msg.sender , _to  , _amount);
    }

    function transferFrom(address _to , uint256 _amount) public checkTransfer(author, msg.sender, _amount){
        require(allowances[author][msg.sender] >= _amount, "No Permission");
        balances[author] -= _amount;
        balances[msg.sender] -= _amount;
        balances[_to] += _amount;

        emit Transfer(author, _to, _amount);
    }
}