// SPDX-License-Identifier: UNLICENSED

import "https://raw.githubusercontent.com/NomicFoundation/hardhat/main/packages/hardhat-core/console.sol";

pragma solidity ^0.8.10;

contract VoteContract {

    function recordVote (string memory user, uint256 votes) external view {
        uint256 time = block.timestamp;
        console.log("User:", user);
        console.log("Votes:", votes);
        console.log("Timestamp:", time);
    }

}
