// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "openzeppelin-solidity/contracts/token/ERC20/ERC20.sol";
import "openzeppelin-solidity/contracts/access/Ownable.sol";

contract Example is ERC20, Ownable {
    struct ForumInfo {
        string description;
        uint deadline;
        address[] voterAccounts;
        bool[] votes;
    }

    mapping(address => bytes32) public memberDIDs;
    mapping(string => ForumInfo) forums;

    modifier onlyMember() {
        require(memberDIDs[msg.sender] != bytes32(0), "only members");
        _;
    }

    modifier onlyHolder() {
        require(balanceOf(msg.sender) > 0, "only holders");
        _;
    }

    modifier onlyHolderOrMember() {
        require(memberDIDs[msg.sender] != bytes32(0) || balanceOf(msg.sender) > 0, "only member or holder");
        _;
    }

    constructor(
        string memory name,
        string memory symbol,
        address[] memory tokenHolders,
        uint32[] memory holdAmounts,
        address[] memory members,
        bytes32[] memory _memberDIDs
    )
        ERC20(name, symbol)
    {
        require(tokenHolders.length > 0, "no token-holders");
        require(tokenHolders.length == holdAmounts.length, "token-holders dismatch with the amounts");
        require(members.length > 0, "no members");
        require(members.length == _memberDIDs.length, "members dismatch with the dids");

        for (uint i = 0; i < members.length; i++) {
            address member = members[i];
            bytes32 did = _memberDIDs[i];

            require(member != address(0), "members is 0");
            require(did != bytes32(0), "dids is 0");
            require(memberDIDs[member] == bytes32(0), "members duplicate");

            for (uint j = 0; j < i; j++) {
                require(_memberDIDs[j] != did, "dids duplicate");
            }

            memberDIDs[member] = did;
        }

        for (uint i = 0; i < tokenHolders.length; i++) {
            address holder = tokenHolders[i];

            require(holder != address(0), "holders is 0");

            _mint(holder, holdAmounts[i]);
        }
    }

    function createForum(
        string memory name,
        string memory description,
        uint32 _seconds
    )
        public
        onlyMember
    {
        require(forums[name].deadline == 0, "the name used");
        require(_seconds > 0, "seconds is 0");

        forums[name] = ForumInfo({
            description: description,
            deadline: block.timestamp + uint(_seconds),
            voterAccounts: new address[](0),
            votes: new bool[](0)
        });
    }

    function vote(
        string memory forumName,
        bool isAgree
    )
        public
        onlyHolder
    {
        ForumInfo storage forum = forums[forumName];

        require(forum.deadline > 0, "forum not exist");

        uint32 voterIndex = 0;
        uint32 nextVoteSeq = uint32(forum.voterAccounts.length);
        for (; voterIndex < nextVoteSeq; voterIndex++) {
            if (forum.voterAccounts[voterIndex] == msg.sender) {
                break;
            }
        }

        require(voterIndex == nextVoteSeq, "already vote");

        forum.voterAccounts.push(msg.sender);
        forum.votes.push(isAgree);
    }

    function getForumState(
        string memory forumName
    )
        public
        view
        onlyHolderOrMember
        returns (bool isClosed, address[] memory pros, address[] memory dissenters)
    {
        ForumInfo storage forum = forums[forumName];

        require(forum.deadline > 0, "forum not exist");
        
        isClosed = block.timestamp < forum.deadline;

        address[] memory _pros = new address[](forum.voterAccounts.length);
        uint _prosCount = 0;
        address[] memory _dissenters = new address[](forum.voterAccounts.length);
        uint _dissentersCount = 0;

        for (uint32 i = 0; i < forum.voterAccounts.length; i++) {
            if (forum.votes[i]) {
                _pros[_prosCount++] = forum.voterAccounts[i];
            } else {
                _dissenters[_dissentersCount++] = forum.voterAccounts[i];
            }
        }

        pros = new address[](_prosCount);
        for (uint32 i = 0; i < _prosCount; i++) {
            pros[i] = _pros[i];
        }

        dissenters = new address[](_dissentersCount);
        for (uint32 i = 0; i < _dissentersCount; i++) {
            dissenters[i] = _dissenters[i];
        }
    }
}