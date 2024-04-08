// SPDX-License-Identifier: UNLICENSED

import "https://raw.githubusercontent.com/NomicFoundation/hardhat/main/packages/hardhat-core/console.sol";

pragma solidity ^0.8.10;

contract Votecontract {

    struct Record {
        string name;
        string selected;
        uint64 votes;
        uint256 time;
    }

    mapping(string => Record) public registry;

    struct Poll {
        uint64 yes;
        uint64 no;
        uint64 total;
    }

    mapping(uint256 => Poll) public polls;

    function recordVote (string memory user, uint64 votes, string memory selected) public {
        require(votes > 0, "Cannot Vote");
        require(registry[user].time == 0, "Cannot Vote");
        require(votes < 5, "Cannot Vote");
        uint256 time = block.timestamp;
        registry[user].name = user;
        registry[user].selected = selected;
        registry[user].votes = votes;
        registry[user].time = time;
        if (keccak256(abi.encodePacked(selected)) == keccak256(abi.encodePacked("yes"))) {
            polls[0].yes = polls[0].yes + votes;
        }
        else {
            polls[0].no = polls[0].no + votes;
        }
            polls[0].total = polls[0].total + votes;
            console.log("Votes Registered!");
    }

}
