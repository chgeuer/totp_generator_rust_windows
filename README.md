# Generate TOTP codes, and put them into the clipboard

The utility reads a JSON from STDIN, takes the "JSON Path" from the command line arguments, and then generates a TOTP code based on the secret. 

## Security warning

Tools such as Google Authenticator on your phone scan the secret value to generate TOTP codes, but lock these secrets deep in the applications local storage on the phone. So your phone is the 2nd factor (2FA) in addition to your laptop. This utility externalizes the secret storage into a JSON file, and it's up to you to protect that JSON file properly. If an attacker is able to get a copy of that secrets.json below, your 2FA approach is toast...

# !!!USE AT YOUR OWN RISK!!!

## Demo

```msdos

> cargo build --release
   Compiling totp_generator_rust_windows v0.1.0 (C:\github\chgeuer\totp_generator_rust_windows)
   Finished release [optimized] target(s) in 19.64s  


> type secrets.json
{
  "github": "HJK",
  "paypal": "XXXYYE5FOAGW5ML7LWWWL4WTZLNJAMXX",
  "google": {
    "alice@gmail.com": "WWWYYE5FOAGW5ML7LRWUL4WTZLNJAMXX",
    "bob@gmail.com": "XXXYYE5FOAGW5ML7LRWUL4WTZLNJAMXX"
  },
  "microsoft": {
    "liveid": {
      "alice@gmail.com": "WWWYYE5FOAGW5ML7LRWUL4WTZLNJAWWW"
    }
  }
}

> type secrets.json | target\release\totp_generator_rust_windows.exe paypal
Copied code for paypal to clipboard

>type secrets.json | target\release\totp_generator_rust_windows.exe google alice@gmail.com
Copied code for google/alice@gmail.com to clipboard

>type secrets.json | target\release\totp_generator_rust_windows.exe microsoft liveid alice@gmail.com
Copied code for microsoft/liveid/alice@gmail.com to clipboard
```
