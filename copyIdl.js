const fs = require('fs');
const idl = require('./target/idl/counter_solana.json');

fs.writeFileSync('./app/src/idl.json', JSON.stringify(idl));
