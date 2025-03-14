<br>

<p align="center">
  <img src="./assets/s3n-dev.png" height="100" alt="0xzero.org" />
</p>
<br>

<p align="center">
   <a href="https://github.com/0xZeroLabs/s3n/network/members"><img src="https://img.shields.io/github/forks/0xZeroLabs/s3n?style=for-the-badge&color=a8c7ff&labelColor=1a1b1f"></a>
   <img src="https://img.shields.io/github/stars/0xZeroLabs/s3n?style=for-the-badge&logo=github&color=e6c419&labelColor=1d1b16">
   <a href="https://x.com/0xZeroOrg"><img src="https://img.shields.io/twitter/follow/0xZeroLabs.svg?style=for-the-badge&logo=x&color=e6c419&labelColor=1d1b16"></a>
   <br>
   <!-- <img src="https://img.shields.io/github/languages/count/0xZeroLabs/s3n?style=for-the-badge&color=ffb4a2&labelColor=201a19"> -->
   <a href="https://github.com/0xZeroLabs/s3n/issues"><img src="https://img.shields.io/github/issues/0xZeroLabs/s3n?style=for-the-badge&color=ffb4a2&labelColor=201a19"></a>
   <a href="https://github.com/0xZeroLabs/s3n/pulls"><img src="https://img.shields.io/github/issues-pr-raw/0xZeroLabs/s3n?style=for-the-badge&color=ffb4a2&labelColor=201a19"></a>
   <a href="https://github.com/0xZeroLabs/s3n/graphs/contributors"><img src="https://img.shields.io/github/contributors-anon/0xZeroLabs/s3n?style=for-the-badge&color=ffb4a2&labelColor=201a19"></a>
   <!-- <img src="https://img.shields.io/github/languages/code-size/0xZeroLabs/s3n?style=for-the-badge&color=ffb4a2&labelColor=201a19"> -->
<br>
  <a href="https://docs.0xzero.org"><img src="https://img.shields.io/badge/docs-%F0%9F%93%84-blue?style=for-the-badge&color=ffb4a2&labelColor=201a19"></a>
  <a href="https://github.com/0xZeroLabs/s3n/blob/master/LICENSE"><img src="https://img.shields.io/github/license/0xZeroLabs/s3n?style=for-the-badge&color=ffb4a2&labelColor=201a19"></a>
</p>

# S3N 🟂

S3N is a TEE as a Service network.

# 🎯 Grand Plan

We're currently focused on achieving the following to get a working version of S3N:

- [ ] Design base architecture

# 📁 Project Structure

<pre>
├── <a href="./app/">app</a>: The main application
│ ├── <a href="./app/sgx">sgx</a>: Configurations for the enclave
│ │ ├── <a href="./app/sgx/config.xml">config.xml</a>: Developer defined parameters of the enclave
│ │ ├── <a href="./app/sgx/enclave.edl">enclave.edl</a>: Enclave Definition Language file defining the enclave interface
│ │ ├── <a href="./app/sgx/enclave.lds">enclave.lds</a>: Linker script for the enclave
│ │ └── <a href="./app/sgx/private.pem">private.pem</a>: Developer key used to sign the enclave, do not use this key to sign your enclave in production, please use your own key
│ ├── <a href="./app/src/main.rs">src/main.rs</a>: Main entrypoint for the application
│ └── <a href="./app/build.rs">build.rs</a>: Builder code used to build the application, you don't need change it
├── <a href="./enclave/">enclave</a>: The SGX enclave implementation
│ └── <a href="./enclave/src/lib.rs">lib.rs</a>: Main library file for the enclave implementation
├── <a href="./mock-lib/">mock-lib</a>: A mock library which is called by the enclave via OCALL
│ └── <a href="./mock-lib/src/lib.rs">lib.rs</a>: Main library file for the mock library implementation
</pre>
