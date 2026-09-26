# Daily Bingo

## Public routes

### Auth

- /signup
    - POST { email, password } -> { token }

- /login
    - POST { email, password } -> { token }

## Protected routes

### Bingo

- /bingo
    - POST {} -> { id }

- /bingo/{bingo_id}
    - GET -> {}
    - PUT {} -> {}
    - DELETE
