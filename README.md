<div align="center">
  <h4><span style="font-weight: normal">Your identity, your data — </span>Anchored in <a href="https://mist.id">Mist</a><h4>

![License](https://img.shields.io/badge/License-Apache_2.0%2C_MIT-black)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-black)
![Status](https://img.shields.io/badge/Status-proof%20of%20concept-white)
![CI Status](https://img.shields.io/github/actions/workflow/status/mist-id/mist/ci.yml?label=CI)

</div>

> **Mist** is an open-source Identity and Access Management (IAM) platform designed to bridge the gap between emerging technologies within the Self-Sovereign Identity (SSI) space and traditional web applications.

## Overview

### What is Self-Sovereign Identity (SSI)?

Self-Sovereign Identity (SSI) is a model for managing digital identities where an individual or business has sole
ownership over the ability to control their accounts and personal data. Key concepts include:

- **SIOP (Self-Issued OpenID Provider)**: A protocol allowing users to authenticate using their own identity provider.
- **Decentralized Identifiers (DIDs)**: Unique identifiers that enable verifiable, decentralized digital identity.
- **Verifiable Credentials (VCs)**: Claims made by an issuer about a subject that can be cryptographically verified.

### Project Goals

The vision is to simplify the integration of Self-Sovereign Identity technologies into traditional applications.
Mist aims to:

1. Lower the barrier to entry for developers wanting to implement SSI in their projects.
2. Provide a seamless, user-friendly IAM experience built on SSI principles.
3. Increase adoption of SSI technologies by offering an easy-to-use, robust solution.

### Current State

I have a working prototype with basic functionality, but it is not yet feature-complete or production-ready.
Current capabilities include:

- Service creation via API
- Basic authentication flow using SIOPv2 and DIDs
- User information retrieval via Verifiable Presentations (VPs) and Verifiable Credentials (VCs)
- Simple user creation and storage within a service
- Basic session management

Check out the [roadmap](https://github.com/orgs/mist-id/projects/1) for more information on upcoming features.

## Demo

There's a [live demo](https://mist-demo.fly.dev) available to see the authentication flow in action.
You'll need an SIOP-compliant wallet to authenticate, such as [Sphereon's mobile wallet](https://github.com/Sphereon-Opensource/mobile-wallet).

The source code for the demo is available in the [`demo`](demo) directory.

## Looking to Contribute?

Check the [development docs](https://docs.mist.id/development/quick-start) for more information on how to get started with development.

## Acknowledgements

- [SpruceID](https://spruceid.com) for their work on the [SSI](https://lib.rs/crates/ssi) library.
- [The DIF](https://identity.foundation) for the [Universal Resolver](https://uniresolver.io) and much more.
- [Sphereon](https://sphereon.com) for their [mobile wallet](https://github.com/Sphereon-Opensource/mobile-wallet).
