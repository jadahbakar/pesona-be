[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
[![Contributor][contributor-shield]][contributor-url]
[![PostgreSQL][postgresql-shield]][postgresql-url]
<!-- PROJECT LOGO -->
<br />
<p align="center">
   <img src="./assets/images/pesona.png" alt="Logo" width="220" height="200">
  <h2 align="center">Pesona - DB </h2> <br />
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

This Database for DAPEN.


<!-- BUILD WITH -->
#### Built With

* [PostgreSQL](https://www.postgresql.org/)
* [goose](https://github.com/pressly/goose/)


<!-- REFERENCES -->
#### References
* [ChatGPT](https://chat.deepseek.com/a/chat/s/3d7b2da9-50e6-4650-ac39-6d9f78604ba2)



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
* And then you need to install [goose](https://github.com/pressly/goose/)
    ```sh
    ❯ brew install goose
    ❯ goose --version
    goose version: v3.22.0
   ```


<!-- INSTALLATION -->
## Installation

#### Create the SQL command
1. Add configuration on ~/.zshrc
  ```sh
  ❯ cat ~/.zshrc  | grep -E '^.*GOOSE.*$'
    export GOOSE_DRIVER=postgres
    export GOOSE_DBSTRING="host=192.168.18.51 port=5432 user=checklist password=checklist*164 dbname=checklist sslmode=disable" 
   ```
2. Creating sql command
  ```sh
  ❯ goose dir "./migrations" -s create test sql
  ```

#### Running migration
1. run the migration
  ```sh
  ❯ cd ${PWD}/data/devops/database/checklist-db/migrations
  ❯ goose up
  ``` 
2. drop all migration
  ```sh
  ❯ goose down-to 0
   ```

<!-- Project structure -->
## Project Structure

```sh
.
├── CHANGELOG.md
├── kamus.md
├── makefile
├── migrations
│   ├── 20250000000010_schemas.sql
│   ├── 20250000000020_sec_params.sql
│   ├── 20250000000021_utl_generate_id.sql
│   ├── 20250000000022_log_users.sql
│   ├── 20250000000126_mst_company.sql
│   ├── 20250000000130_sec_role.sql
│   ├── 20250000000131_sec_permission.sql
│   ├── 20250000000135_sec_user.sql
│   ├── 20250000000144_sec_user_role.sql
│   └── 20250000000147_sec_role_permission.sql
├── README.md
├── script_backup
└── script_restore
```


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[company-website]:http://sd.com/
[contributor-url]: https://github.com/shandysiswandi
[license-url]: ./LICENSE
[linkedin-url]: https://linkedin.com/in/dedystyawan
[rust-url]: https://www.rust-lang.org/
[postgresql-url]: https://www.postgresql.org/
[contributor-shield]: https://img.shields.io/badge/Contributors-1-orange.svg?style=for-the-badge
[license-shield]: https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge&logo=opensourceinitiative&logoColor=white
[linkedin-shield]: https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white
[rust-shield]: https://img.shields.io/badge/rustc-1.86.0-blue.svg?style=for-the-badge&logo=rust
[postgresql-shield]: https://img.shields.io/badge/PostgreSQL-17-blue.svg?style=for-the-badge&logo=postgresql&logoColor=white
[person-url]: https://img.icons8.com/?size=100&id=83190&format=png&color=000000
[Prometheus]: https://prometheus.io/
[Traefik]: https://traefik.io/
[Rust]: https://www.rust-lang.org/tools/install
[Goose]: https://github.com/pressly/goose/