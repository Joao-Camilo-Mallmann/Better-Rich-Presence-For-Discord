## ADDED Requirements

### Requirement: External Image URL Resolution
The system SHALL preserve HTTP and HTTPS image URLs when setting Rich Presence activity assets instead of replacing them with a static fallback string.

#### Scenario: Valid HTTPS Image URL provided
- **WHEN** an application rule or app registry entry provides a valid HTTPS icon URL (e.g., `https://raw.githubusercontent.com/...` or `https://cdn.example.com/icon.png`)
- **THEN** the system passes the URL format intact to the Discord RPC payload builder.

### Requirement: Discord External Asset Proxy Formatting
The system SHALL format external HTTPS URLs according to Discord's external asset format when sending Rich Presence payload images over RPC.

#### Scenario: Sending external asset over Discord RPC
- **WHEN** Discord RPC activity payload is assembled with an external HTTPS image URL
- **THEN** the image asset field is populated with the correctly formatted external proxy identifier or URL string.
