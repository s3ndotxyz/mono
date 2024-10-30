// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IS3NTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

/**
 * @title Primary entrypoint for procuring services from S3N.
 * @author Layr Labs, Inc.
 */
contract S3NServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IS3NTaskManager
        public immutable S3NTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyS3NTaskManager() {
        require(
            msg.sender == address(S3NTaskManager),
            "onlyS3NTaskManager: not from credible squaring task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        IS3NTaskManager _S3NTaskManager
    )
        ServiceManagerBase(
            _avsDirectory,
            IRewardsCoordinator(address(0)), // inc-sq doesn't need to deal with payments
            _registryCoordinator,
            _stakeRegistry
        )
    {
        S3NTaskManager = _S3NTaskManager;
    }

    /// @notice Called in the event of challenge resolution, in order to forward a call to the Slasher, which 'freezes' the `operator`.
    /// @dev The Slasher contract is under active development and its interface expected to change.
    ///      We recommend writing slashing logic without integrating with the Slasher at this point in time.
    function freezeOperator(
        address operatorAddr
    ) external onlyS3NTaskManager {
        // slasher.freezeOperator(operatorAddr);
    }
}