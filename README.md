
# Password Manager

A secure, local password manager built using [Tauri](https://tauri.app/) and Rust, designed to run on macOS. This app securely stores passwords locally, with AES-256-GCM encryption, and uses the Argon2 hashing algorithm for safe master password management.

## Features

- **Secure Storage**: Passwords are encrypted using AES-256-GCM and stored locally in a secure SQLite database.
- **Master Password Protection**: The master password is hashed with Argon2, ensuring strong protection against brute-force attacks.
- **Clipboard Management**: Copies passwords to the clipboard and automatically clears the clipboard after a set time for added security.
- **Local-Only Data**: All password data is stored locally on the user’s machine.
- **Multiple User Accounts**: Supports storing passwords for multiple accounts, each associated with a unique identifier.

## Prerequisites

- **Rust**: Install [Rust](https://www.rust-lang.org/) and `cargo` (Rust’s package manager) for building the Tauri backend.
- **Node.js**: Required to build the Tauri frontend.
- **Tauri**: Install Tauri dependencies for your platform by following the [Tauri setup guide](https://tauri.app/v1/guides/getting-started/prerequisites).

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/yourusername/your-repository-name.git
   cd your-repository-name
   ```

2. Install dependencies:

   ```bash
   # Install Node.js dependencies
   npm install

   # Build the Tauri project with Rust and Node.js
   npm run tauri build
   ```

3. Run the application:

   ```bash
   npm run tauri dev
   ```

## Usage

1. **Setting Up the Master Password**: On the first run, the app prompts you to set a master password. This password is used to derive the encryption key for stored passwords.
2. **Adding Accounts and Passwords**: Use the app interface to add accounts and their associated passwords. Each password is encrypted and saved locally.
3. **Retrieving Passwords**: Decrypt stored passwords by entering your master password.
4. **Clipboard Security**: When a password is copied, it will automatically be cleared from the clipboard after 10 seconds.

## Security Considerations

- **Local Database Storage**: All data is stored locally in `~/Library/Application Support/PasswordManager/database.db`. Ensure that you **do not upload this file to GitHub**.
- **Encryption and Hashing**: Passwords are securely stored using AES-256-GCM encryption and Argon2 hashing for strong protection against unauthorized access.
- **Error Handling**: Error messages are intentionally general and avoid exposing sensitive information.

## Contributing

1. Fork this repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) for lightweight and secure desktop applications.
- Uses [Rust’s](https://www.rust-lang.org/) powerful memory safety features and cryptographic libraries.
