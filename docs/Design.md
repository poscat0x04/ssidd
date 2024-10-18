# Design Document

## Overall Functionality

SSIDd monitors the connected network, checks
if the network satisfies some conditions, and exposes
the result through a systemd target unit.

### Monitoring Networks

Currently, SSIDD assumes the existence of network managers
(hereby referred to as "backends"). The currently supported
backends are:

- NetworkManager

The following backends will probably be added sometime in the future:

- systemd-networkd
- connman

When there are multiple connected networks, we use the
default IPv4 route with the highest priority to determine the
active network (networks without (global) IPv4 connectivity are ignored).

### Checking Conditions

TBA

### Exposing State

TBA

## Components

TBA

### Server Daemon

TBA

### Client Daemon

TBA

### Communication

TBA
