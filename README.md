# di-srv (div.is server/backend)

# Let

## components
1. API written using actix-web (Rust)
2. DB interface using sqlx (Rust)
3. Using podman for container and pod orchestration where possible
4. ansible for deployment automation

## technologies to try
1. go API or other helper packages
2. scala / clojure / phoenix / scala / kotline / other established frameworks as an alternatives to save cost on development time
3. Current stack idea:
    - Back:
        - actix-web
        - sqlx / tokio/actix-postgres
        - main db/api on digitalocean
        - lambda / dynamodb on aws

## tasks for down the road
- [x] Make the models be in teh db crate, not in com crate
- [ ] Make API run command take DB as input, so all exec is handled in root crate

## ways to run (eventually)
1. run application on bare metal, or run it serverless (generally), or on AWS Lambda (specifically), while sacrificing as little functionality as possible
