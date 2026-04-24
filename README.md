# Project Title

Freight Payment – A Soroban Smart Contract for Shipping Cost Splitting on Stellar

## Project Vision

This project demonstrates a **freight payment smart contract on Soroban and Stellar**. It enables trustless handling of shipping payments where:
- Shippers create shipments and lock funds
- Carriers confirm delivery
- Funds are released only after delivery confirmation

The goal is to provide a secure, transparent mechanism for freight payment escrowed on the Stellar blockchain.

---

## Description

A Soroban smart contract dApp that manages freight payment on Stellar Testnet. It handles the complete lifecycle of a shipment: creation, delivery confirmation, and payment release — all with on-chain transparency and trustless execution.

---

## Features

### 1. Secure Escrow
- Shipper creates shipment with carrier and cost
- Payment is held until delivery is confirmed
- No middleman required

### 2. Two-Step Release
- Carrier confirms delivery on-chain
- Shipper releases payment after confirmation
- Full audit trail on Stellar

### 3. On-chain Transparency
- All shipment data stored permanently on blockchain
- Anyone can verify shipment status
- Immutable record of all state transitions

### 4. Beginner-Friendly
- Clear, readable Rust code for Soroban
- Minimal, working example for learning

---

## Contract Functions

- **create_shipment(shipment_id, shipper, carrier, cost)** – Shipper creates a shipment entry
- **confirm_delivery(shipment_id)** – Carrier confirms delivery
- **release_payment(shipment_id)** – Shipper releases payment to carrier
- **get_shipment(shipment_id)** – Returns (shipper, carrier, cost, delivered, paid)

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CB76IO5TWOVXVVANNVS3IOFERLJ2Q5C7X4FBDJQOMGYRYZSW6KWLFRCL](https://stellar.expert/explorer/testnet/tx/9e9ce4440b3c07e0aac2d6045e00d5e988ac4dc4267945d8a8bacc67ab72972b)

![screenshot](https://i.ibb.co/dJ6mp5Vq/image.png)

---

## Future Scopes

### 1. Multi-Carrier Split Payments
- Split costs across multiple carriers for complex routes

### 2. Dispute Resolution
- Add arbiter mechanism for contested deliveries

### 3. Insurance Integration
- Include insurance coverage as part of shipment cost

### 4. Time-Locked Releases
- Auto-release payment if not disputed within a time window

### 5. Frontend dApp
- Build a simple web interface for shippers and carriers to interact

### 6. Tokenized Payments
- Use custom Stellar tokens (Soroban Token) instead of native XLM

---

## Profile

- **Name:** servicemayita
