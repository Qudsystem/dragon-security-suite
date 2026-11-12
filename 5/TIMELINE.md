<!-- Original Path: C:\Users\Hossam Galal\Desktop\RED\TIMELINE.md -->

# Dragon R1 Timeline

This document outlines the high-level timeline for the Unified UI Consolidation phases.

## Phase 1: Preparation (Weeks 1-2)
- Architecture finalization
- Service inventory and analysis
- Establishing docker-compose dependencies

## Phase 2: Web UI Consolidation (Weeks 3-5)
- Adapting React/Vite components to Next.js App Router
- Integrating dragon-ui layout globally
- Validating routing and rendering of Cerberus/webapp and Pentagi UIs

## Phase 3: AI Services Unification (Weeks 6-8)
- Integrating Control Interfaces (DeepCode, CAI, Mem0)
- Building unified chat terminal (Nanochat/CAI)
- Bidirectional WebSocket integration for real-time services

## Phase 4: UX & Branding Standardization (Weeks 9-10)
- Enforcing Dragon hierarchical branding across all routes
- Consistent font and color applying
- Consolidating configuration schemas

## Phase 5: Testing & Deprecation (Weeks 11-12)
- Complete End-to-End Cypress integration tests
- Load testing under simulated 100 concurrent users
- Setting source UIs to read-only and final cut-over to dragon-ui
