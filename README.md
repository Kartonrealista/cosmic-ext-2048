# WIP

# 2048 for Cosmic™ Desktop

A 2048 game written in libcosmic and Rust.

## Development

When you open the repository in your code editor, you will see a lot of comments in the code. These comments are there to help you get a basic understanding of what each part of the code does.

Once you feel comfortable with it, refer back to the [COSMIC documentation](https://pop-os.github.io/libcosmic/cosmic/) for more information on how to build COSMIC applications.

## Install

To install your COSMIC application, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your application:

```sh
just build-release
sudo just install
```
