# FarmCredit Smart Contract API Reference

All contracts are deployed on the Stellar network (Soroban). Invoke them via the Stellar CLI or the `@stellar/stellar-sdk` JS client.

---

## Contracts

| Contract | Purpose |
|---|---|
| `escrow` | Single-tree sponsor escrow with optional 1-year survival insurance guarantee (#1021) and platform fees (#467) |
| `tree-escrow` | Two-tranche donor escrow (75% on planting, 25% after 6 months) with optional survival guarantee |
| `escrow-milestone` | Single-milestone escrow with remainder release |
| `donation-escrow` | Campaign donation escrow w/ XLM / USDC / EURC rails + recurring subscriptions |
| `location-proof` | ZK location proofs for Northern Nigeria boundary |
| `nullifier-registry` | SHA-256 commitment registry — prevents double-counting |
| `species-voting` | On-chain governance for adding new tree species to the catalogue |
| `soil-health` | Soil health scoring for regenerative farming practices + soil carbon sequestration bonus credits |

---

## Common Patterns

### Authorization

Functions marked **Admin-only** require the admin address (set at `initialize`) to sign the transaction. Functions marked **caller-auth** require the calling address to sign.

### Error Handling

Contracts panic with a descriptive string on invalid input. The Stellar SDK surfaces these as `InvokeHostFunctionError` with the panic message in `result_xdr`. Common patterns:

| Panic message | Meaning |
|---|---|
| `"already initialized" | `initialize` called more than once |
| `"amount must be positive"` | `amount ≤ 0` passed to `deposit` |
| `"active escrow already exists for this farmer"` | Duplicate `deposit` for same farmer |
| `"no escrow for farmer"` / `"no escrow found for farmer"` | Farmer address has no escrow record |
| `"commitment already registered"` | Duplicate nullifier / replay attempt |
| `"location outside Northern Nigeria boundary"` | `in_region = false` passed to `submit_proof` |
| `"must hold TREE tokens to vote"` | Voter has zero TREE token balance |
| `"already voted on this proposal"` | Duplicate vote attempt |
| `"proposal has not passed"` | Attempting to execute a non-passed proposal |
| `"planting density below minimum for job size"` — Job area meets threshold but density is too low |
| `"area hectares must be positive"` — `area_hectares ≤ 0` |
| `"survival not yet verified"` — Attempting to call 1-year milestone before survival check |
| `"1-year milestone period not yet elapsed"` — Called before 1 year elapsed since planting |
| `"rating must be between 1 and 5"` — Rating outside valid range |
| `"can only rate after escrow is completed"` — Rating before job completion |
| `"only the original donor can rate the planter"` — Non-donor attempting to rate |
| `"sponsor has already rated this planter"` — Duplicate rating attempt |
| `"soil health score must be between 0 and 1000"` — Score outside valid range |
| `"baseline soil health score must be between 0 and 1000`` | Baseline outside valid range |
| `"improvement score must be between 0 and 1000`` | Improvement outside valid range |
| `"no soil health record for farmer"` | No record found for farmer |
| `"soil health record already exists for farmer"` | Duplicate registration attempt |
| `"carbon credits must be positive"` — `carbon_credits_awarded ≤ 0` |

---

## escrow (Single-Tree Escrow & Sponsor Insurance #1021)

Manages single-tree sponsorships with optional **1-Year Survival Insurance Guarantee (#1021)** and **Platform Fee on Release (#467)**.

### Sponsor Insurance Overview
- **Optional Guarantee:** Sponsors can pay a **+2.00% fee** (200 bps) at deposit time (`deposit_with_insurance`) or add it to a pending deposit (`purchase_insurance`).
- **1-Year Survival Guarantee:** Protects the sponsor for 365 days (`31_536_000` seconds).
- **Full Refund on Death:** If the tree dies within 1 year, the sponsor receives a **100% full refund** of their deposit via `claim_insurance_refund` or verifier `report_tree_dead`.

### `deposit_with_insurance`
Sponsor deposits funds for a tree with the 1-year survival guarantee. Transfers `amount + (amount * 2%)` from sponsor.

**Auth:** `sponsor` (caller-auth)

| Parameter | Type | Description |
|---|---|---|
| `sponsor` | `Address` | Sponsor paying for tree + 2% insurance guarantee |
| `planter` | `Address` | Planter planting the tree |
| `tree_id` | `u64` | Target tree ID |
| `token` | `Address` | SAC token contract address |
| `amount` | `i128` | Tree deposit amount |

### `claim_insurance_refund`
Sponsor claims a 100% refund of deposit `amount` if their insured tree died within the 1year guarantee period.

**Auth:** `sponsor` (caller-auth)

### `report_tree_dead`
Verifier / admin marks an insured tree as dead, automatically refunding 100% of the deposit `amount` to the sponsor.

**Auth:** `verifier` / `admin`

### `get_insurance_info`
Returns `(has_insurance: bool, insurance_fee: i128, expires_at: u64, is_active: bool)`.

---

## tree-escrow

State machine: `Funded → Planted → Survived → Completed` (or `Funded → Refunded`)

**Time-Locked Milestones (#494):** Funds are released in 3 tranches:
- Tranche 1 (30%) at planting verification
- Tranche 2 (40%) at 6-month survival check
- Tranche 3 (30%) at 1-year milestone

**Minimum Planting Density Rule (#514):** For jobs with `area_hectares` ≥ `job_size_threshold`, the contract enforces a minimum planting density of `min_density` trees per hectare. Small jobs below the threshold are exempt from density rules.

**Planter Rating System (#483):** After job completion, sponsors can rate planters (1-5 stars). Ratings are stored on-chain and aggregated into a reputation score (0-100) to track planter performance over time.

**Minimum Planting Density Rule (#514):** For jobs with `area_hectares` ≥ `job_size_threshold`, the contract enforces a minimum planting density of `min_density` trees per hectare. Small jobs below the threshold are exempt from density rules.

### `initialize`

One-time setup. Must be called before any other function.

**Auth:** deployer (anyone, once)

| Parameter | Type | Description |
|---|---|---|
| `admin` | `Address` | Address that will act as verifier/admin |
| `tree_token` | `Address` | TREE token contract address |
| `oracle` | `Address` | Oracle address for survival reports |
| `survival_threshold_percent` | `u32` | Minimum survival rate (0..=100) for Tranche 2 |
| `min_density` | `i128` | Minimum trees per hectare for large jobs |
| `job_size_threshold` | `i128` | Minimum job size (hectares) for density rules |

**Returns:** `void`

**Errors:** panics with `"already initialized"` if called again.

```bash
stellar contract invoke \
  --id $CONTRACT_ID --network testnet --source deployer \
  -- initialize \
    --admin GADMIN... \
    --tree_token GTREE... \
    --oracle GORACLE... \
    --survival_threshold_percent 70 \
    --min_density 1000 \
    --job_size_threshold 10
```

```ts
await client.initialize({
  admin: adminAddress,
  tree_token: treeTokenAddress,
  oracle: oracleAddress,
  survival_threshold_percent: 70,
  min_density: 1000,
  job_size_threshold: 10,
});
```

---

### `deposit`

Donor deposits funds into escrow for a specific farmer. Transfers `amount` of `token` from `donor` into the contract.

**Auth:** `donor` (caller-auth)

| Parameter | Type | Description |
|---|---|---|
| `donor` | `Address` | Address funding the escrow |
| `farmer` | `Address` | Beneficiary farmer address |
| `token` | `Address` | SAC token contract address (e.g. USDC) |
| `amount` | `i128` | Amount in token's smallest unit (must be > 0) |
| `tree_count` | `i128` | Number of trees to be planted (must be > 0) |
| `area_hectares` | `i128` | Planting area in hectares (must be > 0) |

**Returns:** `void`

**Events emitted:** `DonationReceived(donor, farmer) → (amount, token)`**
**Errors:**
- `"amount must be positive"` — `amount ≤ 0`- `"active escrow already exists for this farmer"` — farmer already has an open escrow
- `"planting density below minimum for job size"` — Job area meets threshold but density is too low
- `"area hectares must be positive"` — `area_hectares ≤ 0`

```bash
stellar contract invoke \
  --id $CONTRACT_ID --network testnet --source donor \
  -- deposit \
    --donor GDONOR... \
    --farmer GFARMER... \
    --token GUSDC... \
    --amount 10000000 \
    --tree_count 5000 \
    --area_hectares 5
```

```ts
await client.deposit({
  donor: donorAddress,
  farmer: farmerAddress,
  token: usdcAddress,
  amount: BigInt(10_000_000), // 1 USDC (7 decimals)
  tree_count: BigInt(5_000),
  area_hectares: BigInt(5),
});
```

---

### `verify_planting`

Admin confirms GPS + photo proof of planting. Releases **Tranche 1 (30%)** of escrowed funds to the farmer immediately and mints TREE tokens.

**Auth:** admin-only

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer whose escrow to update |
| `proof_hash` | `BytesN<32>` | SHA-256 of the GPS + photo proof payload |

**Returns:** `void`

**Events emitted:** `PlantingVerified(farmer) → (tranche1_amount, proof_hash)`

**Errors:**
- `"planting already verified or escrow not active"` — status is not `Funded`
- `"no escrow for farmer"` — no escrow record found

```bash
stellar contract invoke \
  --id $CONTRACT_ID --network testnet --source admin \
  -- verify_planting \
    --farmer GFARMER... \
    --proof_hash aabbcc...  # 32-byte hex
```

```ts
const proofHash = Buffer.from(sha256(proofPayload));
await client.verify_planting({
  farmer: farmerAddress,
  proof_hash: proofHash,
});
```

---

### `verify_survival`

Admin confirms 6-month survival check. Releases **Tranche 2 (40%)** to the farmer. Enforces that at least 6 months (≈ 26 weeks) have elapsed since `verify_planting` and survival rate meets threshold.

**Auth:** admin-only

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer whose escrow to update |
| `proof_hash` | `BytesN<32>` | SHA-256 of the survival proof payload |
| `survival_rate_percent` | `u32` | Survival rate (0..=100) |

**Returns:** `void`

**Events emitted:** `SurvivalVerified(farmer) → (tranche2_amount, proof_hash)`

**Errors:**
- `"planting not yet verified"` — status is not `Planted`- `"6-month survival period not yet elapsed"` — called too early
- `"survival rate below minimum"` — survival rate below configured threshold
- `"nothing left to release"` — released amount already equals total

```ts
await client.verify_survival({
  farmer: farmerAddress,
  proof_hash: survivalProofHash,
  survival_rate_percent: 70,
});
```

---

### `verify_year_milestone`

Admin confirms 1-year milestone. Releases **Tranche 3 (30%)** to the farmer. Enforces that at least 1 year (≈ 52 weeks) has elapsed since `verify_planting`.

**Auth:** admin-only

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer whose escrow to complete |
| `proof_hash` | `BytesN<32>` | SHA-256 of the year milestone proof payload |

**Returns:** `void`

**Events emitted:** `YearMilestone(farmer) → (tranche3_amount, proof_hash)`**
**Errors:**
- `"survival not yet verified"` — status is not `Survived`
- `"1-year milestone period not yet elapsed"` — called too early
- `"nothing left to release"` — released amount already equals total

```ts
await client.verify_year_milestone({
  farmer: farmerAddress,
  proof_hash: yearMilestoneProofHash,
});
```

---

### `refund`

Returns the full escrowed amount to the donor. Only callable before planting is verified.

**Auth:** admin-only

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer whose escrow to refund |

**Returns:** `void`

**Events emitted:** `DonationRefunded(donor, farmer) → total_amount`**
**Errors:**
- `"cannot refund after planting has been verified"` — status is not `Funded`

```ts
await client.refund({ farmer: farmerAddress });
```

---

### `get_record`

Read-only. Returns the full escrow record for a farmer.

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer address to look up |

**Returns:** `Option<EscrowRecord>`

```ts
const record = await client.get_record({ farmer: farmerAddress });
// record.status: "Funded" | "Planted" | "Survived" | "Completed" | "Refunded"
// record.total_amount: bigint
// record.released: bigint
```

---

### `rate_planter`

Sponsor rates a planter after job completion. Rating must be 1-5 stars. Only callable by the original donor after escrow is completed. Each sponsor can only rate a specific planter once per escrow.

**Auth:** sponsor (caller-auth)

| Parameter | Type | Description |
|---|---|---|
| `sponsor` | `Address` | Sponsor submitting the rating |
| `farmer` | `Address` | Planter being rated |
| `rating` | `u32` | Rating from 1 to 5 |

**Returns:** `void`

**Events emitted:** `PlanterRated(sponsor, farmer) → rating`

**Errors:**
- `"rating must be between 1 and 5"` — rating outside valid range
- `"can only rate after escrow is completed"` — rating before job completion
- `"only the original donor can rate the planter"` — non-donor attempting to rate
- `"sponsor has already rated this planter"` — duplicate rating attempt

```ts
await client.rate_planter({
  sponsor: sponsorAddress,
  farmer: farmerAddress,
  rating: 5,
});
```

---

### `get_reputation`

Read-only. Returns the aggregated reputation score (0-100) for a planter.

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Planter address to look up |

**Returns:** `unt32` — reputation score (0-100)

```ts
const reputation = await client.get_reputation({ farmer: farmerAddress });
```

---

## soil-health (Soil Health Scoring & Regenerative Agriculture Incentives #1022)

Measures soil health improvement from regenerative farming practices and awards bonus carbon credits for soil carbon sequestration above a farmer's baseline.

### Overview

Regenerative practices (no-till, utilizing cover crops, composting, agroforestry, etc.) improve soil organic carbon and general soil health. This contract lets a verifier record a soil health score (0..=1000) for a farmer, compares it against the farmer's baseline, and awards bonus carbon credits for verified improvement.

- **Soil Health Score:** integer in `[0, 1000]`. Higher is better.
- **Baseline:** the farmer's pre-regenerative-practice soil health score (0..=1000), recorded on first registration.
- **Improvement Score:** `current_score - baseline_score` (clamped at 0).
- **Bonus Carbon Credits:** awarded for improvement above baseline, scaled by a configurable `bonus_rate_bps .

### `initialize`

One-time setup. Must be called before any other function.

**Auth:** deployer (anyone, once)

| Parameter | Type | Description |
|---|---|---|
| `admin` | `Address` | Address that will act as verifier/admin |
| `carbon_token` | `Address` | Carbon credit token contract address |
| `bonus_rate_bps` | `u32` | Bonus carbon credits awarded per point of improvement, in basis points (e.g. 100 = 1% bonus) |

**Returns:** `void`

**Errors:** panics with `"already initialized"` if called again.

```bash
stellar contract invoke \
  --id $CONTRACT_ID --network testnet --source deployer \
  -- initialize \
    --admin GADMIN... \
    --carbon_token GCARBON... \
    --bonus_rate_bps 100
```

```ts
await client.initialize({
  admin: adminAddress,
  carbon_token: carbonTokenAddress,
  bonus_rate_bps: 100,
});
```

---

### `register_baseline`

Records the farmer's baseline soil health score. Must be called before any improvement can be measured. Only one baseline per farmer.

**Auth:** admin-only

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer whose baseline to record |
| `baseline_score` | `u32` | Baseline soil health score (0..=1000) |

**Returns:** `void`

**Events emitted:** `SoilBaselineRegistered(farmer) → baseline_score`

**Errors:**
- `"baseline soil health score must be between 0 and 1000"` — `baseline_score > 1000`
- `"soil health record already exists for farmer"` — duplicate registration

```ts
await client.register_baseline({
  farmer: farmerAddress,
  baseline_score: 400,
});
```

---

### `record_soil_health`

Records an updated soil health score for a farmer and awards bonus carbon credits for improvement above the registered baseline. The improvement score is `current_score - baseline_score` (clamped at 0), and bonus carbon credits are `ceil(improvement_score * bonus_rate_bps / 10_000)`.

**Auth:** admin-only

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer whose soil health to update |
| `current_score` | `u32` | Current soil health score (0..=1000) |

**Returns:** `u32` — bonus carbon credits awarded

**Events emitted:** `SoilHealthRecorded(farmer) → (current_score, improvement_score, carbon_credits_awarded)`**
**Errors:**
- `"soil health score must be between 0 and 1000"` — `current_score > 1000`
- `"no soil health record for farmer"` — baseline not registered yet

```ts
await client.record_soil_health({
  farmer: farmerAddress,
  current_score: 750,
});
```

---

### `get_soil_health`

Read-only. Returns the farmer's soil health record.

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer address to look up |

**Returns:** `Option<SoilHealthRecord>`

```ts
const record = await client.get_soil_health({ farmer: farmerAddress });
// record.baseline_score: number
// record.current_score: number
// record.improvement_score: number
// record.carbon_credits_awarded: number
```

---

### `get_carbon_bonus`

Read-only. Returns the cumulative bonus carbon credits awarded to a farmer for soil sequestration above baseline.

| Parameter | Type | Description |
|---|---|---|
| `farmer` | `Address` | Farmer address to look up |

**Returns:** `u32` — total bonus carbon credits awarded

```ts
const bonus = await client.get_carbon_bonus({ farmer: farmerAddress });
```
