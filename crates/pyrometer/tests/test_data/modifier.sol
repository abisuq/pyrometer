// SPDX-License-Identifier: MIT or APACHE2
pragma solidity ^0.8.0;

contract Modifier {
    uint256 a;

    // modifier Noop() {
    //     _;
    // }

    // modifier RequireBefore() {
    //     require(a == 0);
    //     _;
    // }

    // modifier RequireAfter() {
    //     _;
    //     require(a == 1);
    // }

    modifier Input(uint256 l) {
        require(l == 100);
        a += 1;
        _;
        a += 1;
    }

    // function noop() public Noop {
    //     a = 100;
    // }

    // function requireBefore() public RequireBefore {
    //     a += 1;
    // }

    // function requireAfter() public RequireAfter {
    //     a += 1;
    // }

    // function requireBoth() public RequireBefore RequireAfter {
    //     a += 1;
    // }

    // function input(uint256 b) public Input(b) {
    //     uint256 a = b;
    //     require(a == 2);
    // }

    function input(uint256 b, uint256 q) public Input(b) Input(q) {
        uint256 k = b;
        k;
        require(a == 4);
    }

    // function internalMod(uint256 b) internal Input(b) {
    //     uint256 k = b;
    //     k;
    //     require(a == 2);
    // }

    // function internalModPub(uint256 b) public {
    //     internalMod(b);
    // }

    // function addOne(uint256 x) internal pure returns (uint256) {
    //     return x + 1;
    // }

    // function inputFunc(uint256 x) internal Input(addOne(x)) returns (uint256) {
    //     return x;
    // }

    // function inputFuncConst(
    //     uint256 x
    // ) internal Input(addOne(99)) returns (uint256) {
    //     require(a == 2);
    //     return x;
    // }

    // function inputFunc_conc() internal returns (uint256) {
    //     uint256 y = inputFunc(99);
    //     require(a == 2);
    //     return y;
    // }
}
