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
    function verify(uint256 _start, uint256 _solution) external pure returns (bool) {
        uint256 pos;

        for (uint256 i; i < 0xff;) {
            uint256 dir = _solution >> (i * 2) & 0x3;

            if (dir == 0x0) {
                pos -= 0x10;
            } else if (dir == 0x1) {
                if ((pos + 1) % 0x10 == 0x0) {
                    return false;
                }
                ++pos;
            } else if (dir == 0x2) {
                if (pos % 0x10 == 0x0) {
                    return false;
                }
                --pos;
            } else if (dir == 0x3) {
                pos += 0x10;
            }

            if (_start >> pos == 1) return false;
            if (pos == 0xff) return true;

            unchecked {
                ++i;
            }
        }

        return false;
    }
}