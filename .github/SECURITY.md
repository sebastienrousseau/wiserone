# Security

We take the security of our software products and services seriously, which includes all source code repositories managed through our GitHub repositories.

## Contact Information

To report a security vulnerability privately, use either channel:

- GitHub private vulnerability reporting:
  <https://github.com/sebastienrousseau/wiserone/security/advisories/new>
- Email: <contact@wiserone.com>

Do not open a public issue for a vulnerability.

## Supported Versions

Security fixes are made in the latest release only. wiserone is pre-1.0
and versions advance by 0.0.1, so upgrading to the newest release is the
supported way to receive a fix.

We accept reports in the following languages:English or French.

## Reporting Security Issues

When reporting a security issue, please include as much of the following information as possible to help us understand the nature and scope of the possible issue:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it
- This information will help us triage your report more quickly.

## Response Time

We aim to acknowledge receipt of your vulnerability report within 48 hours and will strive to keep you informed of the progress we're making toward resolving the issue.

## Disclosure Policy

Once we've resolved a reported security issue, we may disclose it publicly. We will coordinate the disclosure with the person who reported the issue to ensure that they are credited for their discovery.

## Response Process

1. **Acknowledge** the report within 48 hours.
2. **Triage** within 14 days: confirm the issue, assess severity, and
   agree on a disclosure timeline with the reporter.
3. **Fix** on a private branch or GitHub security advisory, with a
   regression test, and run the full CI gate.
4. **Release** a patched version, and publish a GitHub security advisory
   (with a CVE where applicable) that names the fixed version.
5. **Disclose** in the release's Highlights, coordinated with the
   reporter.

## Acknowledgments

Unless they prefer to stay anonymous, reporters are credited by name in
the GitHub security advisory and in the Highlights of the release that
fixes the issue.

## Safe Harbour

We promise not to initiate legal action against researchers for disclosing vulnerabilities as long as they adhere to responsible disclosure guidelines, which includes reporting it to us and not publicly disclosing the issue until we've had a reasonable time to address it.
