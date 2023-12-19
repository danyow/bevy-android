# bevy-android

# Support Platforms
- Library (Usable in other rust proyects)
- Mobile
  - Android
  - iOS (⚠️ Soon)

# Support Stores- GithubIO (only for web)# Requirements
- Rust
- Cargo
- [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
- [Cargo Make](https://github.com/sagiegurari/cargo-make) (Optional)
- [Cargo Release](https://github.com/crate-ci/cargo-release) (Optional)
- [Trunk](https://trunkrs.dev) (Optional for web development)

# Configure Github Actions
> [!WARNING]
> After initializing this project and having activated the github workflows, you need to configure the secret variables in your github project (This is done this way to protect the security of your data).

- CI: **Not required**
- CD/Deployment:Here is the list of each variable required for workflows, note that if you have not activated the functionality, you do not need to configure it
    - for Android:
        - KEYSTORE_PASSWORD
            - For this key I recommend to generate a random one (and optionally keep it in a safe place just in case, although theoretically it would not be necessary since you have this autodeployment).
            - Save it temporarily for use in the compilation key generation (KEYSTORE)
        - KEYSTORE
            - For this you have two options [Video Help](https://www.youtube.com/watch?v=ipS7SbyR5Yw&ab_channel=FlutterCoding)
                - [Using Android Studio](https://developer.android.com/studio/publish/app-signing?hl=es-419#generate-key)
                - Using Command Line
                    - `keytool -genkey -v -keystore <key_name>.jks -keyalg RSA -keysize 2048 -validity 36525 -alias <key_alias>`
                    - Fill in your data and you are ready to go
            - Now to put this key in the github secret variable, just run this command `cat <key_name.jks> | base64`
            - The output of the above command is what you should put in this secret
                > [!WARNING]
                > It is known that this is not the most secure way to store the key and although the secrets cannot be seen once created, I will work to implement a more secure way.

# Development Guide
> [!NOTE]
> If you want generate all needed for icons, you can use [this page](https://icon.kitchen)
- Edit the `.env` file if you need
- Edit `src` folder
- Run `cargo make dev` for run as development mode
- Run `cargo make --list-all-steps` for check all aviable tasks
- To upload a new version and trigger all the workflows related to the deployment of a new version, you just have to run the command `cargo release -x patch` (See the `cargo release -h` for more information)

## Other CargoMake Tasks
* **check** - Check all issues, format and code quality
* **clean** - Clean all target directory
* **clippy** - Check code quality
* **default** - Check all issues, format and code quality
* **dev** - Run native launcher with development configuration
* **fix-all** - Try fix all clippy and format issues
* **fix-fmt** - Fix format
* **fmt** - Check format quality
* **test** - Check all unit test
