# Security Policy

## Supported Versions

We take security seriously. Currently, we provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in zkVM from Scratch, please report it responsibly:

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please send an email to: **nullifier-labs@proton.me**

Include the following information:
- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability

### What to Expect

1. **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours
2. **Assessment**: We will assess the vulnerability and determine its impact within 7 days
3. **Resolution**: We will work on a fix and coordinate disclosure timeline
4. **Credit**: We will credit you in the security advisory (unless you prefer to remain anonymous)

### Security Considerations

This project implements cryptographic primitives and zero-knowledge proof systems. Special attention should be paid to:

#### Critical Security Areas
- **Cryptographic Operations**: All hash functions, signature schemes, and field arithmetic
- **Memory Safety**: Especially in `unsafe` code blocks and WebAssembly boundaries
- **Proof System Soundness**: Ensuring proofs cannot be forged or manipulated
- **Side-Channel Attacks**: Constant-time operations in sensitive functions
- **Input Validation**: Proper bounds checking and sanitization

#### Known Security Assumptions
- The project is in early development (Phase 1) and should NOT be used in production
- Cryptographic implementations have not undergone formal security audits
- Performance optimizations may introduce timing side-channels

#### Development Security Practices
- All cryptographic code requires thorough review
- Security-critical PRs require multiple approvals
- Constant-time operations are enforced where necessary
- Dependencies are regularly audited with `cargo audit`

### Scope

This security policy covers:
- Core zkVM implementation
- Cryptographic primitives
- WebAssembly bindings
- Build and deployment scripts

Out of scope:
- Third-party dependencies (report to respective maintainers)
- Issues requiring physical access to systems
- Social engineering attacks

### Security Roadmap

- **Phase 7**: Formal security audit and vulnerability assessment
- **Phase 8**: Security documentation and hardening guidelines

## Security Updates

Security updates will be released as patch versions and announced through:
- GitHub Security Advisories
- Release notes with security tag
- Direct notification to maintainers

Thank you for helping keep zkVM from Scratch secure!