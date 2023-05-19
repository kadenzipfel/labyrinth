// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {IPuzzle} from "./IPuzzle.sol";

contract Labyrinth is IPuzzle {
    /// @inheritdoc IPuzzle
    function name() external pure returns (string memory) {
        return "Labyrinth";
    }

    /// @inheritdoc IPuzzle
    function generate(address _seed) external view returns (uint256) {
        return uint256(keccak256(abi.encode(_seed, block.number)));
    }

    /// @inheritdoc IPuzzle
    function verify(uint256 _start, uint256 _solution) external returns (bool) {
        
    }
}