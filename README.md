# Generate TOTP codes, and put them into the clipboard

The utility reads a JSON from STDIN, takes the "JSON Path" from the command line arguments, and then generates a TOTP code based on the secret.

This utility was just an excercise for me personally, to learn a bit of Rust...

## Security warning

Tools such as Google Authenticator on your phone scan the secret value to generate TOTP codes, but lock these secrets deep in the applications local storage on the phone. So your phone is the 2nd factor (2FA) in addition to your laptop. This utility externalizes the secret storage into a JSON file, and it's up to you to protect that JSON file properly. If an attacker is able to get a copy of that secrets.json below, your 2FA approach is toast...

# !!!USE AT YOUR OWN RISK!!!

## Linux Demo

```bash
$ cat secrets.json
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

$ cat secrets.json | cargo run paypal
Use 386656 to access paypal

$ cat secrets.json | cargo run google alice@gmail.com
Use 765345 to access google/alice@gmail.com

$ cat secrets.json | cargo run microsoft liveid alice@gmail.com
Use 564534 to access microsoft/liveid/alice@gmail.com
```

## Windows Demo

```msdos
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

> type secrets.json | cargo run paypal
Copied code for paypal to clipboard

>type secrets.json | cargo run google alice@gmail.com
Copied code for google/alice@gmail.com to clipboard

>type secrets.json | cargo run microsoft liveid alice@gmail.com
Copied code for microsoft/liveid/alice@gmail.com to clipboard
```
