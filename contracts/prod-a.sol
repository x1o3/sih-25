pragma solidity ^0.8.19;

// No access control (anyone can call most functions).

contract OilseedValueChain {
    // ======================== CUSTOM ERRORS (Gas Efficient) ========================
    error AlreadyRegistered();
    error FarmerNotRegistered();
    error SKUAlreadyExists();
    error SKUNotFound();
    error AlreadyCommitted();
    error NotCommitted();
    error AlreadyRevealed();
    error InvalidReveal();
    error LengthMismatch();

    // ======================== STAGE 1: FARMER REGISTER ========================
    // On-chain: Minimal farmer DID, crop ID hash
    // Off-chain: Full profile, land docs, satellite info
    // Gas saving: DID + ZK Proofs (KYC not saved on chain)

    struct FarmerRecord {
        bytes32 cropIDHash;
        uint64 registeredAt;
    }

    mapping(bytes32 => FarmerRecord) public farmers;

    event FarmerRegistered(
        bytes32 indexed farmerDID,
        bytes32 indexed cropIDHash,
        uint64 timestamp
    );

    function registerFarmer(bytes32 farmerDID, bytes32 cropIDHash) external {
        if (farmers[farmerDID].registeredAt != 0) revert AlreadyRegistered();

        uint64 timestamp = uint64(block.timestamp);

        farmers[farmerDID] = FarmerRecord({
            cropIDHash: cropIDHash,
            registeredAt: timestamp
        });

        emit FarmerRegistered(farmerDID, cropIDHash, timestamp);
    }

    // ======================== STAGE 2: FPO VERIFICATION ========================
    // On-chain: Event "Ownership Transfer #1" with batch hash
    // Off-chain: Quality report, weight slips, photos
    // Gas saving: Batch-level not unit-level, use uint8 code

    // Transfer type codes for gas efficiency
    uint8 constant TRANSFER_FPO_PURCHASE = 1;
    uint8 constant TRANSFER_WAREHOUSE = 2;
    uint8 constant TRANSFER_PROCESSOR = 3;
    uint8 constant TRANSFER_RETAIL = 4;

    event OwnershipTransfer(
        bytes32 indexed batchHash,
        bytes32 indexed fromDID,
        address indexed toAddress,
        uint64 timestamp,
        uint8 transferType
    );

    function fpoPurchase(bytes32 batchHash, bytes32 farmerDID) external {
        if (farmers[farmerDID].registeredAt == 0) revert FarmerNotRegistered();

        emit OwnershipTransfer(
            batchHash,
            farmerDID,
            msg.sender,
            uint64(block.timestamp),
            TRANSFER_FPO_PURCHASE
        );
    }

    // ======================== STAGE 3: WAREHOUSE STORAGE ========================
    // On-chain: Timed anchor digest: warehouse state hash
    // Off-chain: Continuous IoT logs
    // Gas saving: State rollups—hourly or daily commit

    struct WarehouseState {
        bytes32 stateHash;
        uint64 lastUpdated;
    }

    mapping(bytes32 => WarehouseState) public warehouseStates;

    event WarehouseStateUpdated(
        bytes32 indexed warehouseId,
        bytes32 stateHash,
        uint64 timestamp
    );

    function updateWarehouseState(
        bytes32 warehouseId,
        bytes32 stateHash
    ) external {
        uint64 timestamp = uint64(block.timestamp);

        warehouseStates[warehouseId] = WarehouseState({
            stateHash: stateHash,
            lastUpdated: timestamp
        });

        emit WarehouseStateUpdated(warehouseId, stateHash, timestamp);
    }

    // ======================== STAGE 4: LOGISTICS TRACKING ========================
    // On-chain: Event: location milestone hashes
    // Off-chain: Full GPS history
    // Gas saving: Checkpoints only (city-level)

    event LogisticsMilestone(
        bytes32 indexed shipmentId,
        bytes32 locationHash,
        uint64 timestamp,
        bool isDelivered
    );

    function recordLogistics(
        bytes32 shipmentId,
        bytes32 locationHash,
        bool isDelivered
    ) external {
        emit LogisticsMilestone(
            shipmentId,
            locationHash,
            uint64(block.timestamp),
            isDelivered
        );
    }

    // ======================== STAGE 5: PROCESSING ========================
    // On-chain: Event: processing transform hash
    // Off-chain: Yield %, lab results
    // Gas saving: Hash transformations (parent → children batches)

    event BatchProcessed(
        bytes32 indexed inputBatchHash,
        bytes32 transformHash,
        bytes32[] outputBatchHashes,
        uint64 timestamp
    );

    function processBatch(
        bytes32 inputBatchHash,
        bytes32 transformHash,
        bytes32[] calldata outputBatchHashes
    ) external {
        emit BatchProcessed(
            inputBatchHash,
            transformHash,
            outputBatchHashes,
            uint64(block.timestamp)
        );
    }

    // ======================== STAGE 6: PACKAGING ========================
    // On-chain: SKU ID → Parent batch commitment
    // Off-chain: Packaging metadata
    // Gas saving: Merkle tree for large SKU sets

    struct PackageRecord {
        bytes32 parentBatchHash;
        bytes32 merkleRoot;
        uint64 packagedAt;
    }

    mapping(bytes32 => PackageRecord) public packages;

    event SKUPackaged(
        bytes32 indexed skuId,
        bytes32 indexed parentBatchHash,
        bytes32 merkleRoot,
        uint64 timestamp
    );

    function createSKU(
        bytes32 skuId,
        bytes32 parentBatchHash,
        bytes32 merkleRoot
    ) external {
        if (packages[skuId].packagedAt != 0) revert SKUAlreadyExists();

        uint64 timestamp = uint64(block.timestamp);

        packages[skuId] = PackageRecord({
            parentBatchHash: parentBatchHash,
            merkleRoot: merkleRoot,
            packagedAt: timestamp
        });

        emit SKUPackaged(skuId, parentBatchHash, merkleRoot, timestamp);
    }

    // ======================== STAGE 7: RETAIL ========================
    // On-chain: Optionally: Retail event hash
    // Off-chain: Full retail analytics
    // Gas saving: Only fraud-trigger events recorded

    event FraudDetected(
        bytes32 indexed skuId,
        address indexed retailer,
        bytes32 evidenceHash,
        uint64 timestamp
    );

    function reportFraud(bytes32 skuId, bytes32 evidenceHash) external {
        if (packages[skuId].packagedAt == 0) revert SKUNotFound();

        emit FraudDetected(
            skuId,
            msg.sender,
            evidenceHash,
            uint64(block.timestamp)
        );
    }

    // ======================== STAGE 8: AI SCORING ========================
    // On-chain: Event: Score commit hash
    // Off-chain: Entire AI model & inputs
    // Gas saving: On-chain commit-reveal to prevent tampering

    struct AIScore {
        bytes32 commitHash;
        bytes32 revealHash;
        uint64 committedAt;
        uint64 revealedAt;
    }

    mapping(bytes32 => AIScore) public aiScores;

    event AIScoreCommitted(
        bytes32 indexed batchHash,
        bytes32 commitHash,
        uint64 timestamp
    );

    event AIScoreRevealed(
        bytes32 indexed batchHash,
        bytes32 revealHash,
        uint64 timestamp
    );

    function commitAIScore(bytes32 batchHash, bytes32 commitHash) external {
        if (aiScores[batchHash].committedAt != 0) revert AlreadyCommitted();

        uint64 timestamp = uint64(block.timestamp);

        aiScores[batchHash].commitHash = commitHash;
        aiScores[batchHash].committedAt = timestamp;

        emit AIScoreCommitted(batchHash, commitHash, timestamp);
    }

    function revealAIScore(
        bytes32 batchHash,
        bytes32 revealHash,
        bytes32 nonce
    ) external {
        AIScore storage score = aiScores[batchHash];

        if (score.committedAt == 0) revert NotCommitted();
        if (score.revealedAt != 0) revert AlreadyRevealed();

        // Verify commit matches reveal
        bytes32 expectedCommit = keccak256(abi.encodePacked(revealHash, nonce));
        if (score.commitHash != expectedCommit) revert InvalidReveal();

        uint64 timestamp = uint64(block.timestamp);

        score.revealHash = revealHash;
        score.revealedAt = timestamp;

        emit AIScoreRevealed(batchHash, revealHash, timestamp);
    }

    // ======================== BATCH OPERATIONS FOR GAS OPTIMIZATION ========================
    // Process multiple records in single transaction

    function batchUpdateWarehouse(
        bytes32[] calldata warehouseIds,
        bytes32[] calldata stateHashes
    ) external {
        uint256 length = warehouseIds.length;
        if (length != stateHashes.length) revert LengthMismatch();

        uint64 timestamp = uint64(block.timestamp); // Cache timestamp

        for (uint256 i = 0; i < length; ) {
            warehouseStates[warehouseIds[i]] = WarehouseState({
                stateHash: stateHashes[i],
                lastUpdated: timestamp
            });

            emit WarehouseStateUpdated(
                warehouseIds[i],
                stateHashes[i],
                timestamp
            );

            unchecked {
                ++i;
            }
        }
    }

    function batchRecordLogistics(
        bytes32[] calldata shipmentIds,
        bytes32[] calldata locationHashes,
        bool[] calldata deliveryStatuses
    ) external {
        uint256 length = shipmentIds.length;
        if (
            length != locationHashes.length || length != deliveryStatuses.length
        ) revert LengthMismatch();

        uint64 timestamp = uint64(block.timestamp); // Cache timestamp

        for (uint256 i = 0; i < length; ) {
            emit LogisticsMilestone(
                shipmentIds[i],
                locationHashes[i],
                timestamp,
                deliveryStatuses[i]
            );

            unchecked {
                ++i;
            }
        }
    }

    // ======================== VERIFICATION FUNCTIONS ========================

    function verifyPackageOrigin(
        bytes32 skuId
    )
        external
        view
        returns (bytes32 parentBatchHash, bytes32 merkleRoot, uint64 packagedAt)
    {
        PackageRecord memory pkg = packages[skuId];
        if (pkg.packagedAt == 0) revert SKUNotFound();

        return (pkg.parentBatchHash, pkg.merkleRoot, pkg.packagedAt);
    }

    function verifyFarmer(
        bytes32 farmerDID
    )
        external
        view
        returns (bool exists, bytes32 cropIDHash, uint64 registeredAt)
    {
        FarmerRecord memory farmer = farmers[farmerDID];
        return (
            farmer.registeredAt != 0,
            farmer.cropIDHash,
            farmer.registeredAt
        );
    }

    function getWarehouseState(
        bytes32 warehouseId
    ) external view returns (bytes32 stateHash, uint64 lastUpdated) {
        WarehouseState memory state = warehouseStates[warehouseId];
        return (state.stateHash, state.lastUpdated);
    }

    function getAIScore(
        bytes32 batchHash
    )
        external
        view
        returns (
            bytes32 commitHash,
            bytes32 revealHash,
            uint64 committedAt,
            uint64 revealedAt
        )
    {
        AIScore memory score = aiScores[batchHash];
        return (
            score.commitHash,
            score.revealHash,
            score.committedAt,
            score.revealedAt
        );
    }
}
