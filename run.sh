export JWT=$(cargo run)
echo $JWT
curl -H "Authorization: Bearer $JWT" 'https://api.coinbase.com/api/v3/brokerage/accounts'
