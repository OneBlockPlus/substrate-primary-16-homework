
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

abstract contract Context{
    function _msgSender() internal view virtual returns(address) {
        return msg.sender;
    }
}

library SafeMath {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        require(c >= a, "SafeMath: addition overflow");
        return c;
    }

    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return sub(a, b, "SafeMath: subtraction overflow");
    }

    function sub(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b <= a, errorMessage);
        uint256 c = a - b;
        return c;
    }

    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        if (a == 0) {
            return 0;
        }
        uint256 c = a * b;
        require(c / a == b, "SafeMath: multiplication overflow");
        return c;
    }

    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        return div(a, b, "SafeMath: division by zero");
    }

    function div(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b > 0, errorMessage);
        uint256 c = a / b;
        return c;
    }

}

contract Ownable is Context {
    address private _owner;
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor() {
        address msgSender = _msgSender();
        _owner = msgSender;
    }
    function owner() public view returns (address) {
        return _owner;
    }

    modifier onlyOwner() {
        require(msg.sender == _msgSender(), "Ownable call:must is owner");
        _;
    }

    function renounceOwnership()public virtual onlyOwner{
        emit OwnershipTransferred(_owner, address(0));
        _owner = address(0);
    }
}

interface IERC20 {
    function totleSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 value) external returns (bool);
    function allowance(address owner, address spender) external view returns(uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 value) external returns (bool);
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed from, address indexed to, uint256 value);
}


contract ALEX is Context, IERC20, Ownable{
    using SafeMath for uint256;

    string public name;
    string public symple;
    uint8 public decimals;
    uint256 public totle;

    mapping (address => uint256) public balances;
    mapping (address => mapping (address => uint256)) _allowances;

    constructor() {
        name = unicode"ALEX COIN";
        symple = "ALEX";
        decimals = 9;
        totle = 1000000000 * 10**decimals;
        balances[owner()] += totle;

    }
    

    function totleSupply() public view  override returns (uint256) {
        return totle;
    }

    function balanceOf(address account) public view  override returns (uint256){
        return balances[account];
    }

    function transfer(address recipient, uint256 value) public override returns (bool){
        _transfer(_msgSender(), recipient, value);
        return true;
    }

    function _transfer(address from, address to, uint256 value) private { 
        require(from != address(0), "transfer from the zero address");
        require(to != address(0), "transfer to the zero address");
        require(balances[from] > value, "Transfer amount must be greater than zero");
        require(value > 0, "Transfer amount must be greater than zero");
        balances[from] -= value;
        balances[to] += value;
        emit Transfer(from, to, value);
    }

    function allowance(address owner, address spender) public view override returns (uint256) {
        return _allowances[owner][spender];
    }

    function approve(address spender, uint256 value) public virtual override returns (bool){
        _allowances[msg.sender][spender] = value;
        emit Approval(_msgSender(), spender, value);
        return true;
    }

    function _approve(address owner, address spender, uint256 value) private { 
        require(owner != address(0), "approve from the zero address");
        require(spender != address(0), "approve to the zero address");
        _allowances[owner][spender] = value;
        emit Approval(owner, spender, value);
    }

    function transferFrom(address sender, address recipient, uint256 value) public override returns (bool) {
        _transfer(sender, recipient, value);
        _approve(sender, _msgSender(), _allowances[sender][_msgSender()].sub(value, "transfer value exceeds allowance"));
        return true;
    }
    
     

}