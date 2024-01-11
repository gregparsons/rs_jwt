## generate JWT (specifically for coinbase)

```./run.sh```

## .env:
```
RUST_LOG=info
JWT_SECRET="secret"
JWT_KEY_NAME="organizations/..."
```

## convert.sh

First, the Coinbase-generated private key needs to be converted to a format the jsonwebtoken crate can read. To convert it put what coinbase generates 
("-----BEGIN EC PRIVATE KEY-----...") in ec1.pem. The result is a pem starting with "-----BEGIN PRIVATE KEY-----".
```openssl pkcs8 -topk8 -nocrypt -in ec1.pem -out ec2.pem```

## run.sh
```
export JWT=$(cargo run)
echo $JWT
curl -H "Authorization: Bearer $JWT" 'https://api.coinbase.com/api/v3/brokerage/accounts'
```

## references

- https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-auth
- https://jwt.io/
- https://8gwifi.org/jwsgen.jsp
- https://github.com/Keats/jsonwebtoken/pull/359

