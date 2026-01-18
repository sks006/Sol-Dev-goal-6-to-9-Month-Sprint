    
    RentFlow/
    
    ├── anchor/                     # SMART CONTRACTS (Rust/
    Anchor)
    
    │   ├── programs/
    
    │   │   ├── rent_core/          # Logic Engine
    
    │   │   │   ├── src/
    
    
    │   │   │   │   ├── lib.rs      # Main entry & Instruction routing
    
    │   │   │   │   ├── state.rs    # BookingObligation, Vault 
    structures [cite: 82]
    
    │   │   │   │   ├── errors.rs   # Custom Hackathon-specific 
    errors [cite: 83]
    
    │   │   │   │   └── instructions/
    
    │   │   │   │       ├── init_vault.rs     # PDA Vault creation [cite: 93]
    
    │   │   │   │       ├── mint_booking.rs   # Token-2022 Minting
    
    │   │   │   │       ├── deposit_collateral.rs # Locking the NFT
    
    │   │   │   │       └── withdraw_liquidity.rs # The USDC 
    payout logic
    
    │   │   └── compliance_hook/    # Security Enforcer
    
    │   │       └── src/
    
    │   │           └── lib.rs      # Rule: Reject transfer if 
    loan is active [cite: 131]
    
    │   ├── Anchor.toml
    
    │   └── tests/                  # Integrity Testing [cite: 99]
    
    │       └── rent_flow.ts        # Happy path: Mint -> Lock -> Borrow
    
    │
    
    ├── api/                        # BACKEND (Node.js/Express or Fastify)
    
    │   ├── src/
    
    │   │   ├── routes/
    
    │   │   │   └── airbnb_proxy.ts # Mocks the Airbnb API for 
    the demo
    
    │   │   ├── services/
    
    │   │   │   └── metadata_gen.ts # Generates off-chain NFT metadata
    
    │   │   └── server.ts
    
    │   └── package.json
    
    │
    
    └── web/                        # FRONTEND (Next.js + Tailwind)
    
        ├── src/
    
        │   ├── app/                # App Router (Next.js 14+)
    
        │   ├── components/
    
        │   │   ├── WalletProvider.tsx
    
        │   │   ├── BookingCard.tsx # Visualizes the RWA Asset
    
        │   │   └── DashBoard.tsx   # Real-time LTV & Interest 
        tracking
    
        │   └── hooks/
    
        │       └── useRentFlow.ts  # Anchor Program Interactivity
    
        └── tailwind.config.js