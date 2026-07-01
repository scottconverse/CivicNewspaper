# Tester Visibility Report - window render rerun b5b873a

Date: 2026-07-01T03:50:30Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: b5b873a0da6ee9712a8ca1633464c6ee261dd5fc
Directive: test-comms/directives/20260630-window-render-rerun-b5b873a.md

## Visibility

- Pulled coordination branch: yes, fast-forwarded to e079020.
- Active directive confirmed: test-comms/directives/20260630-window-render-rerun-b5b873a.md.
- Installer path: test-comms/artifacts/20260630-window-render-rerun-b5b873a/The Civic Desk_0.3.1_x64-setup.exe.
- Installer byte size observed: 5629721.
- Installer SHA256 observed: A3FDF4BCA93EFBC77A085C5C96063F419DBA640C4B9CA8F913B053BBC5A5439D.
- Installer verification result: PASS; observed size and SHA256 match directive.

## Environment Snapshot

- Windows product: Windows 10 Home, build family 10.0.26100.1.
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H.
- RAM: 16870060032 bytes.
- GPU: Intel(R) UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU.
- Disk free on C:: 376540712960 bytes.

## Next

Continuing with the product clean wipe, install, and required no-manipulation packaged-window render gate. If the installed app does not visibly render within 30 seconds of a normal launch, I will stop and report that blocker per directive.
