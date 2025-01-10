// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./ERC20.sol";

contract OneBlockCourse{

    mapping (address => course[]) internal courseBoughtMapping;
    
    // 课程结构
    struct course{
        string courseName;
        uint coursePrice;
    }

    mapping (uint => course) public courseMap;
    
    address public adminAddress;
    ERC20 public token;

    constructor () {
        adminAddress = msg.sender;
    }

    modifier onlyAdmin{
        require(msg.sender == adminAddress, "only admin can do this");
        _;
    }

    // 添加课程
    function addCourse(uint _courseId, string memory _courseName, uint _price) public onlyAdmin {
        courseMap[_courseId] = course(_courseName , _price);
    }

    // 查询已购买的课程
    function getBoughtCourse() view public returns(course [] memory) {
        return courseBoughtMapping[msg.sender];
    }

    error NoSuchCourse();
    event CoursePurchased(address buyer, uint courseId, string courseName, uint price);

    // 购买课程
    function buyCourse(uint _courseId) public {
        // 找出该课程信息
        course storage selectedCourse = courseMap[_courseId];
        // 结构体中string的判空！
        if (bytes(selectedCourse.courseName).length ==0){revert NoSuchCourse();}
        // 购买
        purchase(selectedCourse);
        emit CoursePurchased(msg.sender,  _courseId,selectedCourse.courseName, selectedCourse.coursePrice);
        
    }
    
    // 使用token购买 而非eth
    function purchase(course memory selectedCourse) internal{
        // 不能使用transfer 传递的是当前合约的地址 不是用户地址
        token.transferFrom(msg.sender , adminAddress, selectedCourse.coursePrice);
        courseBoughtMapping[msg.sender].push(selectedCourse);
    }



}