const express = require('express');
const request = require('request');

const app = express();
app.use(express.json());

app.get('/version',  (req, res) => {
  res.send("shop 0.0.1");
});

app.get('/items',  (req, res) => {
  request('http://host.docker.internal:3030/items/', (error, response, body) => {
    res.set('Content-Type', 'application/json');
    res.send(body);
  });
});

app.post('/login', (req, res) => {
  const name = req.body.name;
  const password = req.body.password;
  const options = {
    uri: "http://host.docker.internal:3010/login.php",
    headers: {
      "Content-type": "application/x-www-form-urlencoded",
    },
    form: {
      name,
      password
    }
  };
  request.post(options, (error, response, body) => {
    res.set('Content-Type', 'application/json');
    res.send(body);
  });
});

app.listen(3000, function () {
});
