// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Script.sol";

contract Deploy is Script {
    function run() public {
        vm.startBroadcast();

        // Compile Labyrinth contract
        string[] memory cmds = new string[](3);
        cmds[0] = "huffc";
        cmds[1] = string("src/Labyrinth.huff");
        cmds[2] = "-b";
        bytes memory code = vm.ffi(cmds);

        // Deploy Labyrinth contract
        assembly {
            pop(create(0, add(code, 0x20), code))
        }

        vm.stopBroadcast();
    }
}