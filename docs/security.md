# 🔐 Security Model

This document describes the security posture of the application.

Security is enforced through multiple layers.

---

## HTTP Security Headers

The application applies strict security headers:

- Content-Security-Policy (CSP)
- Strict-Transport-Security (HSTS)
- X-Frame-Options
- X-Content-Type-Options
- Referrer-Policy
- Permissions-Policy
- Cross-Origin-Resource-Policy

These headers reduce risks such as:

- Cross-site scripting (XSS)
- Clickjacking
- MIME confusion
- Cross-origin abuse

---

## Authentication Security

Authentication is implemented using:

- Secure password hashing (Argon2)
- Unique salts per password
- Session-based authentication
- HTTP-only cookies

Cookie properties:

- HttpOnly enabled
- SameSite policy enforced
- Secure flag enabled in production

---

## Rate Limiting

Requests are throttled using:

tower-governor middleware.

This protects against:

- Brute-force attacks
- Resource exhaustion
- Abuse patterns

---

## Container Security

The runtime environment uses:

- Distroless container images
- Non-root user execution
- Minimal attack surface

No shell tools exist inside the runtime container.

---

## Reverse Proxy Protection

Nginx handles:

- TLS termination
- Request filtering
- Static request boundaries

---

## Future Security Improvements

Planned:

- Session expiration cleanup
- Password complexity policies
- Role-based authorization
- Security audit logging