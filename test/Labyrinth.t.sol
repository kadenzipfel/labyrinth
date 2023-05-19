// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Labyrinth.sol";

contract LabyrinthTest is Test {
    Labyrinth public labyrinth;

    function setUp() public {
        labyrinth = new Labyrinth();
    }

    function testTraverseNoWalls() public {
        uint256 solution = 0x777777777777777;
        assertTrue(labyrinth.verify(0, solution));
    }
}
