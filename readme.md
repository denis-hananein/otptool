# One Time Password Tool

Simple tool to help you migrate your OTP secrets from Google Authenticator to another app.
It supports the following formats:

- QR code
- Link (`otpauth-migration://`)

## Install

To install CLI

```bash
cargo install --git https://github.com/denis-hananein/otptool.git
```

To install as a library:

```bash
cargo add otptool
```

## Usage

```bash
otptool --image ./path/to/qr.jpg
```

```
> name: SVC#1, issuer: -, secret: XXXXXXXXXXXXXX
> name: d@emailsvc.com, issuer: BLA, secret: XXXXXXXXXXXXXX
> name: SVC#2, issuer: Atlassian, secret: XXXXXXXXXXXXXX
```

```bash
otptool --link otpauth-migration://...
```

## Thx

- https://github.com/dim13/otpauth
