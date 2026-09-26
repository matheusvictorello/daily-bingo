# Daily Bingo

## Public routes

### Auth

- /signup
    - POST { email, password } -> { token }

- /login
    - POST { email, password } -> { token }

## Protected routes

### Bingo

- /bingos
    - POST { cols, rows, values } -> { id }

- /bingos/{bingo_id}
    - GET -> { cols, rows, values }
    - PUT { cols?, rows?, values? } -> { cols, rows, values }
    - DELETE
