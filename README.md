[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
[![Contributor][contributor-shield]][contributor-url]
[![Rustc][rust-shield]][rust-url]
[![PostgreSQL][postgresql-shield]][postgresql-url]
 


<!-- PROJECT LOGO -->
<br />
<p align="center">
   <img src="./assets/images/pesona.png" alt="Logo" width="220" height="200">
  <h2 align="center">Pesona - BE </h2> <br />
</p>


<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
        <li><a href="#references">References</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#project-structure">Project structure</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->
## About The Project

This Database for Pesona.

<!-- BUILD WITH -->
#### Built With
* [Rust][rust-url]
* [PostgreSQL][postgresql-url]


<!-- REFERENCES -->
#### References
* [ChatGPT](references-1)

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.


<!-- PREREQUISITES -->
#### Prerequisites
* I use macOS Sonoma 14.6.1 (23G93)
  ```sh
  ❯ uname -a
  Darwin Kernel Version 23.6.0
   ```

<!-- INSTALLATION -->
## Installation

#### Install Depencency
  ```sh
    ❯ cargo install cargo-watch
    ❯ cargo add tokio --features full
    ❯ cargo add tokio -F tracing
    ❯ cargo add futures
    ❯ cargo add chrono
    ❯ cargo add log env_logger
    ❯ cargo add axum --features full
    ❯ cargo add serde -F derive
    ❯ cargo add serde_json
    ❯ cargo add sqlx --features runtime-tokio,chrono,postgres
    ❯ cargo install cargo-watch
  ```


<!-- FEATURES -->
## Features
- [x] Create README.md
- [x] Create makefile
- [x] Create Health
- [ ] Create Auth
- [ ] Create Makefile
  - [ ] Include external environment file (_**app.env**_)
  - [ ] All In One service (_**up**_ - _**clean**_)
  - [ ] Development (_**build**_ - _**run**_ - _**watch**_)
  - [ ] Development with Docker (_**devel-up**_ - _**devel-stop**_ - _**devel-down**_)
- [ ] Create Authentication Service
  - [ ] Create Security Service
- [ ] Create Authorization Service
  - [ ] Create Access Control List for User

<!-- Project structure -->
## Project Structure

```sh
.
├── assets
│   └── images
│       └── pesona.png
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE
├── makefile
├── README.md
└── src
    └── main.rs
```
<!-- LICENSE -->
## License
Distributed under the Proprietary License. See `LICENSE` for more information

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[company-website]:http://sd.com/
[references-1]:https://chat.deepseek.com/a/chat/s/3d7b2da9-50e6-4650-ac39-6d9f78604ba2
[contributor-url]: https://github.com/shandysiswandi
[license-url]: ./LICENSE
[linkedin-url]: https://linkedin.com/in/dedystyawan
[rust-url]: https://www.rust-lang.org/
[postgresql-url]: https://www.postgresql.org/
[contributor-shield]: https://img.shields.io/badge/Contributors-1-orange.svg?style=for-the-badge
[license-shield]: https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge&logo=opensourceinitiative&logoColor=white
[linkedin-shield]: https://custom-icon-badges.demolab.com/badge/LinkedIn-0A66C2?style=for-the-badge&logo=linkedin-white&logoColor=fff
[rust-shield]: https://img.shields.io/badge/rustc-1.86.0-blue.svg?style=for-the-badge&logo=rust
[postgresql-shield]: https://img.shields.io/badge/PostgreSQL-17-blue.svg?style=for-the-badge&logo=postgresql&logoColor=white

