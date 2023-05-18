// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Labyrinth.sol";

contract LabyrinthTest is Test {
    Labyrinth public labyrinth;

    function setUp() public {
        labyrinth = new Labyrinth();
    }
}
