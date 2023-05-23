// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "foundry-huff/HuffDeployer.sol";

interface Labyrinth {
    function name() external view returns (string memory);
    function generate(address _seed) external view returns (uint256);
    function verify(uint256 _start, uint256 _solution) external returns (bool);
}

contract LabyrinthHuffTest is Test {
    Labyrinth public labyrinth;

    function setUp() public {
        labyrinth = Labyrinth(address(HuffDeployer.deploy("Labyrinth")));
    }

    function testName() public {
        assertEq(keccak256(abi.encode(labyrinth.name())), keccak256(abi.encode("Labyrinth")));
    }
}