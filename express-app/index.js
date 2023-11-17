const express = require('express');
const app = express();

app.use(express.static('public'));

app.get('/', function(req, res) {
    let result = 'Hello World';
    result += '<br>';
    result += '<h1 id="result"></h1>';
    result += '<script type="module" src="/script.js"></script>';
    res.send(result);
})

app.listen(3000);
