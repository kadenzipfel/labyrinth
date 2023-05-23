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

    function testTraverseWalls() public {
        vm.roll(524);
        uint256 seed = labyrinth.generate(address(0x3111327EdD38890C3fe564afd96b4C73e8101747));

        assertEq(seed, 0x04020d050d0e54f1031285c9e3fb5283a1c81b8d90016e3a6521244c38eaa307);

        assertTrue(labyrinth.verify(seed, 0x5ff5f7fd5557ddf));
    }

    /// forge-config: default.fuzz.runs = 100000
    function testCannotTraverseWallsRandomSolution(uint16 blockNumber, uint256 solution) public {
        try labyrinth.verify(uint256(keccak256(abi.encode(address(0x3111327EdD38890C3fe564afd96b4C73e8101747), blockNumber))), solution) returns (bool success) {
            assertFalse(success);
        } catch {}
    }
}
