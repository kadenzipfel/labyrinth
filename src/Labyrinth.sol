// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {IPuzzle} from "./IPuzzle.sol";

contract Labyrinth is IPuzzle {
    /// @inheritdoc IPuzzle
    function name() external pure returns (string memory) {
        return "Labyrinth";
    }

    /// @inheritdoc IPuzzle
    function generate(address _seed) external returns (uint256) {
        
    }

    /// @inheritdoc IPuzzle
    function verify(uint256 _start, uint256 _solution) external returns (bool) {
        
    }
}