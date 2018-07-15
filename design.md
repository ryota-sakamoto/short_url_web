# architecture
- frontend: React | Vue
- backend: actix_web
- db: MySQL

# function
- title: authentication
  - f: login
  - f: logout
- title: short url
  - f: add
    - set expire
    - basic(auth)
  - f: remove(auth)

# endpoint v1